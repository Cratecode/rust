# Async Rust

Let's talk about async code in Rust. This is (at least, in my opinion) one of
the most useful features that Rust has to offer, and it streamlines a lot of the
work needed for building many different sorts of applications. But async is also
one of the more confusing aspects of Rust. I don't say that to scare you, but to
temper your expectations, especially if you haven't encountered async in other
languages yet. It will be confusing at first, but once you start writing code
with it, you will start to understand it.

## What is async?

Async (short for asynchronous) is a way of writing code that optimizes waiting.
Many types of applications involve waiting for various things to happen, and
async is one way of dealing with that efficiently. But let's take a step back,
and look at a real-world example.

Cratecode has a program that powers the code running functionality. This program
listens for requests from your browser, which ask it to start up a machine to
run your code in. Before it can start it up, it needs to send out its own
request to a database, to get information about the lesson you're on. It also
handles other messages, like updating the machine every time you update a file.
Here's how that looks:

- First, we wait for a new request to come in. That happens when you first view
  this lesson page.
- Next, once we receive a response, we send a request to the database to grab
  the lesson details, then wait for a response.
- Once we receive a response, we start up a machine to run code on, then wait
  for it to be ready.
- Once it's ready, we start waiting for messages.
- Depending on the type of message, we'll send it to the machine, then wait for
  the response.
- Once we receive a response, we'll send it back to your browser, then wait for
  another message.

If we wrote this out with pseudocode, it would look like this:

```rust
// Wait for the browser to connect.
let conn = wait_for_connection();

// Send a request to the database and wait for
// the response.
let details = get_lesson_details();

// Start a machine to run code in and
// wait for it to be ready.
let machine = start_machine();

// Handle each message.
loop {
    // Wait for a message to come through.
    let msg = conn.wait_for_message();

    // Handle the message, potentially waiting
    // for a response to come in.
    let response = handle_message(msg, details, machine);

    // Send that response back to the browser.
    conn.send_response(response);
}
```

But wait, there are a few problems. First off, this server needs to be able to
handle multiple people, not just you. So, maybe we can try putting it into a
loop, so that it keeps picking up new connections:

```rust
loop {
    let conn = wait_for_connection();

    let details = get_lesson_details();

    let machine = start_machine();

    loop {
        let msg = conn.wait_for_message();

        let response = handle_message(msg, details, machine);

        conn.send_response(response);
    }
}
```

And that will almost work, but we can still handle one person at a time. We only
wait for another connection until we're done with the current one. So there's
still something wrong. We can use threads to do this, which let us run code
concurrently (i.e., without waiting for the first thing to finish before
starting the second), but they're only a half-solution. Maybe we want to handle
messages at the same time as well. We can add some more threads, but at some
point we'll have so many threads that it becomes hard to manage, and our threads
need to start waiting for other threads to stop before they can continue. And
threads aren't very efficient to begin with. The more we start depending on
them, the worse our program will perform.

## Using Async

A better approach is with async. Take a look at this code:

```rust
loop {
    let conn = wait_for_connection().await;

    task::spawn(async move {
        let details = get_lesson_details().await;

        let machine = start_machine().await;

        loop {
            let msg = conn.wait_for_message().await;

            let response = handle_message(msg, details, machine).await;

            conn.send_response(response);
        }
    });
}
```

I've made two changes: `.await` has been added to the end of some of the
function calls, and there's a `task::spawn`. Let's talk about that `task::spawn`
before we get to `await`.

This `task::spawn` creates something a lot like a thread for async code. The
code inside it will run independently of the outer code, so when we receive a
connection, we spawn a task to handle it, then we go right back to waiting for
the next connection. Meanwhile, the task that we started is also running and
doing its job at the same time as the connection loop.

Now, how about `await`? To explain what it does, we need to introduce `Futures`,
and talk a bit more about waiting. A `Future` is a trait in Rust that represents
some computation that will finish in the future. For example, when we ask our
database for data about the lesson, it won't give us a response immediately.
Instead, we get a `Future` containing our data, which means that the data isn't
available yet, but it will be in the future. So, how do we go to the future? By
waiting! And that's exactly what `await` does. It waits until a `Future` is
finished, and gives us the data.

But there's still something missing. Aside from the `task::spawn`, how is this
any different from our first pseudocode? Sure, it's using `await`, but all we've
really done is swapped out one form of waiting for another. But here's where it
gets interesting. Let's zoom out a bit on our code:

