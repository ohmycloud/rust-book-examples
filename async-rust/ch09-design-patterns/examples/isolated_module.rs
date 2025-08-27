use ch09_design_patterns::{get_add, send_add};

fn main() {
    let id = send_add(1, 2).unwrap();
    println!("id: {}", id);
    std::thread::sleep(std::time::Duration::from_secs(4));
    println!("main sleep done");
    let result = get_add(id).unwrap();
    println!("result: {}", result);
}
