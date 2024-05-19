# Rust Syntax

With all of that out of the way, it's finally time to actually start using Rust!
Once you have a proper understanding of how to write Rust,
we'll go over the concepts discussed previously in different projects.
Without any further ado, let's see how Rust is written!

## Entrypoint

If you create a new Rust project,
you'll start off with a file (named `main.rs` inside of the `src` directory) that looks something like this:
```rust
fn main() {
    println!("Hello, Cratecode!");
}
```

If you run this (with the `cargo run` command), then it will print `Hello, Cratecode!` to the screen.

Let's examine how it's put together.

The first part is `fn main() {`.
This defines a function, called `main`, taking in no arguments.
The main function is special: it's what's run when the program is run.

Inside the main function, we have `println!("Hello, Cratecode!")`.
This looks like a function call, but isn't.
It's what's called a **macro**, and you can think of it as a function that gets evaluated at compile time.
What that means is the compiler "calls" the `println` macro,
with an argument of `"Hello, Cratecode!"`, and that macro spits out some new code which does the printing.

Macros are used extensively in Rust, and they help create code that's more compact.
They open the doors to a ton of exciting features—which we won't get into just yet—but you can think of macros
that look similar to `println` as "fancy function calls".

## Variables

Variables in Rust are written almost exactly like how they're written in TypeScript. Here's an example:
```rust
let my_variable = 10;
```
*(keep in mind that, by convention, variable and method names are written in snake_case)*

By default, Rust will figure out what data type a variable is for us, but if it can't
(or we just want to specify it in code), that can be written like this:
```rust
let my_variable: u32 = 10;
```
*(`u32` is an unsigned 32-bit integer, see [here](https://doc.rust-lang.org/book/ch03-02-data-types.html) for more info on Rust's data types)*

Also by default, Rust's variables are immutable.
That means that both the variable itself cannot be changed to a new value,
nor can any of the "inner data" inside it be changed (there are ways around this one, which we'll get into later).

If we want to be able to change a variable, we need to mark it as mutable, like this:
```rust
let mut my_mutable_variable = 10;
// Now, it can be modified.
my_mutable_variable = 100;
```

## Functions

We already took a look at the main function above, so you should have an idea of how they're defined.
Here's another example of a function, which we'll deconstruct:
```rust
fn my_function(arg_1: u32, arg_2: bool) -> bool {
    // Mysterious function that returns a bool.
}
```

First off, we use `fn my_function` to define a function called `my_function`.
Then, the parentheses `()` are used to define what parameters the function takes in.
In this example,
there's a parameter called `arg_1` (which takes in a `u32`—an unsigned, 32-bit integer), and another one called
`arg_2` (which takes in a `bool`—a boolean, true or false).
Then, if we want to specify what the function returns (if it returns anything, which it doesn't need to),
the `->` is used.
In this case, `-> bool` means it will return a boolean.

## Control Flow

### If Statements

We can use if statements and loops to control the way our program runs. If statements are pretty simple in rust:
```rust
if a == b {
    // Do this.
} else if b == c {
    // Or do this.
} else {
    // If nothing works, do this.
}
```

Notably, we don't put parentheses around conditions,
and as a tradeoff, **MUST** always use curly brackets around our code.

### For Loops

For loops, on the other hand, are less simple in Rust.
They always work on iterators (`for of` loops in JavaScript, `for in` in Python, and `foreach` in Java),
which means that they go through each item in some sort of collection (an iterator).

If we wanted to write a conventional for loop in Rust, we could do it like this:
```rust
// Print each number from 0 up until, but not including, 5.
for i in 0..5 {
    println!("{i}");
}

// 0
// 1
// 2
// 3
// 4
```

This loops through a range, which, as the name implies, is a range of numbers from some start to some end.
A range like `a..b` includes every number from `a` up to, but not including, `b`.
We can also write a range like `a..=b`, which includes every number from `a` to `b`:
```rust
// Print each number from 0 up 5.
for i in 0..=5 {
    println!("{i}");
}

// 0
// 1
// 2
// 3
// 4
// 5
```

We can use for loops whenever there's something that can be iterated over.
For example, an array or a `Vec`.
When we loop through these things, we need to be careful whether we're moving the data.
For example:
```rust
// vec! is another macro which creates a `Vec` from a series of elements.
let my_vec: Vec<u32> = vec![1, 2, 3];
for item in my_vec {
    println!("{item}");
}

// `my_vec` is no longer accessible because it was "moved into"
// the for loop.
// Instead, if we did `for item in &my_vec`, then we would still
// be able to access `my_vec` later.
// The tradeoff is that, instead of `item` being of type `u32`, it'll
// be of type `&u32`.
```

## While Loops

While loops, on the other hand, are much simpler.
They just take a condition, and keep looping until that condition is met (or `break` / `continue` was used).
For example:
```rust
while a < b {
    // Run code.
}
```

Just like if statements, we don't use parentheses around the conditions,
and the curly brackets around the code is required.

If we want to do infinite loops, we can use the special `loop` keyword:
```rust
loop {
    // This code will run forever.
}
```

## Expressions

Like a lot of things, Rust expressions are a bit more complicated (and a lot more powerful).
For example, in Rust, we can write code like this:
```rust
let a_plus_b = {
    let a = 5;
    let b = 6;

    a + b
};

// a_plus_b == 11
```

When we create a block (the curly braces `{}`), we can actually "return" a value from it.
Note that we can't do "early returns" here.
Instead, whatever's at the end of the block will be used as that block's "value", so long as it doesn't have a semicolon.

This gives us a neat way to organize code.
Scoping means that any variables within the block won't exist outside it,
so we can write out complex expressions using blocks in order to make them more readable.

We can actually think of a lot of things in Rust as expressions and blocks.
A function, in fact, can be thought of as a block:
```rust
fn a_plus_b(a: u32, b: u32) -> u32 {
    a + b
}
```

Just like with blocks, this function has curly braces around its code,
and the last item (without a semicolon) in the block will be used as the function's return value.
So, we can think of functions and blocks being pretty similar.

Not only that, but if statements (and the code inside them) works nicely with blocks:
```rust
let a_or_b = if condition {
    a
} else {
    b
};
```

An if statement can actually be used as an expression!
In this case, if our condition is true, then `a_or_b` will be `a`, otherwise it will be `b`.
There are all sorts of things in Rust that can be used as expressions,
and using them gives us new ways we can express our code.

## Conclusion

Now that you understand some of Rust's basic syntax,
there's just one more concept to cover before you can get to building some projects.
Next, we'll cover how to store data in rust using structs and traits
(which you can think of as the Rust equivalent of object-oriented programming).
Also, take a look at some of the example code in the previous lesson to get a bit more practice with Rust's syntax,
with this information in mind.
Happy coding!