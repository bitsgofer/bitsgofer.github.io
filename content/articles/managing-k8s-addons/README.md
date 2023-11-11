---
layout: article
title:  Managing Kubernetes addons
author:
  - exklamationmark
date: 2023-11-10
tags:
  - kubernetes
  - devops
blurb: |
  How to manage addons software in Kuberentes more autonomously

---

My team manages a bunch of Kubernetes clusters at work. Each cluster have a
high amount of "addons" - software that we install to provide more features
on top of the basic Kubernetes. To name a few:

- **Prometheus**: to collect system-level metrics.
- **nginx & HAProxy ingress controller**: Enable reverse-proxy, L7 load-balancing.
- **node-local-dns-cache**: Speed up DNS querying time.
- etc

Teams running workloads in our cluster rely on these addons and expect them
to always be running at the latest versions. Unfortunately, this created
a lot of manual work for us.

To outsiders, the process should be quite straight-forward: you install addons
X, Y and Z into clusteres A, B, C. Then whenever there is new versions of X,
Y or Z, you update them in every clusters. What's so hard about it?

It took me a while to be able to articulate the challenges on the ground.
In this article, let's examime why it is hard to manage addons
in multiple k8s clusters.

> "Addons" are defined as a collection of k8s resources
> (e.g: Deployment, Service, Ingress, PDB, etc) that ultimately deploy and
> piece of software like Prometheus, nginx-ingress-controller, etc.

### Deploying a new addons

Naively, we would think that installing a new addons is simple.
After all, you just apply the same YAML files to every clusters, right?

Unfortunately, this is only true if every cluster you manage is the same.

What happens if you manage a pre-prod cluster with 20 nodes and
a huge production cluster of 1000 nodes? Certain components will need to be
scaled accordingly.

For example, if I use Prometheus to collect system metrics, I will need a lot
more `memory.requests` for the large cluster. Infact, we had so much metrics
that a mere 2h windows required >400 GB of memory, more than the RAM in a single
baremetal server. This required us to have an entirely different architecture
for our prod Prometheus (multiple shards that are aggregated).

This shows that if you have N different classifiications of clusters,
you might need up to N ways to install a new addons.

Some dimensions that will quickly inflate this N number are:

- Cluster size: e.g: small: [1, 100) nodes, medium [101, 1000] nodes, large [1001, +Inf).
- Usage tiers of cluster: e.g: QA, internal-workload, production-workload, etc.
- Cluster location: NorthAmerica, Europe, Asia, etc.
- Cluster providers: e.g: your own DC, GCP, AWS, Azure, DO, etc.

### Upgrading an existing addons

Okay, so installation is hard, but updates should be a lot simple, right?
Afterall, we just need to bump some kind of version for the container image
or Helm chart.

Maybe! In many case, simply bumping the versions will work.
However, complexities will arise when you need to deal with dependencies.

For example, the new `X@v1.1.0` require a feature that only exist in `k8s 1.25`,
but some clusters are still at `k8s 1.24`.
Thus, blindly updating might break your addons for good.

Or when upgrading to a new version also requires additional work that aren't
captured in the image or Helm chart. I could recall an instance when we
wanted to upgrade to some a `nginx-ingress controller`, which dropped support
for `networking.k8s.io/v1beta1 - Ingress` object. We had the foresight to chase
after every team in our clusters to update their `Ingress` object. Without that
work, simply upgrading would break a lot of workloads.

### Deleting addons that are no longer used

This is often something that we don't often think about, but might come back
to bite us. Let's use an actual example here, which is an issue I ran into:

- Initially, I have the addons "node-local-dns-cache" running as a DaemonSet
  on every node.
- Using the magic of iptables, it intercepts node-local DNS queries
  and reduces the load on `coredns` DNS servers.
- This also allows DNS resolution on the node to be more resilient:
  if the `node-local-dns-cache` pod cannot run, DNS queries will go to `coredns`
  and won't cause problems for workloads.
- However, when I upgrade `kube-proxy` to use `IPVS` for Service routing
  (due to performance), DNS queries can only flow to `node-local-dns-cache`
  and never to `coredns` again.
- This requires me to split the original `node-local-dns-cache` DaemonSet
  into **two** new DaemonSets, so when I need to bump its version, I can keep
  one DaemonSet running while upgrading another. Otherwise, DNS resolution
  will be down for the node.
- Unknown to me, there is a port clash between the original DaemonSet and
  the two new DaemonSets, such that I would have to delete the original
  addons first before being able to deploy the new ones.
- When I naively rolled out the changes, but forgot to delete the original
  DaemonSet, DNS resolution started failing left and right.

In this case, I effectively have an old `single-instance node-local-dns-cache`
addons and a new `HA node-local-dns-cache` addons.
I should have deleted the old addons first, which would allow me to deploy
the new one.

## Summary

Hopefuly the examples above painted a more realistic picture about managing
k8s addons across multiple clusters.

As you can see, the complexities arise from:

- Having to customize the high-level addons into multiple concrete addons
  for many cluster combination
  (e.g: `size` x `usage-tier` x `location` x `providers`, etc).
- Having to deal with depedencies like k8s version, plus other migration works
  that can't be easily captured.
- Forgetting to clean up addons that are no longer used (and differentiating
  when you are replacing addons vs upgrading).

## Next steps

I don't have a good solution to all of this management problem yet.

As a team, we are trying to look into automation, but my hunch is that we will
only be able to tackle a few scenarios. For example, I believe that anything
involving extra migration work will be hard to automate.

Still, maybe solving 80% of the problem will already be a good thing on its own.
