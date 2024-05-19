# Mandelbrot Set Renderer

Rendering the Mandelbrot Set is one of my favorite projects because it's incredibly simple to create a simple renderer for it, but with enough effort you can improve it and create some astonishing results. Take a look!

$$IMAGE MandelbrotFull.png An image of the Mandelbrot Set that is colorful, complex, and visually interesting$$

By the end of this series of projects, we'll build something capable of creating images like the one above. But first, let's start small. In the first lesson, there was an example of a simple Mandelbrot Set renderer - if you haven't already, [check it out](https://cratecode.com/lesson/rust-a-language-youll-love/75m1jc9k0p/xa3l5ahj5w)! This is what we'll be building today:

$$IMAGE MandelbrotBasic.png An image of the Mandelbrot Set rendered as text with asterisks and spaces$$

## What is the Mandelbrot Set?

Before we can build the renderer, we first need to understand what the Mandelbrot Set actually is. The point of this lesson is to learn Rust, so we won't get too deep into it, but if you're interested, check out [this Wikipedia article](https://en.wikipedia.org/wiki/Mandelbrot_set) on it.

The Mandelbrot Set is a function that looks like this:

$$f\left(z\right)=z^{2}+c$$

At its core, it's actually a really simple function, which makes it all the more incredible that it can create images like the one above. Of course, there's still a few more details to work out - namely, how we turn the function into an image on the screen.

Let's start off with the two variables, `z` and `c`. Both of these are **complex numbers**.

### Complex Numbers

If you aren't familiar with them, that's alright! A complex number takes the form:

$$z=a+bi$$

Where `a` and `b` are real numbers (the kind that you're used to) and `i` is the number $$\sqrt{-1}$$. That's a little weird, but all you need to care about is how to program them in. When you want to do math on them (like adding and multiplying them), you can just think of `i` as a variable. So, to add two complex numbers together, you'll do:

$$z=a+bi$$
$$k=c+di$$
$$z+k=a+bi+c+di=\left(a+c\right)+\left(b+d\right)i$$

Multiplying is a bit similar, but you also need to use the fact that $$i^{2}=-1$$:

$$z\cdot k=\left(a+bi\right)\cdot\left(c+di\right)=ac+\left(ad+bc\right)i+bdi^{2}$$
$$z\cdot k=ac-bd+\left(ad+bc\right)i$$

If you're up for implementing these operations, I'd highly recommend it! However, if you don't want to, that's understandable as well. In that case, I'd recommend using the [num-complex](https://docs.rs/num-complex/latest/num_complex/) crate (which you can install by running `cargo add num-complex`), which can handle all of this math for you.

If you do choose to implement it, here are a few hints:
* ||Use a struct to hold your complex numbers. You can store a real and imaginary part.||
* ||You probably want to store your numbers as f32 or f64, so you can use decimals (f64 is more precise than f32).||
* ||You can look at the source code for num-complex for some ideas.||
* ||The first lesson has a full implementation of complex addition and multiplication.||

And if you're using `num-complex`, here are some hints for you as well:
* ||Use Complex32 or Complex64 for your numbers.||
* ||You can create a new complex number like Complex64::new(1.0, 1.0).||
* ||There are built-in functions for multiplying and adding complex numbers. For example, a.mul(b).||

Try implementing the function above. It should take in two values: `z` and `c`, and return
a new complex number.

Here are some hints for doing it:
* ||To square a number, multiply it by itself. z^2 is z*z.||

### Displaying the Mandelbrot Set

The Mandelbrot Set is the set (list) of all numbers where, for a number `c`, if we iterate the function on itself forever, starting with $$z=0$$, it doesn't go towards infinity.

In practical terms, this means that, for an input number `c`, if we set `z` to zero, then run a bunch of iterations (i.e., 100) that set `z` to `f(z,c)`, if our new value for `z` isn't big (i.e., smaller than `2`), that number is in the set. In pseudocode:

```javascript
let z = 0;
for (let i = 0; i < 100; i++) {
    // z = z^2 + c
    z = f(z, c);
}

if (z < 2) {
    print("In the set!");
} else {
    print("Not in the set.");
}
```

Note that, when dealing with complex numbers, if we want to check their size, we can't just use normal comparison operators, because they don't make sense for complex numbers. Instead, to check if they're super huge, we need to first get their absolute value (distance from zero), then check that. So `z < 2` above should really be `abs(z) < 2`.

And, that's it. Of course, it's a bit more tricky to actually put it onto the screen (and we'll go over that), but this is all you need to write for the Mandelbrot Set side of things. The way we display it is by treating `c` as an xy-coordinate ($$x+yi$$). If it's in the set, we print out an asterisk (`*`), and if it isn't, we print out a space (` `).

## Plotting

Knowing how to display the Mandelbrot Set is good, but we still need a way to make it work with our console. Right now, we have a function that can take a point in the xy-plane and tell us whether to put an asterisk or a space there. So, all that's left to do is figure out how to map the text in our console to it.

Luckily, this isn't too difficult to solve. Here's a simple approach:
* First, figure out your bounds in the xy-plane. I'd recommend going from `x = -1.5` to `x = 1.5`, and `y = 1` to `y = -1`.
* Next, figure out how many characters you're printing out. I'd recommend 90 characters wide and 24 characters down.
* Finally, start in the top-left corner `(-1.5, 1)` and write a loop (or maybe two) that can go to the bottom-right corner `(1.5, -1)`.

Here are some hints for using this approach:
* ||Your outer loop should go across the y-axis.||
* ||You can move by (y_final - y_initial) / height in each iteration of your outer loop.||
* ||You can move by (x_final - x_initial) / width in each iteration of your inner loop.||

Good luck!