```rust
async fn listen_for_connections() {
    loop {
        let conn = wait_for_connection().await;

        task::spawn(async move {
            let details = get_lesson_details().await;

            let machine = start_machine().await;

            loop {
                let msg = conn.wait_for_message().await;

                let response = handle_message(msg, details, machine).await;

                conn.send_response(response);
            }
        });
    }
}
```

Pay special attention to where it says `async fn`. This is called an async
function, and it returns a `Future` containing any data that the function
returns (in this case, no data). And this `Future` is finished when the function
returns. This is where the waiting happens: when you use `await`, it causes the
`Future` returned by the function to pause until the `Future` that's being
awaited is done.

Another way of creating `Future`s is with the `async { ... }` syntax, as seen in
`task::spawn`. The `move` keyword here just means that local variables are moved
into the `Future`. In fact, `async` functions can be thought of as syntax sugar
for this syntax:

```rust
async fn test(to_add: i32) -> i32 {
    let val = get_val().await;
    val + to_add
}

// Is similar to writing:

fn test(to_add: i32) -> impl Future<Output=i32> {
    // This async { ... } creates a Future that,
    // when awaited, will produce the data.
    async move {
        let val = get_val().await;
        val + to_add
    }
}

// Notice that we don't have to write impl Future<...> for the async fn.
// That's because it's inserted automatically.
```

