use std::thread;

struct Philosopher {
    name: String
}

// new is an 'associated function' on Philosopher
// Since Rust is expression-based, the last/only expression,
// instantiating a Philosopher, is returned.
impl Philosopher {
    fn new(name: &str) -> Philosopher {
        Philosopher {
            name: name.to_string()
        }
    }

    fn eat(&self) {
        println!("{} is eating.", self.name);
        thread::sleep_ms(1000);
        println!("{} is done eating.", self.name)
    }
}

fn main() {
    let philosophers = vec![
        Philosopher::new("Immanuel Kant"),
        Philosopher::new("Martin Heidegger"),
        Philosopher::new("Jean-Paul Sartre"),
        Philosopher::new("Søren Kierkegaard"),
        Philosopher::new("Friedrich Nietzsche")
    ];

    for p in &philosophers {
        p.eat()
    }
}
