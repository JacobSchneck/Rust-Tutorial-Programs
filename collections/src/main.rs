use std::collections::HashMap;

fn main() {
    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Blue"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

}

fn strings() {
    // vectors();
    let mut s = String::new();
    // let data = "intial contents";
    // let s = data.to_string();
    // println!(s);

    // let s2 = "bar";
    // s.push_str("bar");
    // println!("s2 is {}", s2);

    // let s1 = String::from("Hello, ");
    // let s2 = String::from("world! ");
    // let s3 = s1 + &s2;
    // println!("{}", s3);

    // let s1 = String::from("t")    
    // let s2 = String::from("t")
    // let s3 = String::from("t")
    // let s = format!("{}-{}-{}", s1, s2, s3);

    
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
}

fn vectors() {
    println!("Hello, world!");
    let _v: Vec<i32> = Vec::new();

    // let v = vec![1, 2, 3];
    // let mut v = Vec::new();
    // v.push(5);
    // v.push(6);
    // v.push(7);
    // v.push(8);

    // let v = vec![1, 2, 3, 4, 5];
    // let third: &i32 = &v[2];
    // println!("The third element is {}", third);

    // match v.get(2) {
    //     Some(third) => println!("The third element is {}", third),
    //     None => println!("There is no third element."),
    // }

    // let does_not_exist = &v[100];
    // let does_not_exist = v.get(100);

    let mut v = vec![1, 2, 3, 4, 5];
    // let mut first = &v[0];
    v.push(6);

    // println!("The first element is: {}", first);

    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 50;
    }
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for el in &row {
        println!("{:#?}", el);
    }

}


