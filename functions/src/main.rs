fn main() {
    println!("Hello, world!");
    // another_function();
    another_function(5);

    let _x = 5;
    let y = { 
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);

    let mut x  = five();
    println!("The value of x is: {}", x);
    x = plus_one(x);
    println!("The value of x is: {}", x);
}

// fn another_function() {
//     println!("Another function.");
// }

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    4
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
