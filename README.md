# FPS Lang

A `frames per second` meme programming language 🙃 written in `Rust`

The concept of this language is to execute statements on a `per frame` level

The lexing/parsing concept is based on the book [Crafting Interpreters](https://craftinginterpreters.com/)

The weird `FPS` part was just a silly idea I had when thinking about `frames per second` in video games 🙃

All lexing and parsing was initally developed completelly manual without using any third-party libraries. I might change this in the future.

Currently I'm not using any lifetimes or Arc/Rc/RefCell as I was too deep into development when I wanted to make this change. I might be adding this in the future in order to remove all those **ugly** `.clone()` 👌

## Logic

### Frame concept

\# represents a frame

```rust
let a=1; # print(a); ## <- exit program on last frame
^        ^
|        |_frame 1 -> 'print(a);' will be executed on frame 2
frame 0 -> 'let a = 1;' will be executed on frame 1
```

### Weird example

And this is where `FPS Lang` shines at being `weird` 🤣

```rust
// this is FRAME 0
print("printed at frame 1 - declared at frame 0");
let a = 0;

#3 // frame 1 will be executed 3 times
print("printed at frames 2|3|4 - declared at frame 1");

for 0..=1 {
    print("printed at frames 2|3|4|5|6|7 - declared at frame 1 inside for loop"); 
    a = a + 1;
    print(a);
}

# // frame 4
print("printed at frame 5 - declared at frame 4");
print(a); // should print 4
##
```

output
```rust
FPS 1 -> printed at frame 1 - declared at frame 0
FPS 2 -> printed at frames 2|3|4 - declared at frame 1
FPS 2 -> printed at frames 2|3|4|5|6|7 - declared at frame 1 inside for loop
FPS 2 -> 1 // 'a' value - printed inside for loop
FPS 3 -> printed at frames 2|3|4 - declared at frame 1
FPS 3 -> printed at frames 2|3|4|5|6|7 - declared at frame 1 inside for loop
FPS 3 -> 2 // 'a' value - printed inside for loop
FPS 4 -> printed at frames 2|3|4 - declared at frame 1
FPS 4 -> printed at frames 2|3|4|5|6|7 - declared at frame 1 inside for loop
FPS 4 -> 3 // 'a' value - printed inside for loop
FPS 5 -> printed at frames 2|3|4|5|6|7 - declared at frame 1 inside for loop
FPS 5 -> 4 // 'a' value - printed inside for loop
FPS 5 -> printed at frame 5 - declared at frame 4
FPS 5 -> 4 // 'a' value - printed at frame 5 (declared at frame 4)
FPS 6 -> printed at frames 2|3|4|5|6|7 - declared at frame 1 inside for loop
FPS 6 -> 5 // 'a' value - printed inside for loop
FPS 7 -> printed at frames 2|3|4|5|6|7 - declared at frame 1 inside for loop
FPS 7 -> 6 // 'a' value - printed inside for loop
```

## Usage

```sh
cargo run -- --help

Usage: fps-lang.exe [OPTIONS]

Options:
  -r, --repl     Flag to access the REPL
  -h, --help     Print help
  -V, --version  Print version
```

### Execute

`Note` that currently it only executes the content of `sample.fps` file.

I'll be adding a custom file input soon 😊

```sh
cargo run --
```

### REPL

Start a REPL

```sh
cargo run -- -r

# REPL  -  FPS Lang #
--------------------
Type '\q' to exit
fps>
```

```sh
fps> let a = 0; print(a); ##
FPS 1 -> 0
```

### Tests

Execute the currently implemented tests

```sh
cargo test

running 21 tests
test ast::environment::tests::assign ... ok
test ast::environment::tests::declare_different_env ... ok
test ast::environment::tests::declare ... ok
test lexer::tests::single_char_tokens ... ok
test parser::tests::test_addition ... ok
test lexer::tests::numeric_literal ... ok
test parser::tests::test_comparison_paren ... ok
test lexer::tests::types ... ok
test lexer::tests::string_literal ... ok
test lexer::tests::unterminated_consumption ... ok
test lexer::tests::two_char_tokens ... ok
test lexer::tests::identifer_literal ... ok
test lexer::tests::keywords ... ok
test parser::tests::declaration ... ok
test parser::tests::declaration_assign ... ok
test parser::tests::declaration_print ... ok
test lexer::tests::comment ... ok
test interpreter::tests::multiple_frames ... ok
test parser::tests::test_comparison ... ok
test ast::expr::tests::pretty_print_ast ... ok
test parser::tests::test_print_statement ... ok

test result: ok. 21 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```



## Types

Currently supported types

| Type                 | Declaration |
| -------------------- | ----------- |
| Int                  | 1           |
| Float                | 1. or 1.0   |
| String               | "value"     |
| Boolean              | true        |
| Range(int, int)      | 0..1        |
| RangeEqual(int, int) | 0..1=       |
| Null                 | null        |


### Syntax

#### Frame

`#` represents a frame

```rust
let a = 0;
print(a);
# 
print(a);
##
```

output
```rust
FPS 1 -> 0
FPS 2 -> 0
```

#### FPS Program end

`##` represents the end of the program

#### Declaration / Assignment

Use `let` to declare a variable.

```rust
let a = 0;
```

#### Range

Use `..` surround by 2 digits to define a `Range`

- `Range` is bound inclusively below and exclusively above

Ranges can also be `RangeEqual` by adding a `=` sign to the range `0..=1`

- `RangeEqual` is bound inclusively below and inclusively above.

#### For loop

```rust
for 0..2 { print("hello"); } ##
```

output
```rust
FPS 1 -> hello
FPS 2 -> hello
```

#### If/Else block

```rust
if 0 == 1 {
    print("equals");
} else {
    print("not equals");
}
```

#### Block with scope logic

```rust
let a = 1;

{
    let b = 1;
    print(a + b);
}

##
```

output
```rust
FPS 1 -> 2
```
