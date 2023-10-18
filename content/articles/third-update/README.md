---
layout: post
title:  Technical bookshelf
author:
  - Mark
  - exklamationmark
date: 2016-04-06
tags:
  - sticky
  - resources
  - books
  - papers
blurb: |
  Below are a list of book and papers that I found helpful for technical work

---

Below are a list of *books/papers that I found helpful* for `programming/technical` work:

# h1

## h2

### h3

#### h4

##### h5

###### h6


foo barj asd;fkjasdlfjaslkjd




#### The Go programming language

![Book cover](https://images.gr-assets.com/books/1426831830l/25080953.jpg)

This is like of the "K&R book" for Go and is a useful `reference` book for the language.

It has section explain some of the language choice, which helped me appreciate the complexity of
language design. It's also a good reminder of why I need to type so much :)

```go
package render

import (
	"fmt"
	"io"
	"io/fs"
	"os"
	"os/exec"
	"path"
	"path/filepath"
	"strings"
	"time"

	"github.com/bitsgofer/notebook-go/internal/fileutil"
	klog "k8s.io/klog/v2"
)

func RenderSinglePage(dstPath, srcPath string, themeFS fs.FS, templatePath string, luaFilterPaths ...string) error {
	// copy theme files to tmp dir.
	// NOTE: not too efficient, since we copy once per function call.
	// => try to call render() once per template/type of content,
	// rather than once per content file.
	// -------------------------------------------------------------------------
	// copy template
	templateTmpDir, tmpTemplatePaths, err := copyToTempDir(themeFS, templatePath)
	if err != nil {
		return fmt.Errorf("cannot copy template file to temp dir; err= %w", err)
	}
	defer os.RemoveAll(templateTmpDir)
	tmpTemplatePath := tmpTemplatePaths[0]
	// -------------------------------------------------------------------------
	// copy lua filters
	luaTmpDir, tmpLuaFilterPaths, err := copyToTempDir(themeFS, luaFilterPaths...)
	if err != nil {
		return fmt.Errorf("cannot copy lua filters to temp dir; err= %w", err)
	}
	defer os.RemoveAll(luaTmpDir)
	var luaFilters []string
	for _, f := range tmpLuaFilterPaths {
		luaFilters = append(luaFilters, "--lua-filter", f)
	}
	// -------------------------------------------------------------------------

	// get metadata
	const timeFormat = "2006-January-02"
	stat, err := os.Lstat(srcPath)
	if err != nil {
		return fmt.Errorf("cannot get stat of %s; err= %w", srcPath, err)
	}
	lastModifiedDate := stat.ModTime().Format(timeFormat)
	currentDate := time.Now().Format(timeFormat)
	dateMetadata := []string{
		"--metadata", fmt.Sprintf("current-date=%s", currentDate),
		"--metadata", fmt.Sprintf("last-modified-date=%s", lastModifiedDate),
	}

	// run pandoc command
	// -------------------------------------------------------------------------
	// args
	args := []string{
		"--standalone",
		"--template", tmpTemplatePath,
		"--no-highlight",
	}
	args = append(args, dateMetadata...)
	args = append(args, luaFilters...)
	args = append(args, fmt.Sprintf("--output=%s", dstPath))
	args = append(args, srcPath)
	klog.V(4).InfoS("pandoc", "args", args)
	// -------------------------------------------------------------------------
	cmd := exec.Command("pandoc", args...)
	// redirect command stdout -> dstPath
	html, err := os.Create(dstPath)
	if err != nil {
		return fmt.Errorf("cannot create html file; err= %w", err)
	}
	cmd.Stdout = html
	// access command stderr
	stderr, err := cmd.StderrPipe()
	if err != nil {
		return fmt.Errorf("cannot get stderr of pandoc command; err= %w", err)
	}
	// run command
	if err := cmd.Start(); err != nil {
		return fmt.Errorf("cannot start pandoc command; err= %w", err)
	}
	// read stderr
	pandocStderr, _ := io.ReadAll(stderr)
	// wait til command finish, then close stdout, stderr
	if err := cmd.Wait(); err != nil {
		return fmt.Errorf("cannot wait for pandoc cmd to finish; stderr= %s; err= %w", pandocStderr, err)
	}

	klog.V(3).InfoS("rendered with pandoc", "src", srcPath, "dst", dstPath, "template", templatePath, "lua-filters", luaFilterPaths)
	return nil
}

func RenderMultiplePages(dstDir, srcDir string, themeFS fs.FS, templatePath string, luaFilterPaths ...string) error {
	srcFS := os.DirFS(srcDir)
	renderErr := fs.WalkDir(srcFS, ".", func(filePath string, d fs.DirEntry, pathErr error) error {

		// exit if we see a path error
		if pathErr != nil {
			return pathErr
		}
		// skip non-README.md files
		if isContent := strings.HasSuffix(filePath, "README.md"); !isContent {
			return nil
		}

		dirName := path.Base(path.Dir(filePath))
		dst := filepath.Join(dstDir, dirName, "index.html")
		src := filepath.Join(srcDir, filePath)
		// ensure dstDir
		if err := fileutil.EnsureDir(path.Dir(dst)); err != nil {
			return fmt.Errorf("cannot create directory for rendered file; err= %w", err)
		}
		// render
		if err := RenderSinglePage(
			dst,
			src,
			themeFS,
			templatePath,
			luaFilterPaths...,
		); err != nil {
			return fmt.Errorf("cannot render src= %s to dst= %s; err= %w", src, dst, err)
		}

		return nil
	})
	if renderErr != nil {
		return fmt.Errorf("cannot render all content; err= %w", renderErr)
	}

	klog.V(3).InfoS("rendered with pandoc", "srcDir", srcDir, "dstDir", dstDir, "template", templatePath, "lua-filters", luaFilterPaths)
	return nil
}
```

#### Time, Clocks and the Ordering of Events in a Distributed System

<https://amturing.acm.org/p558-lamport.pdf>

Classic paper that should be referred to when you are questioning about order of events.

It's more or less a must-read when it comes to distributed systems, as the paper clarifies
the concept of `wall clock`, `partial order`, `concurrency`, etc...

#### Design Data-Intensive Applications

![Book cover](https://images.gr-assets.com/books/1415816873l/23463279.jpg)

A map to the world of databases, message queues and data pipelines.

It presents a high-level overview of data storage and processing systems,
prolems they face at scale as well as approaches to mitigate them.

Since the book touched on concepts of distributed systems & system performance, it's also
a good read if you are into those.

#### Computer Systems: A Programmer's Perspective

![Book cover](https://images.gr-assets.com/books/1387708094l/829182.jpg)

It's a bottom-up book that goes from bits -> program.
This is useful to review how computer actually works,
which will be important when doing scientific computation, debugging or
understanding performance bottlenecks.

It's also a CMU textbook, thus making it even more legitimate.

#### Fooled by randomness

![Book cover](https://images.gr-assets.com/books/1388180506l/38315.jpg)

Not programming-related, but I feel it's a good read.
I walked away with many concepts on sampling, random distribution, ...
and learned not to try Russian roullete, even if you give me 1-billion dollar to do it!

#### Practical Vim: Edit Text at the Speed of Thought

![Book cover](https://images.gr-assets.com/books/1336278962l/13607232.jpg)

Quick intro to Vim, which is my bread and butter now :)

## tmux: Productive mouse-free development

![Book cover](https://images.gr-assets.com/books/1330628877l/13506825.jpg)

Quick intro to Vim, which is my bread and butter now :)


#### Reddit: cscareerquestions

<https://www.reddit.com/r/cscareerquestions>

Useful read when it comes to interviews, salary negotiation and other workplace problems.

I found a lot anecdotal story of various companies there.


<hr/>
<hr/>
<hr/>

##### backlog

```md
- The C++ programming language - 4th edition
- Systems Performance: Enterprise and the Cloud
- The C programming language
- Compilers: Principles, Techniques, and Tools (the Dragon Book)
- The Garbage Collection Handbook: The Art of Automatic Memory Management
- Jepsen serires by Aphyr
- [Destroy all software](https://www.destroyallsoftware.com/screencasts)
- The art of computer programming
- The design of everyday things
- The art of UNIX programming
- Seven databases in seven days
- Feyman lectures on computation
- RESTful web services
- Google SRE book
```
