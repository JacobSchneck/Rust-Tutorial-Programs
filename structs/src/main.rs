struct User {
    username: String,
    email: String, 
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    println!("Hello, world!");
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let mut user2 = build_user(String::from("email@email.com"), String::from("usernameFromUsername.com"));
    user2.active = user1.active;

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let balck = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let rect: Rectangle = Rectangle {
        width: 10, 
        height: 10
    };

    println!("{:?}", area(&rect));
    println!("{:#?}", rect);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("the are of the rectangle is {} square pixels.", rect1.area());
    
}

fn area (rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

// fn build_user(email: String, username: String) -> User {
//         User {
//             email: email,
//             username: username,
//             active: true,
//             sign_in_count: 1,
//         }
// }

fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
}