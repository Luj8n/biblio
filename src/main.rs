use biblio::large_num::LargeNum;
use std::time::Instant;

fn main() {
  let current_time = Instant::now();
  let a = LargeNum::new(vec![2]);
  let b = LargeNum::new(vec![2]);

  // let a = LargeNum::new(vec![8, 9, 9, 9]);
  // let b = LargeNum::new(vec![9, 9, 9]).change_sign();
  // a - b;

  // println!("{0}", a.trim_zeros().to_string());
  // println!("{0} + {1} = {2}", a.to_string(), b.to_string(), (a + b).to_string());
  // println!("{0} - {1} = {2}", a.to_string(), b.to_string(), (a - b).to_string());
  // println!("{0} * {1} = {2}", a.to_string(), b.to_string(), (a * b).to_string());
  let power = 10000;
  println!(
    "{0} ^ {1} = {2}",
    a.to_string(),
    power,
    (a.pow(power)).to_string()
  );

  let duration = current_time.elapsed();
  println!("{:?}", duration);
}
