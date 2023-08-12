// This constant is global.
const MPG: f64 = 37.5;

fn main() {
    println!("Hello, Rust 101!");
    primatives();
    variables();
}

fn variables() {
    // Variables are imutable by default.
    let greeting = "Hello, world!";
    println!("{greeting}");

    // This will to work without declaring greeting with a mut.
    // greeting = "Hello, again!";
    // println!("{greeting}");

    // constants
    println!("Miles per gallon: {MPG}");

    // suffixes & underscores
    let x = 42_u32;
    let y = 1_000_000;
    println!("x = {x}");
    println!("y = {y}");
}

fn primatives() {
    // integers
    println!("Max size of u32: {}", u32::MAX);
    println!("Max size of u64: {}", u64::MAX);
    println!("Max size of u128: {}", u128::MAX);

    // floats
    println!("Min size of f32: {}", f32::MIN);
    println!("Max size of f32: {}", f32::MAX);
    let mpg = 25.1;
    println!("Miles per gallon: {mpg}");

    // boolean
    if mpg > 20.0 {
        println!("{mpg} mpg is decent gas mileage.");
    }

    // characters
    let grade = 'A';
    println!("Grade: {grade}");
}
