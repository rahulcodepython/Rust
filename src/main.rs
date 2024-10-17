fn main() {
    let mut x = 5; // mutable variable
    let y = 8; // immutable variable

    println!("The value of x is: {x}"); // before updating x
    x = 6;
    println!("The value of x is: {x}"); // after updating x

    println!("The value of y is: {y}");

    {
        let x = x * 2; // shadowing && local scope
        println!("The value of x in the inner scope is: {x}"); // access the inner scope
    }

    println!("The value of x is: {x}"); // access the outer scope

    let x = x * 2; // shadowing
    println!("The value of x is: {x}");
}
