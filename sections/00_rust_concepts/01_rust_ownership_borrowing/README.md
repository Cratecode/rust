# Rust Ownership and Borrowing

The way Rust treats variables is fundamentally different from how they're handled in many other languages.
Take a look at this JavaScript code:
```js
const arr = [1, 2, 3];
const arr2 = arr;

arr[0] = 1000;
console.log(arr, arr2); // [1000, 2, 3] [1000, 2, 3]
```

In many languages, variables act like names to assign to pieces of data.
In this example, `arr` and `arr2` are both aliases that really mean the same list.
This is not how variables work in Rust.

## Ownership

Rust variables have the concept of "ownership".
This basically means that a variable owns a piece of data.
Another way to think about is it that a piece of data actually lives inside the variable itself.

If we wanted to write the same piece of code in Rust, it wouldn't do the same thing. If we have something like this:
```rust
let arr = vec![1, 2, 3];
let arr2 = arr;
```

Then the data (a Vec, which is basically the same thing as an array in JavaScript and a list in other languages) is first being placed inside of `arr`, then being **moved** from `arr` to `arr2`.
After it's moved, there isn't anything in `arr`, so we can't use it.

This is the core idea behind how variables work in Rust.
Instead of thinking of variables as a way to name data, think of them as the place where the data is actually stored.
If the variable no longer exists (i.e., goes out of scope), then neither does the data.

## Borrowing

Of course, there's still a way to emulate the JavaScript code above.
We can create these sorts of "aliases" by using **references** to our variables.
These are also called **borrows** (you can think of it as borrowing the data from a variable).
It's important to consider that,
just like how Rust variables represent where the data "lives", references point to variables, not data.
If we move things around,
we'll get compiler errors because Rust doesn't let us have a reference to something that doesn't exist
(check out the code for an example).

Now, let's try writing that code in Rust:
```rust
// We need to mark variables as mutable so that we can modify them.
let mut arr = vec![1, 2, 3];

// This creates an immutable reference.
// We won't be able to modify the list through it.
// If we wanted to, we would need to write &mut arr instead.
// If we used let mut here, it would let us set the variable to a
// new value, but wouldn't give us access to modifying the list.
let arr2 = &arr;
```

But that's about as far as we can go.
One of Rust's rules with borrows is that we can't modify data if an immutable reference to it exists.
Another big rule is that we can have one mutable reference or as many immutable references as we want, but never both.
So, we're stuck.
This is one of the things that creates the most frustration when working in Rust.
There's always a way forward, but it either requires thinking of our problem differently,
or using other tools that Rust provides us.
In most cases, we'll have to take the first option, and this case is no exception.

If we wanted to get the same result, we could simply use:
```rust
let mut arr = vec![1, 2, 3];

arr[0] = 1000;

// The :? means use the Debug formatter, since Vec doesn't have a Display formatter.
println!("{arr:?} {arr:?}");
```

References are extremely useful when dealing with functions.
They let us pass our data to a function without actually giving the data to the function
(so the function "borrows" it instead).
For example:
```rust
// This will move `data` into `my_function_1`.
my_function_1(data);
// After this point, `data` won't exist anymore since it was given to the function.

// Instead, we can pass a reference to the function, which gives it
// access to the data but doesn't transfer ownership of the data to it.
my_function_2(&data);

// A function will decide whether it wants a reference to the data, or the data itself.
// It will also decide whether it wants an immutable reference, or a mutable one.
// You don't get to how a function takes in data. If it requires a mutable reference, you must
// give it a mutable reference. And if it requires the data itself, you must give it the data.
//
// One nice thing about this is that it's immediately obvious what's happening.
// If a function argument looks like `&mut data`, the function is probably modifying `data`.
// If it looks like `data`, then `data` won't be accessible after the function is run.
```

### Mutable References

Mutable references are similar to normal references, but they let us modify the data that they're borrowing.
There can only ever be one mutable reference, and it can't exist if there are immutable references.

In general, this is fine.
There aren't that many cases where you'd need to give away two mutable references simultaneously.
Computers run programs sequentially (one step after another), so you can just create mutable references as needed.
In fact, the place where this falls apart is with concurrent programming, where steps aren't executed one after another.
There are more language features that can be used to circumvent that, but we'll talk about those later.

## Lifetimes

We'll go into more depth on lifetimes in the next lesson,
but the big idea behind them is that data only lives for a certain amount of time on the computer.
Take a look at this code, for example:
```rust
// This creates an uninitialized variable.
// Rust only allows this to exist if it's guaranteed that
// it'll be given a value somewhere in the code.
// Rust will also infer its type from how its used / what it's assigned to.
let list_ref;

// This creates a "scope" (actually, in Rust it's even more powerful, but for most languages this is just a scope).
// It pretty much does the same thing as
// an if statement that always runs.
// At the end of the block, any variables that were created
// can't be accessed anymore, and so are removed from memory.
{
    let list = vec![1, 2, 3];
    
    list_ref = &list;
}

println!("{list_ref:?}");
```

This code will not work.
The issue is that we're setting `list_ref` to a reference of `list`, but `list` only exists within that block.
As soon as it isn't accessible anymore, it'll get removed from memory,
so the `println` wouldn't have anything to access.

In some languages, doing something like this might lead to undefined behavior.
Our reference would still point to a place in memory (specifically, the point that the Vec used to take up),
but that memory could contain anything at all when we print it.
Because this leads to undefined behavior, Rust gives us an error:
```rust
error[E0597]: `list` does not live long enough
  --> main.rs:16:16
   |
16 |     list_ref = &list;
   |                ^^^^^ borrowed value does not live long enough
17 | }
   | - `list` dropped here while still borrowed
18 |
19 | println!("{list_ref:?}");
   |            -------- borrow later used here
```

This is the basic idea behind lifetimes.
Data only lives for a certain amount of time,
and if references outlive the things they point to, issues start to creep up.
To fix this, we would need to make `list` live longer.
An easy way to do that is to move it into a variable that lives longer:

```rust
let new_list;
let list_ref;

{
    let list = vec![1, 2, 3];
    
    // It needs to be done in this order because
    // of our rules with moving while we have a reference.
    new_list = list;
    list_ref = &new_list;
}

println!("{list_ref:?}");
```

Alternatively, we could just remove the scope.
There are also nicer ways to do things like this, which we'll look into soon.
Happy coding!