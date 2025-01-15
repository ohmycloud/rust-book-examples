fn for_each_planet<F>(f: F)
    where F: Fn(&'static str) 
{
    f("Earth");
    f("Mars");
    f("Jupiter");
 }

fn main() {
    for_each_planet(|planet| println!("Hello, {}", planet));
}
