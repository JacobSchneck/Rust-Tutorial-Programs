use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/* Section 16.1*/

// fn main() {
//     let handle1 = thread::spawn(|| {
//         for i in 1..10 {
//             println!("hi number {} from the spawned thread!", i);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });

//     for i in 1..5 {
//         println!("hi number {} from the main thread!", i);
//         thread::sleep(Duration::from_millis(1));
//     }

//     handle1.join().unwrap();
//     let v = vec![1, 2, 3];

//     let handle2 = thread::spawn(move || {
//         println!("Here's a vector: {:?}", v);
//     });

//     handle2.join().unwrap();
// }

/* Section 16.2 */
// fn main() {
//     let (tx, rx) = mpsc::channel();

//     let tx1 = tx.clone();
//     thread::spawn(move || {
//         let vals = vec![
//             String::from("hi"),
//             String::from("from"),
//             String::from("the"),
//             String::from("thread"),
//         ];

//         for val in vals {
//             tx1.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });

//     thread::spawn(move || {
//         let vals = vec![
//             String::from("more"),
//             String::from("messages"),
//             String::from("for"),
//             String::from("you"),
//         ];
//         for val in vals {
//             tx.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });


//     for recieved in rx {
//         println!("Got: {}", recieved);
//     }
// }

/* Section 16.3 */
fn main() {

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();  
    }

    println!("Result: {}", *counter.lock().unwrap());

    // let m = Mutex::new(5);

    // { 
    //     let mut num = m.lock().unwrap();
    //     *num = 6;
    // }

    // println!("m = {:?}", m);
}