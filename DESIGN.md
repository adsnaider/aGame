# Design Document

[TOC]

The goal of this document is to explain the design behind aGame, a modular,
multithreaded game engine.

## Goals

* The game engine has to be fast.
* The game engine should be modular.
  * Replacing a module for another shouldn't interfere with another module.
* Multithreading is a main goal for the game engine. 
  * This goes hand in hand with the original goal of the game engine being fast.

## Why Rust?

One of the quirks about this project is that it's built using Rust. This is
relatively unusual for a game engine as Rust is a relatively new programming
language. There's multiple reasons why Rust is used for the development of this
game engine:

### Rust is a very fast, compiled language

This is really important since a game engine, no matter how well the
architecture is developed, has to be fast to process every frame. This pretty
much limits the options for a game engine to be written in either C, C++, and Rust.

### Rust emphasizes code and type safety

Between C, C++, and Rust, we chose Rust because of its emphasis in type safety
and multithreaded programming.

## What are modules?

In the scope of aGame, a module is a piece of code that may run in a separate
thread tries to solve a particular aspect for the game. Examples of modules are:

* Rendering module
* Sound system module
* Game loop module
* Input module

There's no limits to what a module could be.

### How do they work?

A module, as mentioned previously, will run in its own thread. Each module will
be able to broadcast information out to other modules. Modules may choose to
receive data from another module by requesting a channel to it. When a module
requests a channel, it will be given a receiver on the channel. This receiver
will be able to wait for a new message to arrive. This design permits for a
module to register a callback when a new message arrives.
