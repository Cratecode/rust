// This is a Mandelbrot Set (https://en.wikipedia.org/wiki/Mandelbrot_set)
// renderer. Click the run button to try it out!

use std::sync::{Arc, Mutex};

/// The number of iterations to render at.
const ITERATIONS: usize = 100;

/// The width of the output (in characters).
const WIDTH: usize = 90;
/// The height of the output (in characters).
const HEIGHT: usize = 24;

/// The number of threads to distribute the work to.
const THREADS: usize = 4;

// This could be achieved using the `num-complex` crate.

/// A complex number in rectangular form.
#[derive(Copy, Clone)]
struct ComplexNumber(f64, f64);

impl ComplexNumber {
    /// Adds two complex numbers and returns the result.
    pub fn add(self, other: ComplexNumber) -> ComplexNumber {
        ComplexNumber(self.0 + other.0, self.1 + other.1)
    }

    /// Multiplies two complex numbers and returns the result.
    pub fn mul(self, other: ComplexNumber) -> ComplexNumber {
        ComplexNumber(self.0 * other.0 - self.1 * other.1, self.0 * other.1 + self.1 * other.0)
    }
}

fn calculate_pixel(x: usize, y: usize) -> &'static str {
    // Figure out what x and y mean for complex numbers.
    // x needs to be mapped from the range [0, WIDTH) to [-1.5, 1.5].
    // y needs to be mapped the same way to [-1, 1], then multiplied
    // by -1 because it's flipped (y=0 is the top of the image).
    let x = (((x as f64) / ((WIDTH as f64) - 1.0)) * 4.0) - 2.0;
    let y = -(((y as f64) / ((HEIGHT as f64) - 1.0)) * 2.0) + 1.0;

    let c = ComplexNumber(x, y);

    // Calculate the value using this equation:
    // z_0 = (0, 0)
    // z_n = (z_n-1)^2 + c
    //
    // If either part of z > 2, show a space.
    // Otherwise, show an asterisk (*).
    let mut z = ComplexNumber(0.0, 0.0);

    for _ in 0..ITERATIONS {
        z = z.mul(z).add(c);

        if z.0 > 2.0 || z.1 > 2.0 {
            return " ";
        }
    }

    "*"
}

/// Calculates the given pixels and adds them to the output list.
/// index is a number (starting at 0) representing which thread
fn calculate_pixels<const ROWS: usize>(index: usize, output: Arc<Mutex<[Option<[[&str; WIDTH]; ROWS]>; THREADS]>>) {
    let mut output_chunk = [[" "; WIDTH]; ROWS];
    let row_offset = ROWS * index;

    for y in 0..ROWS {
        let real_y = row_offset + y;

        for x in 0..WIDTH {
            output_chunk[y][x] = calculate_pixel(x, real_y);
        }
    }

    // Add the chunk to the output.
    output.lock().unwrap()[index] = Some(output_chunk);
}

fn main() {
    // This could be achieved using the `rayon` crate.

    let output = Arc::new(Mutex::new([None; THREADS]));

    // Generate the image.
    let mut threads = Vec::with_capacity(THREADS);
    for thread in 0..THREADS {
        let output = output.clone();
        // Each thread will be responsible for HEIGHT / THREADS rows.
        threads.push(std::thread::spawn(move || calculate_pixels::<{HEIGHT / THREADS}>(thread, output)));
    }

    // Wait for it to be generated.
    for thread in threads {
        thread.join().unwrap();
    }

    // Display the image.
    let output = output.lock().unwrap();
    for chunk in output.into_iter() {
        let Some(chunk) = chunk else {
            eprintln!("ERROR: Not all threads completed successfully!");
            return;
        };

        for row in chunk {
            for char in row {
                print!("{char}");
            }

            println!("");
        }
    }
}