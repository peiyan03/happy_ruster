fn main() {
    // Vector has same type for all elements
    // Dynamic and growable array

    /*
    vec![] macro to create a new vector, optional initial values 
    Vec::new() function to create an empty vector, then use push method to add elements
     */
    let mut _v: Vec<i32> = Vec::new();

    // Rust conveniently provides the vec! macro, 
    // which will create a new vector that holds the values you give it
    let mut _v2 = vec![1, 2, 3, 4, 5]; // macro to create a vector with initial values

    _v.push(10);
    _v2.push(6);

    println!("Vector v: {:?}", _v);
    println!("Vector v2: {:?}", _v2);

    // 0 index to access elements
    let _v3: Vec<i32> = vec![10, 20, 30, 40, 50];
    let third: i32 = _v3[2]; // using indexing

    let second: &i32 = &_v3[1]; // Returns a reference to the element
    println!("The third element if v3: {}", third);


    // Get method returns an Option<&T>
    match _v3.get(1) { // get index 1, so second element
        Some(second) => println!("The second element is: {}", second),
        None => println!("There is no second element."),
    }
}



 