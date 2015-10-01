use std::thread;
use std::sync::{Arc, Mutex}; // Arc == 'atomic reference count'

struct Philosopher {
    name:  String,
    left:  usize,
    right: usize
}

// new() is an 'associated function' on Philosopher
// Since Rust is expression-based, the last/only expression,
// instantiating a Philosopher, is returned.
impl Philosopher {
    // usize is the type with which vectors are indexed
    // left and right are indexes of the forks of the Table
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name:  name.to_string(),
            left:  left,
            right: right
        }
    }

    fn eat(&self, table: &Table) {
        // lock() might fail. The mutex might be poisoned, which is when the
        // thread panics while the lock is held. Since it shouldn't happend,
        // we just unwrap() it.
        //
        // The underscore prefix is to ignore the values, since we don't care
        // about the value, but rather just want to acquire the lock.
        //
        // The locks will be released automatically when _left and _right
        // go out of scope.
        let _left  = table.forks[self.left].lock().unwrap();
        let _right = table.forks[self.right].lock().unwrap();

        println!("{} is eating.", self.name);

        thread::sleep_ms(1000);

        println!("{} is done eating.", self.name)
    }
}

struct Table {
    forks: Vec<Mutex<()>>
}


fn main() {
    // Wrapped in Arc<T> to enable sharing across threads.
    // The count will increment when shared in a thread,
    // and decrement when the thread ends.
    let table = Arc::new(Table { forks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(())
    ]});

    // Nietzsche's `0, 4` prevents a deadlock and
    // signifies that he's lef-thanded.
    //
    // The author of the tutorial thinks this is
    // the simplest solution to the problem.
    let philosophers = vec![
        Philosopher::new("Immanuel Kant",       0, 1),
        Philosopher::new("Martin Heidegger",    1, 2),
        Philosopher::new("Jean-Paul Sartre",    2, 3),
        Philosopher::new("Søren Kierkegaard",   3, 4),
        Philosopher::new("Friedrich Nietzsche", 0, 4)
    ];

    // In Vec<_>, _ is a placeholder and Rust can figure it out
    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        // clone() is what increments the reference count.
        // When table goes out of scope, it decrements the count.
        // If we didn't have a count, we wouldn't know how to deallocate  it.
        //
        // Here we shadow table instead of coming up with a new name.
        let table = table.clone();

        thread::spawn(move || {
            p.eat(&table);
        }) // No trailing semicolon makes it an expression (returned)
    }).collect();

    for h in handles {
        h.join().unwrap(); // join() blocks execution until the thread
    }                      // has completed execution
}
