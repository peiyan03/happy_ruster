/* 
Provide why some complier errors occur with Option<T> and Result<T, E> types
1. Mismatched Types: Trying to use an Option<T> where a concrete type is expected without handling the None case.
2. Unwrapping None: Calling unwrap on an Option<T> that is None, leading to a runtime panic
3. Pattern Matching Errors: Failing to cover all cases when matching on an Option<T> or Result<T, E>, leading to non-exhaustive patterns.
*/

enum Results<T, E> { // Define the generic Result type
    Ok(T), // represents success and holds a value of type T
    Err(E), // represents failure and holds an error value of type E
}

// Option is like enum with two variants: Some<T> and None
// // OPTION<T> is used when a value can be either something or nothing
// enum Option<T> { // Define the generiuc option type

//     Some(T),// is the value present, it holds a value of type T
//     None,// is the value absent, No value
// }

fn divideOption(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
      Some(numerator / denominator)
    } 
}

fn divideResults(numerator: f64, denominator: f64) -> Results<f64, String> {
    if denominator == 0.0 {
        Results::Err(String::from("Cannot divide by zero"))
    } else {
      Results::Ok(numerator / denominator)
    } 
}

fn main() {
    let result = divideOption(10.0, 3.0);

    match result {
        Some(x) => println!("Result: {}", x),
        None => println!("Cannot divide by zero"),
    }

    match divideResults(10.0, 0.0){
        Results::Ok(x) => println!("Result: {}", x),
        Results::Err(e) => println!("Error: {}", e),
    }
}
