use std::fmt::Display;
use std::ops::Add;

pub fn add<T: Add + Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}

pub fn generic_display<T: Display>(value: T) {
    println!("{}", value);
}
