fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    const MAX_POINTS: u32 = 100_000;

    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let mut spaces = "   ";
    let spaces = spaces.len();

    let x = 2.0;
    let y: f32 = 3.0;

    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    // println!("The value of tup is {}", tup);
    // let tup = ('a', 3);
    // println!("The value of tup is {}", tup);

    let a = [1, 2, 3, 4, 5];
    println!("The value of a is {}", a[0])
    // let a: [i32; 5] = [1, 2, 3, 4, 5];
    // println!("The value of a is {}", a[0])
    // let a = [3; 5];
    // println!("The value of a is {}", a[0])

}
