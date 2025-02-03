// This shows some examples of how lifetimes work in Rust.

fn update_reference(reference: &mut Vec<String>) {
    // Rust string literals are `&str`.
    // We need to use `to_string()` to turn it into a `String` type.
    // This is actually related to lifetimes.
    // It's difficult to store `&str` in the `Vec` because it's a
    // reference, and we don't know how long it will live for.
    //
    // The string below will actually have the type `&'static str`, which
    // means that the reference will have a static lifetime (i.e., live forever).
    // So, we could use a `Vec<&'static str>` and store our string in it directly.
    // This will only really work for string literals though.
    // Using `String` lets us accept any string, including ones that were made
    // at runtime.
    let my_string = "New String".to_string();

    // Add the `String` to the end of `Vec`.
    // This only works because we have mutable access to
    // the `Vec` (i.e., &mut).
    //
    // This function moves `my_string` into the `Vec`, so it
    // won't be accessible anymore (unless we ask the `Vec` for a reference
    // to it).
    //
    // We can tell that it's moved because we give the value directly.
    // If it looked like `reference.push(&my_string)` (and we changed
    // the type to match), then a reference would be pushed, and we
    // would still be able to access `my_string`.
    //
    // However, this would give us a lifetime error.
    // That's because `my_string` lives in this scope, so after
    // the function ends, the `Vec` would still have a reference to it,
    // but `my_string` wouldn't have existed anymore.
    // So, the compiler would give an error.
    reference.push(my_string);

    // This code will work just fine though.
    // It will change the `Vec` to a new `Vec`.
    // Look at where this function is called for an example.
    let new_vec = vec![];
    // This * means dereference, which gets us the actual
    // Vec instead of the reference to it.
    // By setting it equal to the `new_vec`, we change the data
    // stored inside the variable that the reference points to
    // to `new_data`.
    // Without this operator, we would just be changing our parameter
    // to `new_vec`, which wouldn't change the actual `Vec`.
    // It would also give a type error, since our parameter is `&mut Vec<String>`
    // and we're trying to put a `Vec<String>` into it.
    *reference = new_vec;
}

fn main() {
    // `into()` does a type conversion for us automatically.
    // It's a really cool feature of Rust, and is due to the
    // trait system (which we'll get into later).
    // This only works if Rust can figure out what to
    // convert it to.
    //
    // Rust figures that out from our function call below,
    // which takes in a `&mut Vec<String>`, so clearly this
    // must be of type `Vec<String>` (even if we don't specify),
    // and so `into()` will convert from `&str` to `String`.
    let mut my_vec = vec!["a".into()];
    println!("{my_vec:?}");

    update_reference(&mut my_vec);
    // Printing here is fine because the `&mut` above no longer exists.
    // It's been moved into the function, and so after the function is
    // done running (i.e., now), it no longer exists.
    println!("{my_vec:?}");

    // At this point, `my_vec` will be an empty `Vec`.
    // Notice that in the function above, we never
    // call any clear or remove functions.
    // What's happening is `update_reference` is actually
    // changing the value stored in `my_vec` to a brand-new
    // empty `Vec`.
    //
    // Even though it creates the `Vec` inside its own local scope,
    // because we move it into the variable pointed to by the reference,
    // the new `Vec` has the same lifetime as the old one.
    // By moving, we can "get around" lifetimes.
    // Moving can look like this (where we update a variable
    // from a reference), or it can look like returning
    // a function.
    //
    // When a function returns, its return value is moved
    // into the function calling it.
    //
    // The main thing to consider is that we can't move references
    // out of a function if they point to variables inside it.
    // This is because moving something like this will extend its lifetime,
    // but the thing being pointed at will still have its old lifetime.
    // So, the reference would end up outliving the thing it's pointing at.
    // This would create undefined behavior, so the compiler gives an error.
    //
    // One notable exception to this rule is when we return a reference based
    // on our function arguments.
    // Here's an example (`maybe_bad_function`):

    my_vec.push("new string".into());
    // Print can handle variables nicely like how we've seen above,
    // but more complicated things (like expressions and function calls)
    // need to use this syntax.
    println!("{}", maybe_bad_function(&mut my_vec));
}

fn maybe_bad_function(list: &mut Vec<String>) -> &String {
    // This syntax (where there's no semicolon) is the same thing
    // as return.
    // As with a lot of things in Rust, it has more uses, but in this case,
    // it's the same.
    //
    // This function seems to totally break our rules.
    // Normally, we aren't able to return references, but that rule is there
    // for a specific reason.
    // If a reference lives longer than what it's pointing at, we get undefined behavior.
    // But this reference won't do that.
    //
    // The `Vec<String>` lives outside this function.
    // We can think of it as always being "above" whatever we
    // put here.
    // Wherever the `Vec` is, it's literally above this function call.
    // And because of that, any reference we make to the `Vec` will
    // never outlive it.
    // If we return a reference, that reference will die before the `Vec` does.
    //
    // Because of that, Rust allows this.
    // This is due in part to something called lifetime elision.
    // The function should really look like:
    // `fn maybe_bad_function<'a>(list: &'a mut Vec<String>) -> &'a String`
    // Which basically says that the `Vec` lives for a certain amount of time,
    // and the thing that we return lives for less than or equal to that amount of
    // time.
    //
    // For complex cases, you need to write this out yourself, but Rust
    // can do it automatically here.
    &list[0]
}
