
/*
Data Types in Rust
Primitive Data Types:
1. Integer Types: i8, i16, i32, i64, i128 (signed), u8, u16, u32, u64, u128 (unsigned)
2. Floating-Point Types: f32, f64
3. Boolean Type: bool (true or false)
4. Character Type: char (Unicode scalar value)
5. Unit Type: () (represents an empty value)
Compound Data Types:
1. Tuples: Fixed-size collection of values of different types.
2. Arrays: Fixed-size collection of values of the same type.
3. Slices: Dynamically-sized view into a contiguous sequence (usually part of an array).
4. Strings: UTF-8 encoded, growable, heap-allocated data structure (String) and string slices (&str).   

// i8,i16,i32,i64,i128 theese are signed integers
// i32 range is -2^31 to 2^31-1
// u8,u16,u32,u64,u128 theese are unsigned integers

Signed Integers (i64):
    Use one bit (the most significant bit) to represent the sign of the number (0 for positive, 1 for negative).
    This leaves 63 bits for the magnitude of the number.
    Range: -2^63 to 2^63 - 1 (i.e., -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807).

Unsigned Integers (u64):
    Do not use any bits for the sign, so all 64 bits are used to represent the magnitude.
    Range: 0 to 2^64 - 1 (i.e., 0 to 18,446,744,073,709,551,615).

Demonstrates the use of various integer types in Rust.
 - Smaller the number of bits, smaller the range of values it can hold.
 */

fn data() {
    let a: i8 = -10;
    let b: u8 = 10;
    let dum: u8 = 1;//2554412; // Results in a compile-time error: literal out of range for `u8`

    let c: i16 = -300;
    let d: u16 = 300;//-300; // This will cause a compile-time error because u16 cannot hold negative values.

    let e: i32 = -20000;
    let f: u32 = 20000;

    let g: i64 = -500000;
    let h: u64 = 500000;

    let i: i128 = -10000000000;
    let j: u128 = 10000000000;

    println!("Signed integers: {}, {}, {}, {}, {}, {}", a, c, e, g, i, dum);
    println!("Unsigned integers: {}, {}, {}, {}, {}", b, d, f, h, j);

    let pi: f64 = 3.14; // 32-bit floating point
    
    let check: bool = true; // boolean type
    let check2: bool = false;

    println!("Floating point: {}", pi);
    println!("Boolean values: {}, {}", check, check2);

    // Character type
    let letter: char = 'A';
    let emoji: char = '😊';
    println!("Characters: {}, {}", letter, emoji);

    // String and &str
    let string_slice: &str = "Hello, Rust!"; // string slice
    let string_object: String = String::from("Hello, String!"); // String object

    let check = "yes";
    
}

// --------------------------------------------------------------------------------

fn data_types() {
    // In array, all elements must be of the same type

    let numbers: [i32; 5] = [1, 2, 3, 4, 5]; 
    println!("Array: {:?}", numbers);

    // &str is a string slice, an array of characters, it is reference
    let mix: [&str; 3] = ["Hello", "World", "Rust"];
    println!("String Array: {:?}", mix);
    println!("First element: {}", mix[0]);

    // Tuple can hold different types
    let human = ("Bob", 25, 6.1);
    println!("Tuple: {:?}", human); 
    println!("Name: {}, Age: {}, Height: {}", human.0, human.1, human.2);
    let person: (&str, i32, f64) = ("Alice", 30, 5.5);
    println!("Tuple: {:?}", person);
    println!("Name: {}, Age: {}, Height: {}", person.0, person.1, person.2);

    let check: (String, u32) = ("yes".to_string(), 1); // Need to use String type for owned string by to_string()
    println!("Check: {:?}", check);
}

fn slices_example() {
    let slice: &[i32] = &[10, 20, 30, 40, 50];
    println!("i32 Slice: {:?}", slice);

    let str_slice: &[&str] = &["apple", "banana", "cherry"];
    println!("String Slice: {:?}", str_slice);

    let book: &[&String] = &[&"Title".to_string(), &"Author".to_string(), &"Year".to_string()];
    println!("Book Info: {:?}", book);

    // String adn String slices
    /*
        - String are [growwable, mutable, owned, heap-allocated] data structures
        - &str are [immutable, fixed-length, borrowed, stack-allocated] data structures
     */

    // String are heap allocated, mutable, owned, slower as it grows bigger
    let mut string_name: String = String::from("Hello, Rust!"); //default are always inmutable
    string_name.push_str(" Welcome to Rust programming.");
    println!("String: {}", string_name);

    // &str example, good for memory efficiency wihtout ownership overhead, stack 
    let str_name:  String = String::from("Hello, Rust!");
    let str_slice: &str = &str_name; // Borrowing String as &str slice, only reference, not ownership
    println!("&str Slice: {}", str_slice);
    println!("Original String: {}", str_slice[0..5].to_string()); // Slicing &str
    

}

fn variables(){
    let _a: i32 = 10;
    let a = 15; // Shadowing, _a is still 10, a is 15
    let mut _b: i32 = 20;
    _b = 30; 

    // Constants is immutable by default and is declared using 'const' keyword
    // Must be Capitalized and have type annotation
    const MAX_POINTS: u32 = 100_000;

    println!("_a: {}, a: {}, _b: {}, MAX_POINTS: {}", _a, a, _b, MAX_POINTS);
    println!("MIN_POINTS: {}", MIN_POINTS);
}

// You can declare consts outside functions
const MIN_POINTS: u32 = 0;


// --------------------------------------------------------------------------------

fn main() {
    slices_example();
}