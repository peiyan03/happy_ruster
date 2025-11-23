fn main() {
    
    let x = 5;

    // the x+1 is using the old x to create a new x
    let x = x + 1; // Shadowing x, now x is 6

    {
        let x = x * 2; // Shadowing x again in inner scope, now x is 12
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x in the outer scope is: {}", x); // x is 6 here
}
