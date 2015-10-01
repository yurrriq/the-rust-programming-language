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

    // In Vec<_>, _ is a placeholder and Rust can figure it out
    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        thread::spawn(move || {
            p.eat();
        }) // No trailing semicolon makes it an expression (returned)
    }).collect();

    for h in handles {
        h.join().unwrap(); // join() blocks execution until the thread
    }                      // has completed execution
}
