fn main() {
    use std::cmp::{min,max};
    let least = min(7,1);
    println!("{}", least);

    let maxiam = max(33,42);
    println!("{}", maxiam);

    // Types are namespaces too
    let x = str::len("amos");
    println!("{}", x);

    // `Vec` is a regular struct, not a primitive type
    let mut v = Vec::new();
    v.push(1);
    v.push(5);
    v.push(6);

    for i in v {
        println!("{}", i);
    }
}
