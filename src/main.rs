fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("Guess: {}", guess);

    let x: i32 = 5; // integer
    let y: f32 = 3.0; // floating number
    let z: bool = true; // boolean
    let a: char = 'a'; // character
    let b: [i32; 5] = [1, 2, 3, 4, 5]; // array - [type; size]
    let c: (i32, f64, u8) = (500, 6.4, 1); // tuple
    let (d, e, f) = c; // destructuring tuple
    let g = c.0; // access tuple element
    let h = c.1; // access tuple element

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("Months: {:?}", months);

    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {}", z);
    println!("a: {}", a);
    println!("b: {:?}", b);
    println!("c: {:?}", c);
    println!("d: {}", d);
    println!("e: {}", e);
    println!("f: {}", f);
    println!("g: {}", g);
    println!("h: {}", h);
}
