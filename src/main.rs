use std::io;

// These constants are global.
const MPG: f64 = 37.5;
const BIRTHDAY: i32 = 1;

fn main() {
    println!("Hello, Rust 101!");
    primatives();
    variables();
    challenge1();
    compound_primatives();
    strings();
    user_input();
    math();
    challenge2();
    comparisons();
    conditionals();
    matches();
    loops();
    challenge3();
    vectors();
    structs();
    enums();
}

fn enums() {
    #[derive(Debug)]
    enum Shape {
        Circle(f32),
        Rectangle(f32, f32),
    }

    let circle = Shape::Circle(10.0);
    let rectangle = Shape::Rectangle(30.0, 40.0);

    dbg!(circle);
    dbg!(rectangle);
}

fn structs() {
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    let area = rect.area();
    println!("The area of the rectangle is {area}");
}

fn vectors() {
    let vec = vec![1, 2, 3];
    dbg!(&vec);

    for num in vec {
        println!("{num}");
    }
}

fn challenge3() {
    println!("Rust 101 Calculator");
    println!("You must select two values (x and y) and an operator.");

    //Receive a value for X
    println!("Please give me a value for X.");

    let mut x = String::new();
    let _ = io::stdin().read_line(&mut x);
    let x: i32 = x.trim().parse().expect("Entry was not an integer.");
    let float_x = f64::from(x);

    //Receive a value for Y
    println!("Please give me a value for Y.");

    let mut y = String::new();
    let _ = io::stdin().read_line(&mut y);
    let y: i32 = y.trim().parse().expect("Entry was not an integer.");
    let float_y = f64::from(y);

    //Receive an operator
    println!("Choose an operator: +, -, *, /");
    let mut operator = String::new();
    let _ = io::stdin().read_line(&mut operator);
    let operator_slice = operator.trim();

    //Match operator
    match operator_slice {
        "+" => {
            add(x, y);
        }
        "-" => {
            subtract(x, y);
        }
        "*" => {
            multiply(x, y);
        }
        "/" => {
            divide(float_x, float_y);
        }
        &_ => {
            println!("Invalid entry. Exiting program.");
        }
    }
}

//Math functions
fn add(x: i32, y: i32) {
    println!("The result of {} + {} = {}", x, y, x + y);
}
fn subtract(x: i32, y: i32) {
    println!("The result of {} - {} = {}", x, y, x - y);
}
fn multiply(x: i32, y: i32) {
    println!("The result of {} * {} = {}", x, y, x * y);
}
fn divide(x: f64, y: f64) {
    println!("The result of {} / {} = {}", x, y, x / y);
}

fn loops() {
    let mut count = 0;
    loop {
        count += 1;
        println!("Count: {count}");
        if count == 10 {
            break;
        }
    }

    let mut count = 0;
    while count < 10 {
        count += 1;
        println!("Count: {count}");
    }

    for _ in 1..=10 {
        println!("Count: {count}");
    }

    let numbers = vec![1, 2, 3, 4, 5];

    for num in numbers {
        println!("Number: {num}");
    }
}

fn matches() {
    let age = 33;
    // match must cover all possible values.
    match age {
        1..=24 => println!("Cannot hold office"),
        25..=29 => println!("Can run for the house"),
        30..=34 => println!("Can run for the senate"),
        35..=i32::MAX => println!("Can run for president"),
        _ => println!("Are you an infant"),
    };
}

fn conditionals() {
    println!("How much money do you have?");
    let mut input_money = String::new();
    let _ = io::stdin().read_line(&mut input_money);

    let money: i32 = input_money
        .trim()
        .parse()
        .expect("Entry was not an integer");

    println!("How old are you?");
    let mut input_age = String::new();
    let _ = io::stdin().read_line(&mut input_age);

    let age: i32 = input_age.trim().parse().expect("Entry was not an integer");

    if (age >= 21) && (money >= 5) {
        println!("We're getting a drink!");
    } else if (age >= 21) && (money < 5) {
        println!("Come back with more money!");
    } else if (age < 21) && (money >= 5) {
        println!("Nice try, kid!");
    } else {
        println!("You're too young and too poor.");
    };
}
fn comparisons() {
    let a = 5;
    let b = 10;
    let c = true;
    let d = false;

    println!("a > b: {}", a > b); // false
    println!("a >= b: {}", a >= b); // false
    println!("a < b: {}", a < b); // true
    println!("a <= b: {}", a <= b); // true
    println!("a == b: {}", a == b); // false
    println!("a != b: {}", a != b); // true
    println!("True or False: {}", c || d); //true
    println!("True and False: {}", c && d); //false
}

fn challenge2() {
    /* Build a simple calculator that takes two user inputs
       then calculates the addition, subtraction, multiplication, and division
       of those two inputs.
    */

    println!("Give me a value for x");
    let mut input_x = String::new();
    let result_x = io::stdin().read_line(&mut input_x);

    let x: i32 = input_x.trim().parse().expect("Entry was not an integer!");
    let float_x = f64::from(x);

    println!("Give me a value for y");
    let mut input_y = String::new();
    let result_y = io::stdin().read_line(&mut input_y);

    let y: i32 = input_y.trim().parse().expect("Entry was not an integer!");
    let float_y = f64::from(y);

    println!("{result_x:?}");
    println!("{result_y:?}");
    println!("The result of {} + {} is {}", x, y, x + y);
    println!("The result of {} - {} is {}", x, y, x - y);
    println!("The result of {} * {} is {}", x, y, x * y);
    println!("The result of {} / {} is {}", x, y, float_x / float_y);
}

fn math() {
    let x = 11;
    let y = 9;
    println!("{} + {} = {}", x, y, x + y);
    println!("{} - {} = {}", x, y, x - y);
    println!("{} * {} = {}", x, y, x * y);
    println!("{} / {} = {}", x, y, x / y);
    println!("{} % {} = {}", x, y, x % y);
}

fn user_input() {
    println!("username: ");
    let mut username = String::new();
    let result = io::stdin().read_line(&mut username);
    println!("Welcome {}...", username.trim_end());
    println!("{result:?}");
}

fn strings() {
    // There are many different types of strings.
    // We will likely use String and &str the most.
    // str - string slice
    // &str - borrowed string slice - cannot be modified
    // String - can be modified
    // &str - essentially a subset of String
    // String - has capacity

    // There is no push_str method for &str.
    // let name = "Wyatt";
    // name.push_str(" Earp");
    // println!("{name}")

    let mut name1 = String::new();
    name1.push_str("Wyatt Earp");
    println!("{name1}");

    let name2 = "Virgil Earp".to_string();
    let name3 = String::from("Doc Holiday");
    println!("{name2}");
    println!("{name3}");

    let message = "Hello, \"world\"!";
    println!("{message}");

    let heart = '\u{2764}';
    println!("{heart}");
}

fn compound_primatives() {
    // TUPLES
    // Max number of items in a tuple is 12.
    let student_a = ("Johhny", "C", 2.01);
    // let name = student_a.0;
    // let grade = student_a.1;
    // let gpa = student_a.2;
    let (name, grade, gpa) = student_a;
    println!("Name: {name}, Grade: {grade}, GPU: {gpa}");

    // ARRAYS
    // Use brackets [].
    // Max number of items in an array is 32.
    // Elements must be similar data types.
    let students = ["Johnny", "Bill", "Ike"];
    println!("Second student: {}", students[1]);

    // SLICES
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let slice = &numbers[2..6];
    println!("{slice:?}");
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
