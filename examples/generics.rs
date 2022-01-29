use std::fmt::Display;

use records::record;

#[record]
#[derive(Clone)]
struct Printer<T> {
    it: T,
}

impl<T: Display> Printer<T> {
    pub fn print(&self) {
        println!("PRINTER: {}", self.it)
    }
}

pub fn main() {
    let printer = Printer::new("Hello, world!");
    printer.print();
    let (msg,) = printer.clone().into();
    println!("Message: {}", msg);
}
