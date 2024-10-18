fn main() {
    let name = String::from("ABCD_abcd");
    let result = string_method(&name);

    println!("The result is: {}", result);
}

fn string_method(s: &String) -> usize {
    let bytes = s.as_bytes();

    println!("{:?}", bytes);
    println!("{:?}", bytes.iter().enumerate());

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
        println!("{}: {}", i, item);
    }

    s.len()
}
