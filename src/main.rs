fn main() {
    let mut a = 1;

    a = takes_and_return_ownership(a); // a is moved to takes_and_return_ownership. Now it is owned by takes_and_return_ownership
    println!("{}", a); // This will print 1 because a is moved to takes_and_return_ownership and then returned back to main

    takes_reference(&a); // a is borrowed by takes_reference. It is still owned by main

    takes_ownership(a); // a is moved to takes_ownership. Now it is owned by takes_ownership
                        // println!("{}", a); // This will give an error because a is moved to takes_ownership
}

fn takes_and_return_ownership(a: i32) -> i32 {
    println!("{}", a);
    a
}

fn takes_reference(a: &i32) {
    println!("{}", a);
}

fn takes_ownership(a: i32) {
    println!("{}", a);
}
