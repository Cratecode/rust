# Rust Enums

Previously, we looked at how structs can be used to store data in Rust. Another way to organize data is with enums.

In most languages, enums are a data type that can take up different states. For example, you could have a `Status` enum for some operation with the states `Started`, `InProgress`, `Success`, `Error`. Usually, anywhere we can use an enum, we could just use a simple number (`0` for `Started`, `1` for `InProgress`, etc.), but enums give us a human-readable way to express these different states and prevent invalid states from being expressed (using an enum, we can only specify the states listed above, but if we swapped them out for a number, we wouldn't just be limited to the numbers we expect, which can cause bugs in our code). Enums in Rust are a bit different though.

Rust's enums can behave exactly as described above - but they're much more powerful. In Rust, enums aren't just limited to storing different states - they can also store data. That's a bit abstract, so let's take a look at an example.

## Simple Enums

Before we begin, let's get Rust's syntax for enums down. We'll use the example from above:

```rust
/// The status of a request.
#[derive(Debug, PartialEq)]
pub enum Status {
    /// The request has just begun.
    Started,
    /// The request is running but hasn't been completed yet.
    InProgress,
    /// The request succeeded.
    Success,
    /// The request failed.
    Error,
}
```

To declare an enum, we'll use the `enum` keyword. We can also declare it as public (so that other files can access it) by putting `pub` before it. After that goes the name of the enum, and inside the curly brackets goes each variant of the enum, separated by a comma.

Now, take a look at the line where it says `#[derive(Debug, PartialEq)]`. This piece of code automatically derives a trait implementation for whatever it's placed on. What that means is that Rust will automatically implement the `Debug` and `PartialEq` traits for us. The `Debug` trait lets us print out debug information about the type, and is usually a useful trait to have implemented, but the `PartialEq` trait is a bit more interesting. By default, we aren't able to perform basic operations (like equality) on data, unless the trait corresponding to it has been implemented. In this case, the trait needed for `==` is `PartialEq`, so by putting it into our `derive`, we have the compiler take care of implementing it for us, which allows us to use `==` with this enum.

Now that we have it set up, let's use it:

```rust
// Imagine these functions send out web requests.

fn good_request() -> Status {
    Status::Success
}

fn bad_request() -> Status {
    Status::Failure
}

fn main() {
    // This == only works because we derived PartialEq above!
    println!("{}", good_request() == Status::Success);
    println!("{}", bad_request() == Status::Failure);
}
```

## Enums with Data

Of course, we wouldn't just want to know whether a request succeeded or not - we'd want to get the data that it requested, if it succeeded, and the error that occurred, if it didn't. Enums make that extremely simple to deal with because they let us directly embed data into them. Take a look:

```rust
/// The status of a request.
#[derive(Debug, PartialEq)]
pub enum Status {
    /// The request has just begun.
    Started,
    /// The request is running but hasn't been completed yet.
    InProgress,
    /// The request succeeded.
    /// Contains the response string.
    Success(String),
    /// The request failed.
    /// Contains the error code.
    Error(u32),
}
```

The only thing different about this enum definition is that `Success` has a `String` inside of it, and `Error` has a `u32` inside of it. And I do mean inside it - we can store a String inside the `Success` response and extract when we need to use it. Creating the data is actually pretty easy: `Status::Error(100)` would put `100` inside of the `Error` variant of `Status`. Data in enums work exactly like tuples and tuple structs, and you can even specify multiple pieces of data to store inside them. The neat part of using enums like this is we can't put an error code inside a `Success` variant; `Status::Success(100)` would give us an error, because it isn't valid. Now, let's see how to take data out of enums!

## Match

Rust's match statement is the bread and butter of dealing with enums. It uses something called pattern matching, which is a very powerful feature that lets you define a "pattern" (what your data should look like). These patterns also contain some placeholder variables that get set to the data taking up the space that they're at in the pattern, which can let you extract data. Pattern matching can be used for enums, structs, ranges of numbers, and a whole host of other things, but we'll be focusing on enums for right now.

The match statement takes a piece of data in and lets you write a bunch of patterns and the code that will get executed if that pattern matches. They're a bit like a nicer, more powerful way to write out a big if-else-if block. Let's take a look:

```rust
impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Status::Started => write!(f, "Request started!"),
            Status::InProgress => write!(f, "Request in progress..."),
            Status::Success(msg) => write!(f, "Request succeeded: {msg}"),
            Status::Error(code) => write!(f, "Request failed with error {code}"),
        }
    }
}
```

Here's an implementation of the `Display` trait for `Status`. Note that we could have added `Display` to our `derive` above, but implementing it manually like this allows us to customize the messages. With this implementation, we can write code like `println!("{status}")` and it will print out a message using the code above.

The most important part is the `match` statement. A match statement takes in a piece of data (in this case, it's `self`, but it could be any variable or expression). Next, inside the curley brackets are all the patterns. The first pattern, `Status::Started` is just one variant of the enum. The match statement will check if `self` is `Status::Started`, and if so, it'll run the code to the right of the arrow, then exit out of the match statement. Otherwise, it'll keep running down the patterns until it finds one that matches.

Let's skip ahead to `Status::Success(msg)`. This pattern is similar to the ones above, but it also includes a placeholder variable. If the data looks like this (i.e., it is a `Status::Success`), then that placeholder variable will be set to the data inside it (which, in this case, is the response string). So, `msg` is a variable set to the data that was inside the status. Note that `Status::Success` wouldn't be a valid pattern here, because there isn't any data that looks like that; success must have something inside it. If we didn't care about what the data was, we could write `Status::Success(_)`. `_` is a special name that throws away anything that gets put into it.

One thing to keep in mind is that match statements must be exhaustive. This means that you must have patterns covering every possible piece of data that could go into the match. When dealing with enums, this rule essentially boils down you needing one pattern per variant of your enum. This requirement can be helpful, as it's all too easy to forget to implement logic for a certain variant, and even easier to add another one later down the line and forget to update some piece of code buried deep in your project.

That being said, if you don't want to have to handle every possible piece of data, there are a few ways around it. The easiest is to use the default pattern (`_`), which matches anything:
```rust
match status {
    Status::Success(msg) => println!("Request succeeded: {msg}"),
    Status::Error(code) => println!("Request failed with error {code}"),
    _ => println!("Not done yet..."),
}
```

By convention, the default pattern should be the last pattern in your match.

## Option and Result

Enums are used almost everywhere in Rust, and the reason is that Rust doesn't have `null`, nor does it have exceptions (other than panicking). Instead, it uses enums. There are two very important built-in enums, called `Option` and `Result`, which specify nullable and fallible values, respectively.

These enums are very similar to the example we were looking at above. Here are their definitions:

```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

The main difference between these enums and the ones that we've been using are that they use generics. If you haven't used them before, generics are essentially like placeholders. If I write out a type like `Result<String, Error>`, then the `T` type argument corresponds to `String`, and the `E` type argument corresponds to `Error`. So, this enum has an `Ok` variant containing a `String` and an `Err` variant containing an `Error`. We use generics because they let us write code for something once, then use that code for almost any type without having to re-write or modify it.

Just like our enum above, we can match on options and results in to extract the data stored inside them. What makes using enums so useful (as opposed to `null` and exceptions that you'll see in other languages) is that you'll never be surprised by a value being `null` when you don't expect it to be, or an error being thrown when you didn't realize there was a chance it would be. If you call a function in Rust that returns an `Option<String>`, you won't be able to do anything with it until you extract the `String` inside of it. This forces you to handle null values and errors, which means that crashes due to null values or exceptions are rare in Rust.

There are a lot of built-in functions for making dealing with these enums easier, and I'd encourage you to check out the documentation for [Option](https://doc.rust-lang.org/std/option/) and [Result](https://doc.rust-lang.org/std/result/). Of these functions, the one of the most important (and most dangerous) ones is the `unwrap` function. Calling this function on an option or result gives you the value inside, and panics if there isn't a value. To be clear, **if you can avoid using the unwrap function, you should**. By calling this function, you're acknowledging that a value may not exist, or an error may be returned, and if that's the case, your program will crash. Unwrap isn't an evil function - in fact, what it does is the default behavior of a ton of programming languages - but if you can, avoid it at all costs. Handling errors and none values properly will make your code much more reliable and significantly less likely to crash. That being said, if your program should crash if it encounters an error, then unwrap is the way to go.

## If Let

One common way to handle enums is with an `if let` statement. You can think of it as a miniaturized version of the `match` statement that only works on a single pattern, and it's super useful when dealing with enums like `Option`. You can write code such as:

```rust
let my_value = Some(50);
match my_value {
    Some(num) => println!("Num: {num}"),
    // This means "do nothing".
    None => {}
}
```

As:

```rust
let my_value = Some(50);
if let Some(num) = my_value {
    println!("Num: {num}");
}
```

If let is basically just an if statement that runs if a pattern matches. Its syntax looks like `if let PATTERN = VALUE { CODE }`. Sometimes using a `match` is a bit overkill, and `if let` can be a lot nicer. If you need to handle multiple different branches (such as running different code on success or on error), you should stick to a match.

## Let Else

Let else if almost like the opposite of if let. Instead of letting you run code if something matches, you can run code if it doesn't. The catch is that code has to return/break, so it's mainly useful for stopping a function/loop early if you don't end up with the right data. You can rewrite code such as:

```rust
let my_value = Some(50);
match my_value {
    Some(num) => println!("Num: {num}"),
    None => return,
}
```

To:

```rust
let my_value = Some(50);
let Some(num) = my_value else {
    return;
};

println!("Num: {num}");
```

The syntax of a let else looks like: `let PATTERN = VALUE else { CODE };`. If the pattern matches, then the code after it will run. Otherwise, the code inside the block will run.

## Question Mark Operator

The last way to deal with these enums is with the question mark operator. It's an operator that specifically applies to `Option` and `Result`, and propagates nones/errors. It functions a lot like `unwrap` and gives you the value inside, but instead of crashing your program if there's no value, it exits the function early and returns the none / error value. At some point, you'll still need to handle the error / none, but this can help you group together and consolidate those checks. Here's an example of it in action:

```rust
fn get_value() -> Option<u32> {
    let my_value = Some(50);
    // Use the question mark to extract the data.
    // If it's None, then None will be returned immediately.
    let my_new_value = my_value? * 100;

    // The function's return type is Option,
    // so we need to wrap this inside Some for it to work.
    Some(my_new_value)
}
```

To make dealing with this a bit easier, you can use the [ok_or](https://doc.rust-lang.org/std/option/enum.Option.html#method.ok_or) function to convert an `Option` to a `Result`, and the [ok](https://doc.rust-lang.org/std/result/enum.Result.html#method.ok) function to convert a `Result` to an `Option`. There are also crates like [anyhow](https://docs.rs/anyhow/latest/anyhow/) which can make error handling much easier.

## Conclusion

Enums in Rust are a powerful tool, and even if you don't create your own enums, you'll use them extremely often in the form of `Option` and `Result`. Get familiar with using them, especially the different methods of extracting data from them listed above. Each of these methods are useful in different situations, and they can be used to significantly clean up your code. Using enums in this way forces you to handle all possibilities, and is one of the easiest (and most effective) steps you can take to increasing the reliability of your software.
Happy coding!