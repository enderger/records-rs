use records::record;

#[record]
struct Greeter {
    greeting: String,
    emphasis: char,
}

impl Greeter {
    fn greet(&self, name: &str) {
        println!("{}, {}{}", self.greeting, name, self.emphasis);
    }
}

pub fn main() {
    let greeter = Greeter::new(String::from("Hello"), '!');
    greeter.greet("World");
}
