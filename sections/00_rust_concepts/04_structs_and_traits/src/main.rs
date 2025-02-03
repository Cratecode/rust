// This is a struct.
// It's a way to group data together into
// little packets.
// Structs are similar to classes in Java/C#, but
// they only contain data - no functions or methods.
// In order to save time, Rust has a way to
// "derive" implementations for certain traits (more
// on that later).
// The code "#[derive(Clone, Debug)]"
// is telling Rust to create implementations for the
// Clone and Debug traits for our struct.
// What this means is that, by implementing the
// Clone trait, we can run the `clone()` function on
// an instance of our struct to clone it.
// Deriving the Debug trait means that we can do
// something like println!("{user:?}") to
// print out an instance of User for debugging.
// The ":?" inside the print means to use debug printing.

/// A User record, containing basic information about their account.
#[derive(Clone, Debug)]
pub struct User {
    /// The user's unique ID.
    id: u32,
    /// The user's display name (username).
    name: String,
    /// The user's bio (information about them).
    bio: String,
}

// If we want to add methods to our struct,
// we can use an impl block.

impl User {
    // Functions inside the impl block can be called
    // on instances of User.
    // They can also be called like User::my_function().
    // Most of the time, the first argument of our
    // function should be "&self", which means that
    // the function takes in an **immutable** reference
    // to the struct that it's implemented on.
    // When we call a function like "user.id()", Rust
    // will transform this into "User::id(&user)".
    // So the first argument (self) ends up being the
    // value that the function's called on.

    /// Returns the user's ID.
    pub fn id(&self) -> u32 {
        // self is a User struct, so we can
        // access fields on it using this
        // dot syntax, just like in Java/C#.
        self.id
    }

    // The function above didn't return a reference because
    // it was returning a primitive.
    // We actually could use a reference there, but it'd
    // be all but pointless because a reference takes up
    // about the same amount of memory as a number anyway,
    // so we wouldn't be saving anything by doing it.
    // Strings, on the other hand, can be quite large, so we
    // might not want to create copies of them.
    // Instead, we can return a reference, which can
    // save memory and increase performance.
    // If the code calling this does need to copy the
    // String, then it can do itself like this:
    // user.name().clone()

    /// Returns the user's name.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Returns the user's bio.
    pub fn bio(&self) -> &String {
        &self.bio
    }

    // Reading data from the struct isn't the only thing we can do here:
    // we can also set it.
    // To do that, we'll need a mutable reference to the struct,
    // so instead of writing "&self", we'll write "&mut self".
    // We'll also take in a value to set.
    // This function can be called with user.set_bio(new_bio),
    // which is the same thing as User::set_bio(&mut user, new_bio).

    /// Sets the user's bio to a new value.
    pub fn set_bio(&mut self, bio: String) {
        self.bio = bio;
    }

    // Unlike in other languages, we don't need to create
    // a constructor.
    // But it's probably still a good idea to do so.
    // By convention, we'll create a method called "new",
    // but we can really call it anything.
    // To call this, we'll use User::new(id, name, bio).
    pub fn new(id: u32, name: String, bio: String) -> User {
        // For the struct above, this syntax actually won't
        // work in other files.
        // This is because struct fields are **private by default**.
        // In other words, from an outside file/module, we can't
        // access the id, name, or bio fields on our struct.
        // That means that we also can't set them, so we can't
        // create a new instance of the struct.
        // This can actually be a good thing, but if you want
        // other files to be able to create a new value of
        // the struct, you'll need to add "pub" before each of its
        // fields above. For example, "pub id: u32".
        User {
            // This means the same thing as id: id,
            // or setting the id field to the id variable
            // above.
            // Rust lets us collapse it if both have the same name.
            id,
            name,
            bio,
        }
    }
}

// We can implement traits like this.
// Here, we're implementing the Display trait for User.
// This trait lets us write things like println!("{my_user}").
// Notice that this is different from the Debug trait above:
// they both do essentially the same thing, but the Debug trait
// is used for formatting a struct as a String for debug information,
// and the Display trait is used for formatting the struct as a String
// so it can be displayed to the user.
impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ID: {}\nName: {}\n============\n{}",
            self.id, self.name, self.bio
        )
    }
}

// Now, we can put everything together and create
// a function that takes users in and prints them out.
// This takes in a reference to a user because we don't need
// to take ownership of it to print it out.
// If we wrote "user: User", then everything would still work,
// but the code that called this function wouldn't be able
// to use its user anymore because ownership of it would be transferred
// into here.

/// Prints a user to the console, with newlines before and
/// after the user.
pub fn print_user(user: &User) {
    println!("\n{user}\n");
}

fn main() {
    // We need to write into() to convert string literals,
    // which have a type of &str, into a String.
    // The differences between these two are subtle, but
    // &str is a reference to a string, and String is
    // an actual string.
    let mut my_user = User::new(
        1,
        "Admin".into(),
        "Please contact webmaster@example.com if you notice any problems.".into(),
    );

    println!("User's ID: {}", my_user.id());
    // We could also write println!("{:?}", my_user).
    println!("Debug: {my_user:?}");

    print_user(&my_user);

    my_user.set_bio("Maintenance in progress...".into());
    println!("New Bio: {}", my_user.bio());
    println!("Debug: {my_user:?}");

    print_user(&my_user);
}
