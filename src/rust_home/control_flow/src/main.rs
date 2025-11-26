fn main() {
    /*
    if and else needs to have compateable types, same types
    - It cant be an integer in one branch and a string in another
     */
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    }
    if number % 2 == 0 { 
        println!("nope")
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    loop_example();
    loop_label_example();
    for_loop_example();
    while_example();
}


fn loop_example() {
    loop{
        println!("again!");
        break;
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // returning value from loop
        }
    };

    println!("The result is {result}");
}


fn loop_label_example() {
    let mut count = 0;
    'counting_up: loop { //outer loop with label
        println!("count = {count}");
        let mut remaining = 10;

        'innerloop: loop {  //inner loop
            println!("remaining = {remaining}");
            if remaining == 9 {
                break 'innerloop;
            }
            if count == 2 {
                break 'counting_up; //breaks the outer loop
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn for_loop_example() {
    let a = [10, 20, 30, 40, 50];

    // Note: You can explicitly define the type of the element in the loop as follows:
    // for element in a.iter() {
    //     let element: &i32 = element; // Explicitly define the type
    //     println!("the value is: {element}");
    // }
    // Alternatively, if consuming the array, you can use:
    // for element in a {
    //     let element: i32 = element; // Explicitly define the type
    //     println!("the value is: {element}");
    // }
    for element in a {
        println!("the value is: {element}");
    }
}

fn while_example() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}