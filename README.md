# rustty: a terminal UI library

[![Build Status](https://travis-ci.org/cpjreynolds/rustty.svg?branch=master)](https://travis-ci.org/cpjreynolds/rustty)

- [API Documentation](http://cpjreynolds.github.io/rustty)
- [Intro](#intro)
- [Installation](#installation)
- [Concepts](#concepts)
	- [The Terminal](#the-terminal)
	- [Cells](#cells)
	- [Events](#events)
- [Usage Guide](#usage-guide)
- [Contact](#contact)

## Intro

Rustty is a terminal UI library that provides a simple, elegant abstraction
over the underlying system.

Rustty is based on the concept of cells and events; a terminal is an array of
cells, each holding a character and a set of foreground and background styles.
Events are how a terminal communicates changes in its state; each event
represents some form of action by the user, be it a keypress or a window resize.
Each terminal has an event stream that receives input events that a program can
then respond to.

## Installation

Installation is simple, to use `rustty`, first add this to your `Cargo.toml`:

```toml
[dependencies]
rustty = "*"
```

Then, add this to your crate root:

```rust
extern crate rustty;
```

## Concepts

The purpose of this section is to introduce and explain the main concepts of
rustty.

### The Terminal

The terminal representation can be thought of as such:

```
0-------------------------------cols (x)
|
|
|
|
|
|
|
|
rows (y)
```

Along the x-axis are the columns and along the y-axis are the rows. The
upper-left corner is the origin, which begins at index (0, 0) and extends to
(cols, rows). Each point (x, y) represents a single cell, which is the next
topic.

### Cells

A cell is a single unit on a character display, it represents a single
character, and its foreground and background styles. A terminal is a large array
of cells.

### Events

Events are how changes in the terminal state are represented and given to a
program to respond to. Each terminal has an event stream; when an event occurs,
such as a keypress, it is pushed on to the back of the stream, when a program
reads an event from the stream, it is taken from the front of the stream. Events
are asynchronous by nature and are dealt with as such, reads and writes from the
event stream can occur asynchronously and without resource contention.

## Usage Guide

Examples and usage suggestions can be found in the [API
documentation](http://cpjreynolds.github.io/rustty).

## Contact

If you have any issues with the library please report them with the issue
tracker.

If you have any further questions don't hesitate to email me, and I will try to
respond promptly.
