# Smiley Face

Now then, let's get you writing some code! We'll be making a smiley face generator, capable of outputting smiles such as:
```
  **  **  
  **  **  

** *   ** 
  * ***  *
```

And:
```
  **  **  
  **  **  

* ***     
 *   **  *
       ** 
```

And even:
```
  **  **  
  **  **  

*         
 *       *
  * **  * 
   *  **  
```

It does this using a really simple algorithm. First, your starting height is 0. Then, for each vertical column (in this case, there are 10), decide to move up, down, or stay where you are. After that, draw a "*" and move to the right.

First thing's first, we're going to use a random number generator. There's a Rust library for that called [rand](https://docs.rs/rand/latest/rand/), which you can install by running `cargo add rand` in your terminal. Do that now, then take a look through the documentation to learn how to use it.

Second, you're going to need to have a list of strings for each row, and slowly build them up. You won't be able to print anything out until the very end. Hint: the easiest way to do this is to start with a blank grid (filled with spaces), then replace a character with a "*" when drawing. Finally, print out the entire grid at the end.

`main.rs` has some starter code for you, including printing the eyes and that blank grid. All you have to do is implement the smiley!

Here are some hints to help you along:
* Replace a character in a string: ||string.replace_range(x..=x, "*")||
* Random number between -1 and 1: ||rng.random_range(-1..=1)||
* Adding with type casting: ||y = (y as i64 + val) as usize||

Happy coding!