use std::cell::RefCell;
use std::sync::{Arc, Mutex};

fn main() {
    let mut vec = vec![0, 1, 2];
    {
        let vec = &mut vec;
        test(move || {
            vec.push(3);
            println!("{:?}", vec);
        });
    }

//    std::thread::spawn(||{
//        println!("{:?}",vec);
//    }).join() ;
    println!("{:?}", vec);
}

fn test<F>(mut closure: F)
    where F: FnMut() {
    closure();
}