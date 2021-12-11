use records::record;

#[record]
struct Greeter {
    greeting: String,
    name: String,
}

impl Greeter {
    fn hello(&self) {
        println!("{}, {}!", self.greeting, self.name);
    }
}

pub fn main() {
    let greeter = Greeter::new(String::from("Hello"), String::from("World"));
    greeter.hello();
}
