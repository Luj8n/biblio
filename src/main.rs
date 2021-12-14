use biblio::large_num::{LargeNum, ToLarge};
use std::time::Instant;

fn main() {
  let current_time = Instant::now();
  let a = "2".to_large();

  let power = 100000;
  println!("{0} ^ {1} = {2}", a.clone(), power, a.pow(power).to_string().len());

  let duration = current_time.elapsed();
  println!("{:?}", duration);
}
