fn main() {
  let x = vec![1,2,3,4,5,6,7,8,9,10]
      .iter()
      .map(|x| x + 3)
      .fold(0, |x, y| x + y);

  println!("{}",x);
}
