const VERSION: &str = env!("CARGO_PKG_VERSION");
const PI: f64 = 3.141592653589793;

fn datatypes() {
    println!("Hello, world!");

    // Primitive Datatypes

    // In Integers: Rust has signed (+ & -) and unsigned (only +) types of diff sizes
    // signed integers: i8, i16, i32, i64, i128
    // unsigned integers: u8, u16, u32, u64, u128
    let a: i8 = -127;
    let e: i32 = 2147483647;
    let i: i64 = 922337203685477580;
    // let a: u8 = -127; // not allowed
    println!("The value of e is {}!", e);
    println!("The value of i is {}!", i);
    println!("The value of a is {}!", a);

    // Floating points
    // f32, f64
    let pi: f64 = 3.1415926;
    println!("The value of pi is {}!", pi);

    // boolean values: true or false
    // bool
    let is_showing: bool = true;
    println!("The value of is_showing is {}!", is_showing);

    // character type
    let letter: char = 'a';
    println!("The value of letter is {}!", letter);

    // Compound Data types
    // arrays, tuples, slices, and strings (slice string)

    // Arrays
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of number is {:?}!", numbers);

    let fruits: [&str; 3] = ["apple", "apple", "banana"];
    println!("The value of fruit is {:?}!", fruits);
    // let mix = [1,2,"okay", true]; - invalid array

    // Tuples
    // can container mixed data
    let human: (&str, i32, &str) = ("Gaurav", 30, "A+");
    let numan: (String, i32, [i32; 5]) = ("Gaurav".to_string(), 30, [1,2,3,4,5]);
    println!("The value of human is {:?}!", human);
    println!("The value of human is {} {} {:?}!", numan.0, numan.1, numan.2);

    // Slices: dynamic contiguous elements, all next to each other, faster access
    let number_slice:&[i32] = &[1,2,3,4];
    println!("The value of number_slice is {:?}!", number_slice);
    let animal_slice:&[&str] = &["lion", "elephant", "crocodile"];
    println!("The value of animal_slices is {:?}!", animal_slice);
    let book_slice:&[&String] = &[&"IT".to_string(), &"Science".to_string()];
    println!("The value of book_slices is {:?}!", book_slice);

    // String Vs String Slices (&str)
    // String [growable, mutable, owned string type]
    let mut stone_cold: String = String::from("Hell, ");
    stone_cold.push_str("Yeah!");
    println!("Stone cold says: {}", stone_cold);

    let sample_string: String = String::from("Hello World!");
    let sample_slice: &str = &sample_string;
    let sample_slice_range: &str = &sample_slice[0..5];
    println!("The value of sample_string is {:?} {:?}!", sample_slice, sample_slice_range);

}

fn main() {
    print_msg();
    human_id("Gaurav", 30, 186.0);

    let x: i32 = {
        let price: i32 = 5;
        let quantity: i32 = 9;
        price * quantity
    };

    println!("Value of X is {}!", x);
    println!("Area of circle with radius {} is {}", 7, area_of_circle(7.0));
}

fn print_msg() {
    println!("Hello, world!");
}

fn human_id(name: &str, age: i32, height: f32) {
    println!("{} is {} years old with height {} cm", name, age, height);
}

fn area_of_circle(radius: f64) -> f32 {
    (radius * PI) as f32
}