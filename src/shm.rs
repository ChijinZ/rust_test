extern crate memmap;

use std::{thread, time, env};
use std::io::{self, Write};
use std::fs::OpenOptions;

use std::path::{PathBuf, Path};

use memmap::*;

const MAP_SIZE: u64 = 1 << 16;

fn main() {
    let a = env::args().skip(1).collect::<Vec<_>>();
    match a.first().unwrap().as_str() {
        "create" => create(),
        "read" => read(),
        _ => panic!("failed"),
    };
}

fn create() {
    let mut path = Path::new("./a.txt");
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path).unwrap();
    file.set_len(MAP_SIZE);

    let mut mmap = unsafe { MmapMut::map_mut(&file).unwrap() };
    println!("{:?}", (&mmap[0..128]));
    thread::sleep(time::Duration::from_millis(10000));
    println!("{:?}", (&mmap[0..128]));
}

fn read() {
    let mut path = Path::new("./a.txt");
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(&path).unwrap();
    let mut mmap = unsafe { MmapMut::map_mut(&file).unwrap() };
    for i in 0..128 {
        mmap[i] = i as u8;
    }
}