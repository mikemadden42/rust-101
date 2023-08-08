fn main() {
    println!("Hello, Rust 101!");

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
