use std::thread;
use std::cell::Cell;

fn main() {
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);
    println!("Hello, from main thread!");

    t1.join().unwrap();
    t2.join().unwrap();
    is_some_and();
    cell(&Cell::new(1), &Cell::new(1));
}

fn f() {
    println!("Hello from another thread!");

    let id = thread::current().id();
    println!("This is my thread id: {id:?}");
}

fn is_some_and() {
    let raku = Some("Perl 6");
    if raku.is_some_and(|x| x.contains("Perl")) {
        println!("hey, Perl 6");
    }
}

fn cell(a: &Cell<i32>, b: &Cell<i32>) {
    let before = a.get();
    b.set(b.get() + 1);
    let after = a.get();
    if before != after {
        is_some_and();
    }
}

fn test() {
    let bananas: f64 = 3.0;
    let apes: f64 = 2.0;

    println!("bananas={bananas} apes={apes}, {} bananas per ape", bananas/apes)
}