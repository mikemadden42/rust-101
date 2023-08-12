// These constants are global.
const MPG: f64 = 37.5;
const BIRTHDAY: i32 = 1;

fn main() {
    println!("Hello, Rust 101!");
    primatives();
    variables();
    challenge1();
}

fn challenge1() {
    /* Challenge 1 - Build a program that has the following:

    1) Has a global constant integer named 'birthday' with a value of 1
    2) Has a local string variable named 'my_name' with your name as the value
    3) Has a local string variable named 'my_birthday' with your birth month/day (no year) as the value
    4) Has a local mutable integer variable named 'age' with your current age as the value
    5) Has a local integer variable named 'new_age' with your age after your birthday as the value
    6) Prints out 'My name is X and I am X years old. I will turn X on X'

    */

    let my_name = "Johnny";
    let my_birthday = "07/13";
    let age = 141;
    let new_age = age + BIRTHDAY;

    println!(
        "My name is {my_name} and I am {age} years old. I will turn {new_age} on {my_birthday}"
    );
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
