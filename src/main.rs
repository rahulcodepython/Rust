fn main() {
    let a = 5; // owned by main function
    {
        let b = 10; // owned by inner block
        println!("a: {}, b: {}", a, b);
    }
    println!("a: {}", a);

    let x = 7; // owned by main function
    let y = 8; // owned by main function
    let z = sum(x, y); // z is owned by main function
                       // x and y are borrowed by sum function, so they are not accessible here
    println!("sum: {}", z);

    copy();
}

fn sum(a: i32, b: i32) -> i32 {
    a + b // return value
}

fn copy() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2
                 // println!("s1: {}", s1); // error: value borrowed here after move
    println!("s2: {}", s2);
    let s3 = s2.clone(); // s2 is cloned to s3
    println!("s2: {}, s3: {}", s2, s3);
}
