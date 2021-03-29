fn main() {
    // println!("Hello, world!");
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    take_ownership(s2);
    let x = 5;
    makes_copy(x);
    
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    change (&mut s);

    let r1 = &s;
    let r2 = &s;
    println!("{} {}", r1 , r2);


    // Slices
    let string = String::from("Hillo");
    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..];

    let stri = String::from("Hello World");

}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(" , world!"); 
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}