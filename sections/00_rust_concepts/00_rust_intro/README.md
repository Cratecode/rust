# Rust - A Language You'll Love

Rust is a language that makes it harder to write buggy software. It's also a language with automatic memory management without a garbage collector, which makes it incredibly fast (comparable to C and C++). That's a really nice bonus, but with how fast computers and programming languages are today, you'll hardly notice a difference unless you're dealing with a ton of data. Instead of focusing on how fast Rust is, I'd like to set our sights on one of Rust's other big features: code written in Rust is far less prone to errors than code written in many other languages.

Before we go any further, it's worth mentioning that if you're new to programming, Rust may not be for you (at least for right now). That's because Rust has a lot of rules and new concepts, as well as a ton of advanced features, which might be difficult to learn on top of some of the core programming concepts. If that's fine by you, keep reading on ahead! Otherwise, check out the [intro to programming](https://cratecode.com/unit/qfrcfkaw33) course, then come back here.

Before we get into some of the specifics behind writing Rust, it's important to understand what big features the language has. Rust is like a mix between a "low-level" language (like C or C++) and a "high-level" language (like C# or JavaScript). A lot of the features provided by high-level languages (such as object-oriented programming, automatic memory management—if you've written code in a high-level language and never had to think about allocating and freeing memory, that's what this feature is, etc.) are done so in a way that has runtime overhead. That means that you get these features, but they come at a performance cost. Rust, on the other hand, prefers "zero-cost abstractions", which basically means that you get these nice features (abstractions), but without a performance cost (zero-cost).

Rust is also a language that was created more recently. As such, a lot of its design decisions are based on the mistakes made by other languages. Rust doesn't have `null` or `throw` (at least in the conventional sense, there are better alternatives that it uses), so you'll rarely get an unexpected, program-crashing error. It also has a large ecosystem that's part of the language itself. When you install Rust, you're also installing a package manager ([Cargo](https://crates.io/)), a linter ([Clippy](https://github.com/rust-lang/rust-clippy)), and a code formatter ([rustfmt](https://github.com/rust-lang/rustfmt)). You can use cargo to manage packages (`cargo add package-name`), and you can run Clippy with `cargo clippy` and rustfmt with `cargo fmt`. This is not common. In most languages, linters (which are just programs that find mistakes in your code) and formatters (which make your code look nicer) are built by the community, so you need to deliberately install and configure them, as well as choose from one of the several such programs available. In other languages, there isn't even a package manager (which is a program that lets you seamlessly use other people's code in your own projects). By having these things installed by default, Rust code is compact and efficient (you can easily use others' code instead of reinventing the wheel), even less error-prone (Rust being less error-prone is because of the language, not the linter, but having a linter helps remove even more bugs), and follows a consistent format (formatted code is easier for people to read and work with, and a formatter means you don't need to think about code style—the computer will do it for you).

These are all awesome features, and things you won't really find in many other languages, but what really takes the cake is Rust's type system and borrow checker. If you've ever worked with a statically typed language (Java, TypeScript, etc.), you might already have an idea behind how types work. Basically, having types (in a statically typed language) means that you always know what kind of data something is, and lets you constrain what sort of data variables/parameters can be. For example, a function that multiplies two values should probably only accept numbers because multiplication doesn't really make sense for anything else. Rust has a much more powerful type system than many other languages. Explaining what this actually means is a little tricky without actually looking at and writing Rust code, but one way this power is visible is that Rust can have types that force you to do something. For example, instead of having `null`, Rust has a type that basically says "there may or may not be a value here". If you see that type, you must make sure there's a value (or explicitly say that you're fine with the program crashing if there isn't one), or else Rust will fail to compile if you don't. The nice thing about this is that you can't be tricked into being given a "null" value. There aren't really `NullReferenceExceptions` in Rust because you will always know if something can be "null", and will need to handle it in some way. This is a great feature (and is also available in languages like Kotlin), but the real significance is that it isn't a magic feature provided by the language. You can make your own version of it, or create something similar that fits your needs. This is pretty vague, but just know that the Rust type system gives you access to wonderful features that aren't really possible in other languages.

The other important concept to know about is Rust's borrow checker. Basically, the borrow checker is a set of rules that ensure your program is "safe", and it's what lets Rust have automatic memory management without a garbage collector (which is just a thing that cleans up memory when it isn't needed—we'll get more into this later). There are a lot of things you can do with memory, but a lot of them will just cause your program to crash, or create very strange and hard-to-debug bugs. There's a formal definition for it, but this is basically what "undefined behavior" means. When writing code in C or C++, this is something that you need to take into account, and is the cause of many bugs (for a good example, look up "heartbleed"). Rust's borrow checker ensures that your code is safe (i.e., doesn't have undefined behavior), and will refuse to compile your program (which is good, because if it did compile it, you'd probably end up with a random, hard-to-debug error when your program runs instead of a nice message that tells you exactly what you did wrong). Note that the borrow checker isn't perfect, and will sometimes give errors for code that's perfectly fine, but Rust provides a way to get around this (which you'll probably never need to use, because it's rarely needed, and most of the simple cases where it is needed have been put into libraries that you can use without ever needing to think about it). But that's not what makes the borrow checker special. Language can also prevent undefined behavior with runtime checks and garbage collectors (which works very well, but at a performance cost). Where Rust's borrow checker really shines is with a concept called fearless concurrency.

Concurrency means that you can split your program up into different tasks (or threads) that can be run independently of one another (i.e., without waiting for one to finish before starting another one). These tasks may also run simultaneously, because your computer can execute multiple pieces of code at the same time - which can speed up programs (if your computer can execute 4 things at once, then you're able to do 4 times the work) and make certain types of programs (like web servers) more efficient. But there's a cost to concurrency. Imagine if you have a simple program that counts up to some large number. It might look something like this pseudocode:
```js
let counter = 0;

function count() {
    for(let i = 0; i < 1_000_000; i++) {
        counter++;
    }
}

count();
count();
count();

// Now, counter should equal 3_000_000.
```

If you want to make this program count even faster, you might make it multithreaded, which means that parts of the program run simultaneously. In that case, it might look something like this pseudocode:

```js
// This counter variable is shared between all the
// threads, so they'll all add to it.
// We'd expect them to count to 3 million, but as we'll see,
// that's not really what happens.
let counter = 0;

function count() {
    for(let i = 0; i < 1_000_000; i++) {
        counter++;
    }
}

// We're running all three of these
// loops at the same time.
multithread(count); // thread 1
multithread(count); // thread 2
multithread(count); // thread 3
```

Now, we have three different threads simultaneously incrementing `counter`. Great, right? That should make it work 3 times faster. Except, that `counter++` code actually compiles to something like this:
```js
const oldCounter = counter;
counter = oldCounter + 1;
```

In other words, it stores the value of counter, adds 1 to it, and then sets counter to the new value. What appears to be a single operation in our code turns out to be multiple, and that can spell trouble when dealing with concurrency.

Because things are running simultaneously, we might end up with this case (the comments show which thread is running it):
```js
// In this example, oldCounter is unique per-thread,
// but counter exists across all of them.
//
// If you're familiar with object-oriented languages, you
// can think of oldCounter sort of like an instance variable,
// and counter sort of like a static variable.
const oldCounter = counter; // thread 1
const oldCounter = counter; // thread 2

// At this point, both threads have stored the old value
// of counter, and haven't modified counter yet.

counter = oldCounter + 1; // thread 1
counter = oldCounter + 1; // thread 2

// Because both threads saved the old version of counter,
// those two lines effectively do the same thing twice.
// Imagine before this that counter = 123.
// Then, both threads save 123 into their oldCounter variables,
// and then both threads set counter to 124 (123 + 1).
// So, even though we did counter++ twice, it only ended up adding
// 1 to the counter instead of 2.
```

In this case, while we have two threads simultaneously updating it, the counter only ends up updating by 1. This is because both threads get the old value, then update it to that old value + 1. So, they both do the same thing. In fact, this is just one of the many ways that these threads can interact. If we run this code, we'd expect it to count to 3,000,000. I ran it a couple of times, and I got these values: `1617080`, `1706661`, `1541751`, and `1863179`. This is bad. Not only is the code not getting the same answer, it gives an answer that's basically random each time. This type of error is called a race condition. But it turns out that this is the least of our worries. If we look at what `counter++` actually does in the computer, we'll find more and more of these bugs as things get more and more complicated. Eventually, we'll see that some threads try to update the value at the same time, which can be even worse and will probably cause memory issues (leading to undefined behavior). This is called a data race. Luckily for us, the borrow checker's rules, along with a few extra features added to Rust, solve this completely. If our code has a data race, it won't compile, so we don't really need to think about them. We can still create race conditions, and absolutely need to think about them when writing code, but the way Rust is written will still nudge us towards not having them.

One final Rust feature to talk about is its compiler errors. Rust is a strict language, which helps reduce the number of bugs in our code, but also leads to there being much more compiler errors than in other languages. Luckily, Rust error messages are extremely helpful. They tell us exactly where and what the issue is, give examples explaining the error and how to fix it, and sometimes even suggest how to fix the error, right in the message. Rust also has a ton of warnings that tell you that your code is valid and will compile, but might have some pieces that should be changed. And on top of that, Clippy (Rust's linter) provides a ton of other errors and warnings, which are just as helpful and show you where even more issues in your code might be.

That should give you a good introduction to what Rust is about. Rust is a language with a ton of advanced features that make it fun and easier to write in. Getting a Rust program to compile is often more difficult than with other languages, but once it does compile, you'll be assured that your program is mostly bug-free. In the next few lessons, we'll dive deeper into some of Rust's concepts, as well as actually taking a look at Rust code. Happy coding!