# rustty: a terminal UI library

[![Build Status](https://travis-ci.org/cpjreynolds/rustty.svg?branch=master)](https://travis-ci.org/cpjreynolds/rustty)
[![crates.io](https://img.shields.io/crates/v/rustty.svg)](https://crates.io/crates/rustty)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/cpjreynolds/rustty/master/LICENSE)

- [API Documentation][1]
- [Intro](#intro)
- [Installation](#installation)
- [Concepts](#concepts)
	- [Terminal](#terminal)
	- [Cells](#cells)
	- [Events](#events)
- [Usage Guide](#usage-guide)
- [Contact](#contact)

## Intro

Rustty is a terminal UI library that provides a simple, concise abstraction over an
underlying terminal device.

Rustty is based on the concepts of cells and events. A terminal display is an array of cells,
each holding a character and a set of foreground and background styles. Events are how a
terminal communicates changes in its state; events are received from a terminal, processed, and
pushed onto an input stream to be read and responded to.

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
rustty and the decisions behind its design.

### Terminal

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

A cell is a single point on a character display, representing a single
character and its foreground and background styles.

### Events

Events are how changes in a terminal's state are represented.
A terminal has an associated event stream which acts much like a UNIX pipe,
or a FIFO queue. When events occur they are pushed on to
the back of the stream; when events are read they are taken
from the front of the stream.

## Usage Guide

Examples and usage suggestions can be found in the [API
documentation][1].

## Contact

If you encounter any issues with the library, have questions or suggestions,
please [submit an issue](https://github.com/cpjreynolds/rustty/issues/new).

[1]: https://docs.rs/rustty
