/* Section 17.2 */

// pub trait Draw {
//     fn draw(&self);
// }

// pub struct Screen {
//     pub components: Vec<Box<dyn Drraw>>,
// }

// impl Screen {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }

// pub struct Button {
//     pub width: u32,
//     pub height: u32,
//     pub label: String,
// }

// impl Draw for Button {
//     fn draw(&self) {

//     }
// }

// use gui::Draw;

// struct SelectBox {
//     width: u32,
//     height: u32,
//     options: Vec<String>,
// }

// // Alternative restrictive and not polymorphic

// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }

// impl<T> Screen<T> 
// where 
//     T: Draw,
// {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }

/* Section 17.3*/
pub struct Post {
    state: Option< Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}

trait State {}

struct Draft {

}

impl State for Draft {}
