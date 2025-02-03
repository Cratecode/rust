// This shows some examples of how ownership and borrowing work in Rust.
// A few lines here are commented out because they would create compiler errors.
// Try uncommenting them to see those errors.

fn main() {
    // The list ([1, 2, 3]) lives in/is owned by "arr".
    let arr = vec![1, 2, 3];

    // Now, the list is being moved to arr2.
    let arr2 = arr;

    // This code won't compile because there isn't
    // anything in arr anymore.
    // It's in arr2.
    //
    // println!("{arr}");

    // If we want to modify the list, we need to mark it as mutable.
    let mut arr = arr2;

    // This moves the list from arr2 to a new variable called arr (which "hides" the previous one).
    // This variable is marked as mutable, so we can modify it.
    // Rust variables are immutable by default.

    // With that, we can modify it.
    arr[0] = 100;

    // If we want to pass data around, we can use a borrow.
    let arr_borrowed = &arr;

    // This is a reference to arr, not the data inside of it.
    // If we tried to run something like:
    //
    // let new_arr = arr;
    //
    // It would fail, because we would be moving arr to new_arr.
    // That would make our reference (arr_borrowed) "invalid", as
    // it would still reference arr, but arr wouldn't really exist anymore.
    //
    // Rust notices this and gives us an error saying that arr cannot be moved
    // while a reference to it exists.
    // If we want to move it, we need to get rid of the reference.

    // Rust has two different types of reference: mutable reference and immutable reference.
    // The reference above is immutable, which means that we can't use it to modify the thing
    // being referenced.
    // We can either have one mutable reference, or as many immutable references as we'd like.
    //
    // This code will cause an error because we're trying to modify the list (which creates
    // a mutable borrow automatically) while we have an immutable reference.
    //
    // arr[0] = 1000;

    // Rust is smart and won't give those errors unless the reference is actually used.
    // If the reference isn't used, it may as well not exist, so the error would be pointless.
    println!("{}", arr_borrowed[0]);

    // If we want to explicitly create a mutable reference, it would look like this:
    let arr_borrowed_mut = &mut arr;

    // Rust allows us to create that (even though it violates the rules) because it isn't
    // actually used, so it may as well not exist.
    // If we did this, it would cause an error:
    //
    // arr_borrowed_mut[1] = 1;

    // This is here for the same reason as the println above.
    // If the line above was uncommented without this println, there would be no error,
    // because Rust would realize that, even though we're modifying the list while
    // an immutable borrow exists, that borrow wouldn't have been used since before
    // the modification, and if we pretend that it was destroyed before we modified the list,
    // the rule wouldn't be broken.
    // It wouldn't have actually been destroyed, of course, but if we don't use it past a certain
    // point, then it may as well not exist past that point, for all intents and purposes.
    println!("{}", arr_borrowed_mut[1]);
}
