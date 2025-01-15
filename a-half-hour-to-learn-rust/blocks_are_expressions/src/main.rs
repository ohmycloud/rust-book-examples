fn main() {
    let x = { 42 };    // blocks are also expressions, which mean they evaluate to a value
    println!("{}", x); // 42

    let x = {          // this `x` shadow the previous variable binding
        let y = 1;     // first statement
        let z = 2;     // second statement
        y + z          // this is the *tail* - what the whole block will evaluate to
    };
    println!("{}", x); // 3
    
    let lucky_number = fair_dice_roll_with_if(); // `if` conditions are also expression
    println!("{}", lucky_number);

    let lucky_number = fair_dice_roll_with_match(); // `match` is also an expression
    println!("{}", lucky_number);
}

fn fair_dice_roll_with_if() -> i32 {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let i: i32 = rng.gen();
    if i > 0 {
        42
    } else {
        6
    }
}

fn fair_dice_roll_with_match() -> i32 {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let roll: i32 = rng.gen_range(1, 7);
    if roll > 3 {
        42
    } else {
        6
    }
}
