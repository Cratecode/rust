# Structs and Traits

The last thing you need to learn before you can start really diving into Rust is
how to properly store data. If you've ever used an object-oriented language
before (Java, C#, etc.), then you can think of structs and traits as the Rust
version of that.

Structs are essentially ways to create little packets of data. For example,
instead of writing functions that take in a user's data as individual
parameters, like:

```rust
fn user_function(id: u32, name: String, bio: String) {
    // ...
}
```

We can package all this data up into a struct called `User` and use it like
this:

```rust
fn user_function(user: User) {
    // ...
}
```

This can help make writing code a lot easier. Structs are a lot like classes in
other languages, except without functions or methods.

## Structs

In order to create a struct, we need to define it like this:

```rust
struct User {
    id: u32,
    name: String,
    bio: String,
}
```

We start off with `struct`, then the name of the struct afterward. Then, we put
all the struct's fields inside curly brackets. Each field is formatted as
`name_of_field: DataType`.

Note that, by default, structs are private. That means that they aren't
accessible from other files/modules. In order to fix that, you can add a `pub`
before `struct`:

```rust
pub struct User {
    id: u32,
    name: String,
    bio: String,
}
```

Struct fields are also private by default. That means that we won't be able to
access them from outside files/modules. If we want to make them accessible, we
can either create a method to get/set them, or make the fields themselves
public:

```rust
pub struct User {
    pub id: u32,
    pub name: String,
    pub bio: String,
}
```

If we have a struct, we can access its fields using a dot syntax, similar to
many other languages:

```rust
my_user.id // Get the id field of my_user.
my_user.id = 10; // Set the id field of my_user.
```

And if we want to create an instance of a struct, we can use the following
syntax:

```rust
User {
    id: 1,
    // String literals have a type of &str, but
    // the name field wants a String, so we need to
    // use into() to convert it.
    name: "User Name".into(),
    bio: "Bio".into(),
}
```

## Tuple Structs

We can also create a type of struct called a tuple struct. Instead of defining
fields, we can define them like a tuple:

```rust
struct UserTuple(u32, String, String);
```

And instead of accessing them by name, we access them by index:

```rust
my_user_tuple.0 // Get the first (u32) field of the user.
```

Like with normal structs, the syntax for creating a new instance of them is very
similar to how we define them:

```rust
UserTuple(1, "User Name".into(), "Bio".into())
```

It's worth noting that Rust also supports tuples that aren't structs. So, we can
have a function that takes in `(u32, String, String)` directly:

```rust
fn user_tuple_no_struct(user: (u32, String, String)) {
    // Print out the first field in the tuple (u32).
    println!("{}", user.0);
    // ...
}

user_tuple_no_struct((1, "User Name".into(), "Bio".into()));
```

Even though both contain exactly the same data, we aren't able to interchange
them. So the following code wouldn't work:

```rust
user_tuple_no_struct(UserTuple(1, "User Name".into(), "Bio".into()));
```

### Zero-Sized Structs

Rust also has structs that contain no data at all. They are only useful in
certain cases, so we'll only cover them briefly, but you can define them like
this:

```struct
struct NoData;
```

And create them by just using their names:

```rust
NoData
```

## Struct Methods

Even though structs don't have any methods in their definition, we can still add
them another way. We'll use the `impl` block:

```rust
impl User {
    pub fn id(&self) -> u32 {
        self.id
    }
}
```

This creates a function, called `id`, that takes in a reference to a `User`
(`&self` means a reference to the thing that the method is implemented on, which
in this case is a `User`) and returns a `u32`. If we have an instance of a
`User`, called `my_user`, then we can call the function by doing `my_user.id()`.
This means the same thing as `User::id(&my_user)` (which is just a different way
to call these methods).

If we wanted to modify data on the struct, we could create a function like this:

```rust
pub fn set_bio(&mut self, bio: String) {
    self.bio = bio;
}
```

The only difference here is that we take in a `&mut self` instead of a `&self`
(because we need to have mutable access to the data if we want to modify it),
and we take in a second parameter. To call this, we can use
`my_user.set_bio("New Bio".into())`, which is the same thing as
`User::set_bio(&mut my_user, "New Bio".into())`.

Finally, we can write a function that doesn't take in an instance of the struct.
This is useful if we want to write constructors:

```rust
pub fn new(id: u32, name: String, bio: String) -> User {
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
```

## Traits

If Rust structs are like classes without methods, then traits are like
interfaces. They let us write "blueprints" for methods that need to exist on a
class. We can write traits like this:

```rust
trait HasDataID {
    fn get_id(&self) -> u32;
}
```

This creates a trait called `HasDataID`. Inside the trait definition is all the
methods that need to exist on the trait. If we have something that implements
this trait, then it needs to have an `id` method that takes in a reference to
itself and returns a `u32`.

To implement this for user, we can write it as:

```rust
impl HasDataID for User {
    fn get_id(&self) -> u32 {
        self.id
    }
}
```

Then, we can write functions like this:

```rust
pub fn print_id(value: impl HasDataID) {
    println!("{}", value.get_id());
}
```

This special syntax (`impl HasDataID`) means that the function can take in any
piece of data that implements `HasDataID`. So, we could put a `User` in and it
would work just fine. Behind the scenes, this is using generics (which we'll get
into later), and it actually creates a copy of the function for each different
kind of data that gets put into it. You can also use this `impl Trait` syntax
for the return value of a function, so that any code calling it **doesn't need
to worry about how the data is structured, just what it can be used for**.

One of the best things about traits is that you can write functions based on the
implementation of multiple traits by putting a `+` between them. Imagine if you
have a `HasDataID` trait (like the one described above), as well as a `HasName`
trait. With the `impl Trait` syntax, you can create functions that require both
of these traits to be implemented:

```rust
pub fn print_name_id(value: impl HasDataID + HasName) {
    println!("ID: {}", value.get_id());
    println!("Name: {}", value.get_name());
}
```

Now, we can call this function with our `User` struct, but we could also have a
new struct (say, `Lesson`) that would also work just fine with the function
above (so long as it implements the `HasDataID` and `HasName` traits). This
pattern of requiring multiple traits gives us superpowers: not only can we write
a single piece of code to work with multiple different types of data (in a way
that's even more powerful than what can be achieved in traditional
object-oriented languages, as evidenced by the example above), but Rust
libraries are also often designed to take advantage of this fact.

In many web servers, for example, you'll find functions that are capable of
taking in hundreds of distinct data types almost like magic. An excellent
example of this is this truncated example of `actix-web`:

```rust
async fn page(path: web::Path<(String, String)>) -> impl Responder {
    let (a, b) = path.into_inner();
    format!("You said {a} and {b}.")
}

// ...
App::new().route("/mypage/{a}/{b}", web::get().to(page))
// ...
```

When you compile your program, `actix-web` looks at the parameters specified in
your request handler function (`page`) and uses them to figure out how to
extract data from the web request and give them to you in whatever order or
format you specify. You can add more parameters to request handler to take out
different pieces of data and the web server will automatically give them to you.
All of this is powered by Rust's trait system: functions in Rust implement a
function trait, and the trait system is powerful enough to implement this sort
of API that's incredibly easy to use.

## Conclusion

Now that you know how to work with data in Rust, it's time to move onwards.
You'll have access to a series of different projects, which will each walk you
through different Rust concepts. These are real-world projects, and will only
interact with parts of Rust that are actually relevant to them. So, pick the
project that relates the most to what you want to use Rust for, or just go with
one that sounds interesting. If you want to learn more, you can always come back
and do another project (hint: click the text below to view a "map" of every
lesson to quickly navigate between them). Happy coding!
