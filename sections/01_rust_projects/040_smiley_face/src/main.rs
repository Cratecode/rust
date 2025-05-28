use rand::prelude::*;

fn main() {
    let mut rng = rand::rng();

    println!("  **  **  ");
    println!("  **  **  ");
    println!();

    // 10 spaces (feel free to change it though!)
    let mut mouth = vec!["          ".to_string(); 4];

    // TODO: your code here

    for row in mouth {
        println!("{row}");
    }
}