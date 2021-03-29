fn main() {
    let number = 3;

    if number > 5 {
        println!("condition was true");
    } else if number == 3 {
        println!("the numbah was three");
    } else {
        println!("condition was false");
    }

    let number = if true { 5 } else { 6 };
    println!("The value of number is: {}", number);

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    let mut numbah = 3;

    while numbah != 0 {
        println!("{}!", numbah);
        numbah -= 1;
    }

    println!("Lift off what!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("The value is: {}", a[index]);
        index += 1;
    }

    for element in a.iter() {
        println!("the value beist {}", element);
    }

    for n in (1..4).rev() {
        println!("{}!", n);
    }
    println!("LIFT OFFFFF!");
}