**One of the most important things to understand** is that async functions
produce `Futures`, but they usually don't perform any real work. To get the work
done, you either have to `await` the `Future`, or pass it to something that will
run it for you (for example, `task::spawn`, which we'll get to later). If your
code doesn't seem to be doing anything, make sure that you're using the
`Futures` returned to you, since just calling an async function isn't enough.

## Async Runtimes

One of the neat things about Rust async is that it doesn't enforce a specific
runtime. What that means is that `async`, `await`, and `Future` provide the
building blocks needed for async, but they don't form the full picture. You also
need a runtime, which is a piece of code responsible for actually executing your
`Futures`. The one we're going to look at is called [Tokio](https://tokio.rs/),
but there are all sorts of other runtimes that are better suited for specific
purposes. For example, [smol](https://github.com/smol-rs/smol) is a lightweight
async runtime, and [Embassy](https://embassy.dev/) is a runtime intended for use
on embedded devices.

For our purposes, Tokio is an essential library for async code, and it provides
many tools and APIs for working in the async world. To use it, add it as a
dependency (`cargo add tokio --features all`), making sure that you have the
`all` feature selected (unless you know exactly what you'll be using). Then, in
your `main.rs`, your main function should look like this:

```rust
#[tokio::main]
async fn main() {
    // Write code here!
}
```

Now, you can write async code inside your main function. You'll find that a lot
of libraries in the Rust ecosystem make use of async functionality, and you'll
need to have an async runtime (usually Tokio) set up to use them. One example is
the [reqwest](https://docs.rs/reqwest/latest/reqwest/) library, which is used to
make HTTP requests. For example, you could use it to retrieve Cratecode's
sitemap (which is a list of pages that exist on a website):

```rust
#[tokio::main]
async fn main() {
    let req = match reqwest::get("https://cratecode.com/sitemap.xml").await {
        Ok(req) => req,
        Err(err) => {
            eprintln!("An error occurred while sending the request: {err:?}");
            return;
        }
    };

    let body = match req.text().await {
        Ok(body) => body,
        Err(err) => {
            eprintln!("An error occurred while reading the response: {err:?}");
            return;
        }
    };

    println!("{body}");
}
```

(take a look at the documentation for reqwest if you want to learn more about
making HTTP requests)

## Tokio Usage Patterns

Here are a few common patterns that you'll encounter when writing async code
with Tokio.

### Tasks

Imagine we have some sort of web server, and we want to log how many requests we
receive. One way we can do this by keeping count in a database. But sending a
request to a database takes time, and if we `await` the database query before
sending a response to the client, then our web server will be needlessly slower
(because it now has to wait for the database). What we really want is to update
the database in the background, and to do that we might be tempted to write code
like this:

```rust
async fn update_database() {
    // ...
}

async fn handle_request() {
    // ...

    // Update the database in the background.
    update_database();

    // ...
}
```

But this won't work, because `update_database()` returns a `Future`, and
`Future`s **do not do anything unless they are used!** If we want to run a
`Future` in the background, we can use tasks:

```rust
// Tasks are a feature provided by Tokio,
// although other async runtimes provide very
// similar interfaces.
use tokio::task;

async fn update_database() {
    // ...
}

async fn handle_request() {
    // ...

    // Update the database in the background.
    task::spawn(update_database());

    // ...
}
```

Tokio's [task::spawn](https://docs.rs/tokio/latest/tokio/task/fn.spawn.html)
function takes in a `Future`, which can include the return value of an `async`
function, or an `async` block. There are some great examples on the
documentation, so give it a good look through!

You can also `await` the value returned by `task::spawn` if you want to

### Join

Another common pattern is being able to wait for multiple `Future`s to complete.
For example, imagine we need to query two different types of data from our
database. One way we can do it is like this:

```rust
async fn run_code() {
    let data1 = query1().await;
    let data2 = query2().await;
}
```

However, this means that we won't begin the second query until the first one is
finished. Doesn't that seem inefficient? A better solution would be to send both
queries to the database at the same time, then wait until both of them are
finished. Tokio provides a macro called
[join](https://docs.rs/tokio/latest/tokio/macro.join.html), which we can use to
do this:

```rust
async fn run_code() {
    let (data1, data2) = tokio::join!(query1(), query2());
}
```

Notice how there aren't any `await`s here — that's because the `join` macro does
it for you automatically.

If you need to join together a list of `Future`s, you can use
[join_all](https://docs.rs/futures/latest/futures/future/fn.join_all.html)
(provided by the `futures` crate) or
[JoinSet](https://docs.rs/tokio/latest/tokio/task/struct.JoinSet.html) (from
Tokio), among other options.

### Select

Sometimes, you need to race two `Future`s and handle whichever one completed
first. One example is in the message loop for a server — you might want to
receive messages from multiple different sources, and handle them when they come
in:

```rust
/// This code reads in messages from multiple sources and handles
/// each one appropriately.
async fn input_loop() {
    loop {
        if msg from connection A {
            // Handle message.
        } else if msg from connection B {
            // Handle message.
        }
    }
}
```

We can't express this with `await`, since we don't know which connection will
have a message coming in first, and we can't use `join`, because we want to
handle each message as it comes in, not wait for one message from connection A
and another from connection B. Instead, we can make use of another one of
Tokio's macros: [select](https://tokio.rs/tokio/tutorial/select).

This macro will take in multiple `Future`s and run some code for the first one
that finishes. It looks a lot like a `match` statement. Here's how we can adapt
the example above:

```rust
async fn input_loop() {
    loop {
        tokio::select! {
            msg = get_msg_from_a() => {
                // Handle message.
            }
            msg = get_msg_from_b() => {
                // Handle message.
            }
        }
    }
}
```

Each arm of the `select` macro is formatted as `data = future => { code }`,
where `data` is the name of the variable to store the value returned by the
`Future`, `future` is the `Future` to run (in this case, imagine
`get_msg_from_a` is an `async fn`), and `code` is the code that will be executed
if that arm finishes first.

### Blocking

Code that might take a long time to execute is called blocking, and should
generally be avoided when using async. The reason is that it can slow down other
parts of the system, which might show up as unexpected and unwanted latency.
Blocking code usually looks like a heavy computation or reading from the
filesystem, but it can really encompass any piece of code that takes a long time
to execute.

The first way to avoid blocking is to see if there's an async alternative to
what you're doing. For example, instead of using the built-in filesystem APIs
built-in to Rust, you can use
[the ones provided by Tokio instead](https://docs.rs/tokio/latest/tokio/fs/).
For other cases, you can make use of the
[spawn_blocking](https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html)
API provided by Tokio. For example:

```rust
use tokio::task;

/// Computes the nth factorial, returning a [Future]
/// that resolves to the computed value, once complete,
/// or None if it failed.
async fn factorial_async(num: u32) -> Option<u32> {
    task::spawn_blocking(move || {
        let mut acc = 1;
        for i in 0..=num {
            acc *= i;
        }

        acc
    }).await.ok()
}
```

If your program involves a lot of heavy computations, you may be better off
looking at alternative solutions. Check out
[this part of the Tokio documentation](https://docs.rs/tokio/latest/tokio/index.html#cpu-bound-tasks-and-blocking-code)
for more information.

## Conclusion

Async code is a large part of the Rust ecosystem, and it's used for many
different types of applications, most prominently those that deal with
networking. If you haven't encountered async before, it will take some time
getting used to, but once you have a few projects under your belt, you will
understand it. This article is intended to give you a high-level overview of how
async works, and there are some great resources to help you dive further in.
Check out the [Rust Async Book](https://rust-lang.github.io/async-book/) and the
[Tokio Tutorial](https://tokio.rs/tokio/tutorial).

You can also check out some projects to guide you through async.
[Click here to check out the chat app backend project](https://cratecode.com/lesson/rust-chat-app-backend/k9lvm27fq1/xa3l5ahj5w).
Happy coding!
