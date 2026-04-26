# TinyLang

A Simple ruby-like language written in Rust.

Built for Rust pathway in Resolution YSWS by Hack Club.

## Getting Started

### Installation

Download the binary from [Releases](../../releases) or build from source:

```bash
cargo build --release
./target/release/tinylang --file script.tl
```

## Quick Example
```
yell "Hello, TinyLang!";
x = 42;
yell x;

craft greet(name) 
  yell name;
end;

greet("World");
```

## Stack Used
- Rust
- LALRPOP
- ureq for HTTP requests

## Features
- Supports If-Else statements
- Has some weird funny keywords like "yell" to print text and "craft" to define functions, or my favourite "cap" for false and "nocap" for true
- Also supports HTTP requests
- Implemented a while loop too
- Supports File I/O through built-in functions

## Reference to build the project
https://rust.resolution.hackclub.com/guides/programming-language