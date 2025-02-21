# Chat App Backend

For those interested in using Rust for web or application development, this
article covers building a simple real-time messaging application. The frontend
for it has already been written, and you will be responsible for configuring
your web server to serve it, as well as writing the code to enable the messaging
functionality to work.

First, let's cover how a messaging app works. This one uses WebSockets to enable
communication between our frontend and backend. If you aren't familiar with
WebSockets, they're a technology that allows us to create a communication
channel between a client and a server. This lets the frontend code (the client
running on the user's computer) send and receive messages from the backend code
(the server running on your computer).

So, when we type a message into our chat app and press "Send", a WebSocket
message is sent up to the server containing that message. Then, the server can
relay that message over to every other client (user) connected to it. It will
also store the message somewhere, so that when new clients connect to it, it can
send them the entire message history so that they're caught up to speed.

## High-level Design

Right now, there's a `public` directory containing a self-contained `index.html`
file. This file hosts the frontend for the messaging application. You should
configure the web server so that accessing `/` points to `public/index.html`.
You can do that explicitly, or you can point the server to the `public`
directory, and it can handle managing which routes map to which files. That way,
you can add other files to the `public` directory later on, without needing to
modify the code.

If your website is `example.com`, this means that accessing
`https://example.com` should have the web server respond with
`public/index.html`. Since this file contains the frontend for the application,
receiving it will tell the browser how to display the interface and how to
communicate with the backend code that you'll be building.

Then, you should handle WebSocket connections at `/socket` (this is just what
the frontend code is configured to connect to, but you can modify it to anything
you want so long as the frontend and backend code agrees).

Upon a client connecting, you should send them every message that has been sent
so far. This will be done by sending one WebSocket message (as a string) per
chat message in the history. For example:

Message 1:

```json
"User: Hello!"
```

Message 2:

```json
"User: Is anyone there?"
```

Message 3:

```json
"Other User: Hi!"
```

Again, this is just the format that the frontend code (in `public/index.html`)
has been written to accept. You can use any format you want, so long as you also
modify the frontend code to work with it. For example, to keep things simple,
the user is included directly in the message instead of being broken out into
"user" and "message" fields. If you want, you can add this functionality, but
the frontend code has to be modified to work with it.

Next, when the user types in a message and presses the "Send" button, their
message (appended to their name) will be send to the backend server as a string.
For example:

```json
"User: Nice to meet you!"
```

And when the backend code receives this message, it should relay it to every
client (including the one who sent it). It should send it as a string (the same
format that it received it as). For example:

```json
"User: Nice to meet you!"
```

## Using axum

### Initial Setup

axum is a library used for building web applications in Rust. There are many
libraries like it out there, so feel free to use whichever one you want. This
section will cover how to set up and use axum, and you can find more information
about it (or other frameworks) by looking at their documentation.

First, to install axum, run `cargo add axum --features ws` in your console (the
`ws` is to enable axum's WebSocket support). This will add it to your
`Cargo.toml` and let you use it in your code. You also need to install tokio,
which is an async runtime for Rust. We won't go too far into async/await in this
lesson, but we will briefly touch on it and it is required for axum to function.
You can install tokio with this command: `cargo add tokio --features full` (the
`full` here gives you access to the full functionality of tokio). Finally, we'll
need some features from the tower_http library — in particular, it can help us
serve files in the `public` directory. Install it with
`cargo add tower_http --features fs` (the `fs` here enables filesystem-related
features that we need).

With all these dependencies, now's a good time to mention that you can run
`cargo check` in the console to check for errors, instead of using `cargo run`
or `cargo build`. The difference between the two is that `cargo check` just
looks for errors, whereas `cargo run` or `cargo build` try to compile your
program. As a result, `cargo check` is much, much faster, and is the recommended
way to quickly check if your code is valid.

Next, head over to [the documentation page](https://docs.rs/axum/latest/axum/)
for it. I'm going to include some code samples here, but at some point they will
become out-of-date. I would recommend using them for high-level concepts and to
look at the documentation for the actual code to use.

First, set up your main function like this (if there's already an existing one,
you'll need to remove it or modify it to match):

```rust
#[tokio::main]
async fn main() {

}
```

This will let you run async code using tokio (which is required to start the
axum web server). Next, you'll need to create a new `Router`. This tells axum
how to handle different paths. For example, on the `/` path, we want to serve
`public/index.html`, and on the `/socket` path, we want to handle the WebSocket.
The `Router` contains that behavior.

### Configuring the Router

First, create a new `Router` in your `main` function like this:

```rust
let app = Router::new();
```

You'll need to import `Router` from axum by putting a line like this at the top
of your file:

```rust
use axum::Router;
```

You can also run `cargo check`, and Rust will give you suggestions for what to
import.

Next, we'll handle serving the static frontend. This is based on
[this axum example](https://github.com/tokio-rs/axum/blob/main/examples/static-file-server/src/main.rs),
which might be more up-to-date than the code I'm going to show you. Next, modify
your router so that it looks like this:

```rust
let app = Router::new()
          .fallback_service(ServeDir::new("public"));
```

You'll need to import some more things. Run `cargo check` to figure out how to
do this. Once you import everything, you'll probably get an error about type
annotations. Don't worry! We'll fix that soon. This line above tells axum to use
the service returned by `ServeDir::new("public")` to handle anything that hasn't
already been handled. In other words, tower_http provides a `ServeDir` service
that contains all the code needed to read from a directory, and will try to
serve any files in it. First, axum will try the other routes (right now there
aren't any), and if it can't find any matches, it'll look in the `public`
directory.

Now, let's add the finishing touches to make everything work. Add the following
lines after your router:

```rust
let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
axum::serve(listener, app).await.unwrap();
```

Make sure you import the `TcpListener` from tokio, not std!

The first line creates a listener on port `3000`, which is the port where the
content will be served on. This means that you can access it at
`http://localhost:3000`. This is also the port that Cratecode uses to serve
content on the web view tab, which will let you see it on this page (the one
you're reading this on).

Also, notice the `.await` on both of these lines. This means that the program
will wait for the operation to finish until continuing. I won't delve into
async/await right now, but just know that, whenever a function returns a
`Future` type or is itself an `async` function, you can `.await` it to retrieve
the value (so long as you're inside an `async` function).

Now's a good time to test it out and make sure it works. Try running the
program, then open the web view tab (it's right above the console) and see if it
works. If all's well, you should see the interface for a chat application. You
can try interacting with it, but it won't work without the backend code.

## Handling WebSocket Connections

### Configuring Server State

First, we need to store some global state that we can access across every
websocket connection. One thing we'll need is a list of messages to send down to
every client when they first connect. The second thing we'll need is a channel
that we can use to broadcast messages with.

A channel is something we can use to send and receive messages with between two
tasks. In this case, we'll use something called a broadcast channel. The
broadcast channel will let us send messages through it, and listen for any
messages that have been sent. Every message sent through the broadcast channel
will be sent to every task receiving messages through it. There are other types
of channels for different purposes, but this one is the best when we want to
send a message to every listener (broadcast it).

Then, to implement the chat WebSocket, we'll send messages through the broadcast
channel when we receive them from the client, and send messages to the client
when we receive them from the broadcast channel. The broadcast channel basically
serves as the way that the code handling one client on our backend can
communicate with the code handling the other clients on the backend.

Finally, we'll also store the message in our message history list. Because of
the way axum works, we can't just store a `Vec<String>`. Instead, we'll use an
`Arc<Mutex<Vec<String>>>`. That's an Arc storing a Mutex storing a Vec storing
some Strings. Whew! That's a big type. You don't have to worry too much about
what it really means — we'll cover that in a later lesson — but know that it's
what lets you share this list globally across every client. The `Arc` lets us
store global read-only data, and the `Mutex` lets modify that data.

Now, let's put this together into a `ServerState` struct at the top of the
program:

```rust
#[derive(Clone)]
struct ServerState {
    /// The list of every message that this server has received.
    message_history: Arc<Mutex<Vec<String>>>,
    /// A channel to share messages between all connected clients.
    message_channel: broadcast::Sender<String>,
}
```

When importing, make sure you get `Arc` and `Mutex` from std, and `broadcast`
from tokio!

The `#[derive(Clone)]` at the top tells Rust to automatically make this struct
cloneable (which means we can duplicate it). The reason is that axum actually
creates a copy of the server state every time it's needed. This means that if
you tried to modify the server state, since it's just a copy, those changes
wouldn't be reflected to other requests. That's actually why we need to use
`Arc<Mutex<...>>` here. It's what allows us to modify and share our state across
different clients.

The way that it works is a bit complicated, but when we clone an `Arc`, it
creates a new `Arc` that points to the same piece of data. So, when axum clones
our `ServerState`, we get two distinct `ServerState` values which point to the
same list.

Next up is the `broadcast::Sender<String>`. This type can be used to send data
(in this case, a `String`) into our broadcast channel. Just like `Arc`, when we
clone it, we get a new `Sender` that points to the same channel. You might be
wondering how we're able to receive data from the channel if all we have is a
`Sender`. For broadcast channels, we can create a `Receiver` using a `Sender`
(which we'll get to once we start implementing the logic for it).

Now, to make use of it, we have to add a few more lines into our `main`
function. First, we'll create the history and channel like this:

```rust
let message_history = Arc::new(Mutex::new(Vec::new()));
let (tx, _rx) = broadcast::channel(32);
```

The `broadcast::channel(32)` part creates a channel that can hold 32 messages
before it starts to overflow. This means that the channel can hold 32 messages
that haven't been seen by every receiver yet, not that only 32 messages can only
be sent through it. Unless there are some bugs in our code, or we receive an
extreme number of messages, we should be able to handle an endless number of
messages.

The `(tx, _rx)` part is a tuple destructuring. The `channel` function returns a
`Sender` and a `Receiver`, and we can extract them using that syntax. We won't
be using the receiver, so `_rx` is underscored to represent that it's unused.

After this, we need to create the server state. We'll do that like this:

```rust
let state = ServerState {
    // This is syntax sugar for message_history: message_history.
    message_history,
    message_channel: tx
};
```

Finally, we need to give our state to axum. We can do this by adding
`.with_state(state)` to our router, like this:

```rust
let app = Router::new()
       // ...
          .with_state(state);
```

And now, we should be able to access this state across all our requests. Now's a
good time to make sure that everything still works. Run `cargo check` to make
sure that there aren't any errors, then run the program to ensure that it still
displays the frontend page.

### Receiving Connections

This section is based on
[this WebSocket example](https://github.com/tokio-rs/axum/blob/main/examples/websockets/src/main.rs).
This example should be up to date if these code examples aren't.

First, create these two functions in your file:

```rust
async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<ServerState>,
) -> impl IntoResponse {

}

async fn handle_socket(
    mut socket: WebSocket,
    state: ServerState
) {

}
```

The first function is going to be responsible for handling connections to
`/socket`. All of its parameters are given to us by axum. The first parameter,
`ws: WebSocketUpgrade`, is a special tool for converting regular HTTP into
WebSocket connections. When a browser connects, it starts off the communication
as HTTP (or HTTPS), and then that gets "upgraded" into a WebSocket connection.

The next parameter is `State(state): State<ServerState>`. This is our server
state, and axum has a special type called `State` that holds our server state.
The `State(state)` part of is called struct destructing. It's just like the
tuple destructuring we saw for the channel, except it operates on structs. In
this case, `State` is actually a tuple struct, so the `State(state)` means
"extract the first (and only) item from the State struct and place it into a
variable called state". The type (`State<ServerState>`) refers to the whole
expression, so the type of the `state` variable will be `ServerState`.

Finally, `impl IntoResponse` means that this function should return some sort of
HTTP response. In this case, we'll use the `WebSocketUpgrade` to generate this
response, and it'll tell the web browser to switch to WebSocket.

Next up is the `handle_socket` function. This is one that we'll call ourselves,
and will be responsible for managing the WebSocket. This means that it'll
receive data for it (and handle all the logic needed for that), and it'll also
deal with relaying any data from the channel to the WebSocket.

Now, let's add some code. The first thing we'll do is to add this line to
`ws_handler`'s body:

```rust
ws.on_upgrade(move |socket| handle_socket(socket, state))
```

This will tell the `WebSocketUpgrade` to upgrade the connection, and, when it
succeeds, run `handle_socket` with the new socket. It will also return the
response that instructs the browser to upgrade.

After that, we'll add the following line to our route handler (before the
`with_state` line):

```rust
.route("/socket", any(ws_handler))
```

This tells axum that, when the client goes to `/socket`, it should run
`ws_handler`. In turn, this will cause `handle_socket` to run. You can try
adding a `println!` inside of `handle_socket`, then restarting the server and
reloading the frontend web view. If all goes well, you should see a message
being printed out as the frontend connects to the WebSocket server.

At this point, we have a working web server. All we have to do now is implement
the logic.

## Chat Message Logic

Now, I'm going to add some comments in to the `handle_socket` function, but your
job will be to figure out how to implement it.

```rust
async fn handle_socket(
    mut socket: WebSocket,
    state: ServerState
) {
    // First, grab every message from state.message_history
    // and send it to socket.

    // Next, create a receiver from state.message_channel, and
    // continuously receive messages from it.
    // When you receive a message, send it to the socket.

    // At the same time, listen to messages being sent from
    // the socket.
    // When you receive one, place it into the broadcast sender
    // in state.message_channel as well as state.message_history.
}
```

At this point, I would recommend you head over to the next lesson about async
Rust, then come back here to finish the project. You can still give it a try
right now, but there is some information contained in the next lesson that will
be useful to you here.

Here's some important information to get you started:

- To read/write data from/to `state.message_history`, do
  `state.message_history.lock().unwrap()`. For example, you can do
  `state.message_history.lock().unwrap().push("My message".to_string())`.
- Ensure that you aren't holding a lock across an await point or
  `tokio::select`. Doing so will cause issues that are hard to debug. Run
  `cargo clippy` to detect this (as well as other types of issues). You may have
  to clone your messages Vec to avoid this.
- If you're getting error messages about `Send` bounds, it most likely means
  that you're holding a lock across an await point. To fix this, you can try
  introducing a variable, or wrapping the code with the lock inside it with a
  scope (`{}`).
- You should have an infinite `loop` that your message receiving code goes
  through. Then, in each iteration of the loop, try to receive a message from
  the WebSocket or the broadcast channel. If the WebSocket returns `None`, then
  it's closed, so you should `break` out of the loop. If the broadcast channel
  returns `None`, then something's gone wrong and there are no more senders.
  Similarly, if you receive an error here, either the WebSocket has closed or
  something has gone wrong. In both cases, you should break out of the loop and
  stop handling the WebSocket.
- You will want to use `tokio::select` in order to receive messages from the
  WebSocket and from the broadcast channel at the same time. You can view the
  documentation for it [here](https://tokio.rs/tokio/tutorial/select).
- Take a look at the documentation for
  [broadcast channels](https://docs.rs/tokio/latest/tokio/sync/broadcast/fn.channel.html).
  If you run `state.message_channel.subscribe()`, you will receive a `Receiver`
  that you can receive messages from.
- Also look at the documentation for
  [axum websockets](https://docs.rs/axum/latest/axum/extract/ws/index.html), and
  in particular, the
  [WebSocket struct](https://docs.rs/axum/latest/axum/extract/ws/struct.WebSocket.html)
  that you have access to.

This project involves a lot of concepts that are probably new to you, and you
probably will get stuck somewhere along the way. That's okay! Try to get as far
as you can, and consult resources when you get stuck. Once you've given it a
good try, you can compare your code to the example below.

Good luck! If you get stuck along the way, look online for other resources or
examples. The axum repository has some great examples showcasing different
features, not just WebSockets.

## Example Code

Here's an example implementation of this project. You can use it as a reference
for your own code, but please give the project a try before consulting the
example! The best way to learn is by spending some time struggling with the
problem.

```rust
use axum::extract::{
    ws::{Message, WebSocket},
    State, WebSocketUpgrade,
};
use axum::response::IntoResponse;
use axum::routing::any;
use axum::Router;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tower_http::services::ServeDir;

#[derive(Clone)]
struct ServerState {
    /// The list of every message that this server has received.
    message_history: Arc<Mutex<Vec<String>>>,
    /// A channel to share messages between all connected clients.
    message_channel: broadcast::Sender<String>,
}

#[tokio::main]
async fn main() {
    let message_history = Arc::new(Mutex::new(Vec::new()));
    let (tx, _rx) = broadcast::channel(32);

    let state = ServerState {
        // This is syntax sugar for message_history: message_history.
        message_history,
        message_channel: tx,
    };

    let app = Router::new()
        .route("/socket", any(ws_handler))
        .fallback_service(ServeDir::new("public"))
        .with_state(state);

    let listener = TcpListener::bind("localhost:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<ServerState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: ServerState) {
    // Send over every message to the client.
    let messages = state.message_history.lock().unwrap().clone();

    for msg in messages {
        if let Err(err) = socket.send(Message::text(msg)).await {
            eprintln!("Error while sending initial data: {err:?}");
            return;
        }
    }

    let mut recv = state.message_channel.subscribe();

    // Receive loop.
    loop {
        tokio::select! {
            val = recv.recv() => {
                let Ok(val) = val else {
                    break;
                };

                if let Err(err) = socket.send(Message::text(val)).await {
                    eprintln!("Error while sending message to socket: {err:?}");
                    return;
                }
            }
            val = socket.recv() => {
                let Some(Ok(val)) = val else {
                    break;
                };

                // Only handle text messages.
                if let Message::Text(text) = val {
                    // Send message to all connected clients.
                    if state.message_channel.send(text.to_string()).is_err() {
                        break;
                    }

                    // Save message to history.
                    state.message_history.lock().unwrap().push(text.to_string());
                }
            }
        }
    }
}
```
