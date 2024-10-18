fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s;

    // There may have multiple reference variables of a variable at a same time in a scope. But, there can be only one mutable reference variable of a variable at a same time in a scope without

    // When I am taking a mutable reference of a variable, I can't take any other reference of that variable in the same scope. This is because, mutable reference can change the value of the variable, so it is not safe to have other reference of that variable in the same scope.

    // If we do this, we will get a compile time error. Because mutable reference may change the value of the variable which doesn't reflect the change to the other reference variables. So, it is not safe.

    println!("{}, {}, and {}", r1, r2, r3);
}
