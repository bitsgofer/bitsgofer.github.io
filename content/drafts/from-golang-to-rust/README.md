---
layout: article
title:  From Golang to Rust
author:
  - exklamationmark
date: 2023-10-19
tags:
  - programming-languages
  - golang
  - rust
blurb: |
  First impressions of Rust, from someone who work a lot with Golang.

---

I have wanted to learn Rust for a while. It seems trendy in my technical sphere
(computing systems, database, high-performance server, etc). However,
it has been hard to find time to learn after my son was born.
Luckily, I managed to get some time after he went to daycare
and paying back my sleep debt (yay!).

This post is written with two main perspective:

- Go vs Rust, because I read/write lots of Go at work.
- Rust's "ergonomics", i.e: how easy is it to do certain programming tasks.

## Overall impression

In general, Rust seems like a modern language with features I liked from my
time with Go, C++, Ruby, etc. However, the learning curve is definitely stiffer.

For example, it took me quite a bit more time to write a multi-threaded
web server. In general, I ended up look into many things that Go provides
out of the box: goroutines, async IO, etc.

Still, I would say Rust is a good language to learn. Especially to help me
write system-level software. I also secretly hope that the language might be
useable when I attempt to make games as well :D

## The "learning" project

To learn Rust in a semi-usable setup, I attempted to rewrite my blogging
tool entirely in Rust. This allows me to explore these parts of the language:

- Creating cross-platform CLI programs.
- Use the Rust standard library.
- Writing a purpose-built web server (for HTML and Godot web games).
- Explore how to express various things in Rust, e.g: writing polymorphic
  code, use generics, error handling, writing test, etc.

To speed up the feedback loop. I also used GPT-4 to provide some code samples,
instead of trying to read the [The Rust Programming Language](#) from cover
to cover.

## My experiences

#### Immutable variables is nice


Having variables being immutable by default is nice, especially when they are
passed around a lot.

I also like the fact variable declaration (mostly) tell you whether they are
on the stack (default syntax) or the heap (wrapped in `Box`).
This just make it easier with simulating code in your head.

#### Option, Result and pattern matching

These are just great. I like them back when I tried Haskell but it was rare to
find them outside of functional the world.

Essentially, when combing these with a `match` clause, it's easy to let the
compiler checks that you have handled all the cases:

```rust
let x: Option<i64> = Some(42);

match x {
    Some(val) => {
        println!("{:#?}", val);
    },
    // None => {
    //     println!("None");
    // }
}
```

The code above leads to this compile-time error:

```
error[E0004]: non-exhaustive patterns: `None` not covered
 --> src/main.rs:6:11
  |
6 |     match x {
  |           ^ pattern `None` not covered
  |
note: `Option<i64>` defined here
 --> /rustc/cc66ad468955717ab92600c770da8c1601a4ff33/library/core/src/option.rs:563:1
 ::: /rustc/cc66ad468955717ab92600c770da8c1601a4ff33/library/core/src/option.rs:567:5
  |
  = note: not covered
  = note: the matched value is of type `Option<i64>`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
  |
9 ~         },
10~         None => todo!(),
  |

For more information about this error, try `rustc --explain E0004`.
```

#### Redeclaring local variables

This type of shadowing tends to lead to be consequences in most languages.
However, I do fine cases code like these became useful:

```rust
let input = get_param_as_string("input");
let input = std::fs::read(input).unwrap();
```

Similar code in Go wouldn't work:

```go
input := get_param_as_string("input")
input, _ := os.ReadFile(input)
```

From my experiences, code that tends to linearly "process data" (i.e: take
the string input file name, the read the input from it) gains a lot from this.

Descriptive, short variable names are hard to find and this stops us from
adding suffixes to them just to satisfy the compiler. Plus, in cases like these,
we rarely reuse the earlier variable after the new one is declared => less
chance for conflict.

In particular, this helps a lot with Rust since there's lots of "wrapper" types
like `Option`, `Result`, etc.

#### Generics

At a glance, the generics syntax feels pretty similar to my C++ days,
whereas Go's generics syntax is not as unfamiliar (but still serviceable).

The great thing here is that compiler error messages on generic are **A LOT
easier** to understand (looking at you, C++).
