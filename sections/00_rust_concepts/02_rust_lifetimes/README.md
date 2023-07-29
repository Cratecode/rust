# Rust Lifetimes

Just to recap from the previous tutorial,
lifetimes in Rust indicate how long a value lives (exists) for, and what that means for references.
References can't outlive the variable they're pointing to,
or else
we get undefined behavior because the memory the reference points
to can have anything in it once the actual data is gone.

In order to prevent undefined behavior
(or rather, compiler errors, since the compiler will refuse to compile if there is undefined behavior like this),
we need to play special attention to lifetimes.

There are a few general ideas, but here's a function that illustrates why lifetimes are important:
```rust
fn bad_function() -> &Vec<i64> {
    let my_vec = vec![1, 2, 3];
    &my_vec
}
```

This function will fail to compile. In fact, here's the error:
```rust
error[E0106]: missing lifetime specifier
 --> src/main.rs:1:22
  |
1 | fn bad_function() -> &Vec<i64> {
  |                      ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
  |
1 | fn bad_function() -> &'static Vec<i64> {
  |                       +++++++
```

The compiler tells us what the issue is, and also gives us a suggestion.
This suggestion isn't useful in this case (we'll get into what `'static'` means) though.

The issue here is the same as the one described above.
`my_vec` lives inside the function, and will no longer exist once it returns.
But that reference will be given to whatever function calls it,
and so will live longer than the thing being referenced.
This is bad, and so the compiler rejects it.

In general, references "flow downwards".
Functions pass references as arguments, but references usually aren't given as return values.
If we want to get a value out of a function, there are a few things we can do.

## Moving

One simple solution is moving.
Just like how we moved data from one variable to another, we can move data from one scope to another.
Doing so will extend the lifetime of our data, and everything works out nicely.
For example:
```rust
fn good_function(my_vec: &mut Vec<i64>) {
    let special_data = 1000;
    
    // There's no references here.
    // `special_data` is being moved
    // into the `push` function, and because
    // that function adds it to the `Vec`,
    // `special_data` is effectively moved into
    // the `Vec`.
    my_vec.push(special_data);
}
```

The idea here is pretty simple.
If we move data upwards, then that data will live for longer (and will be accessible from outside the function).

## Returning

Similarly, we can also just return data.
You can also think of returning as a form of moving.
Returned data will be moved into the function calling it.
```rust
fn good_function() -> i64 {
    let special_data = 1000;
    
    // This syntax is the same thing as
    // return.
    special_data
}
```

## Longer Lifetimes

The most complicated way is through longer lifetimes.
Check out the example file for a more comprehensive example, but it basically comes down to this:
```rust
fn probably_good_function(my_vec: &mut Vec<String>) -> &String {
    &my_vec[0]
}
```

Remember, our rule is that a reference can't outlive the thing it points to.
And in this case, that checks out.
That `Vec` that we reference exists outside our function, and will live for longer than the reference we return.
This is because, if we think about the program as a series of sequential steps,
the `Vec` was created before this function was run, and so will be removed after any references we return are removed.
Again, check out the example file for an actual example of this.

## 'static Lifetimes

This is pretty much just an extension of what's above, but it's worth understanding.
The compiler told us that `'static` might help us solve our problem, and in some cases, it will.
`&'static ...` (the format `&'a ...` is just a way to give a "name" to a lifetime,
but `'static` has a special meaning) means that we have a reference to something,
and that something has a static lifetime.
A static lifetime means that the value, for all intents and purposes, lives forever (i.e., until the program ends).
We can return references with static lifetimes because those references, by definition, will always point to something.

An example of something with a static lifetime is a string (specifically, an `&str`).
If we write a string literal, then that string will always exist in the program,
and so will have a static lifetime (written `&'static str`).
Therefore, we can write a function like this:
```rust
fn definitely_good_function() -> &'static str {
    "my static string"
}
```

And Rust will accept it.
Normally, we need another lifetime to use as a bound
(like how the example above uses `&mut Vec<String>` as a bound for how long `&String` will live),
but because `'static` lives forever, there's no need.
Rust will just accept it.

## Conclusion

A lot of the concepts we've covered are new and tricky.
This should give you a basic overview of how they work,
but you'll need a lot of practice before you actually get the hang of it.
Luckily, Rust will give you helpful error messages to guide you,
and so long as you have the basics down, you'll always be able to make progress forward.
The next lessons will start to actually look at how Rust is written instead of the core concepts,
so there'll be a lot less theory and a lot more coding moving forward!