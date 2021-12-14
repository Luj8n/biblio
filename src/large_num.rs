// TODO: write tests
// TODO: add 'Div' function
// TODO: make operations work with primitive types
// TODO: write benchmarks

use std::{
  cmp::Ordering::{Equal, Greater, Less},
  fmt::Display,
  ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

#[derive(Clone, Debug)]

pub struct LargeNum {
  digits: Vec<u8>,
  positive: bool,
}

impl LargeNum {
  pub fn new(digits: Vec<u8>, positive: bool) -> LargeNum {
    assert!(digits.iter().all(|&v| (v as u16) < 10));

    LargeNum { digits, positive }.trim_zeros()
  }

  pub fn from(string: &str) -> LargeNum {
    assert!(!string.is_empty());

    let (string, positive) = {
      if let Some(stripped) = string.strip_prefix('-') {
        (stripped, false)
      } else {
        (string, true)
      }
    };

    let digits: Vec<u8> = string
      .chars()
      .map(|x| x.to_digit(10).expect("All characters must be digits (0..=9)") as u8)
      .rev()
      .collect();

    LargeNum { digits, positive }.trim_zeros()
  }

  pub fn zero() -> LargeNum {
    LargeNum {
      digits: vec![0],
      positive: true,
    }
  }

  pub fn one() -> LargeNum {
    LargeNum {
      digits: vec![1],
      positive: true,
    }
  }

  pub fn signnum(&self) -> i8 {
    if self.digits == vec![0] {
      0
    } else if self.positive {
      1
    } else {
      -1
    }
  }

  pub fn change_sign(self) -> LargeNum {
    let mut large_num = self;
    large_num.positive = !large_num.positive;
    large_num
  }

  pub fn abs(self) -> LargeNum {
    let mut large_num = self;
    large_num.positive = true;
    large_num
  }

  fn trim_zeros(self) -> LargeNum {
    let zero_count = self.digits.iter().skip(1).rev().take_while(|x| **x == 0).count();

    LargeNum {
      digits: self.digits[0..self.digits.len() - zero_count].to_vec(),
      positive: self.positive,
    }
  }

  fn unsafe_cmp(&self, other: &LargeNum) -> Option<std::cmp::Ordering> {
    if self.digits.len() == other.digits.len() {
      let i = (0..self.digits.len())
        .rev()
        .find(|&i| self.digits[i] != other.digits[i])
        .unwrap_or(0);

      match self.digits[i] == other.digits[i] {
        true => Some(Equal),
        false => match self.digits[i] < other.digits[i] {
          true => Some(Less),
          false => Some(Greater),
        },
      }
    } else {
      match self.digits.len() < other.digits.len() {
        true => Some(Less),
        false => Some(Greater),
      }
    }
  }

  pub fn add(&self, other: &LargeNum) -> LargeNum {
    let (bigger_num, smaller_num) = {
      if self.clone().abs() > other.clone().abs() {
        (self, other)
      } else {
        (other, self)
      }
    };

    match (bigger_num.signnum(), smaller_num.signnum()) {
      (0, 0) => LargeNum::zero(),
      (_, 0) => bigger_num.clone(),
      (0, _) => smaller_num.clone(),
      (1, 1) => bigger_num.unsafe_add(smaller_num),
      (-1, -1) => bigger_num.unsafe_add(smaller_num).change_sign(),
      (1, -1) => bigger_num.unsafe_sub(smaller_num),
      (-1, 1) => bigger_num.unsafe_sub(smaller_num).change_sign(),
      _ => panic!(),
    }
  }

  fn unsafe_add(&self, other: &LargeNum) -> LargeNum {
    let (bigger, smaller) = (&self.digits, &other.digits);

    let mut new_digits: Vec<u8> = vec![];
    let mut carry: u16 = 0; // 0 or 1

    for i in 0..=bigger.len() {
      if i < smaller.len() {
        let t = bigger[i] as u16 + smaller[i] as u16 + carry;
        new_digits.push((t % 10) as u8);
        carry = t / 10;
      } else if i == bigger.len() {
        if carry == 1 {
          new_digits.push(1);
        }
      } else {
        let t = bigger[i] as u16 + carry;
        new_digits.push((t % 10) as u8);
        carry = t / 10;
      }
    }

    LargeNum {
      digits: new_digits,
      positive: true,
    }
  }

  pub fn sub(&self, other: &LargeNum) -> LargeNum {
    let (bigger_num, smaller_num) = {
      if self.clone().abs() > other.clone().abs() {
        (self, other)
      } else {
        (other, self)
      }
    };

    match (bigger_num.signnum(), smaller_num.signnum()) {
      (0, 0) => LargeNum::zero(),
      (_, 0) => bigger_num.clone(),
      (0, _) => smaller_num.clone().change_sign(),
      (1, 1) => bigger_num.unsafe_sub(smaller_num),
      (-1, -1) => bigger_num.unsafe_sub(smaller_num).change_sign(),
      (1, -1) => bigger_num.unsafe_add(smaller_num),
      (-1, 1) => bigger_num.unsafe_add(smaller_num).change_sign(),
      _ => panic!(),
    }
  }

  fn unsafe_sub(&self, other: &LargeNum) -> LargeNum {
    let (bigger, smaller) = (&self.digits, &other.digits);

    let mut new_digits: Vec<u8> = vec![];
    let mut carry: u16 = 0; // 0 or 1

    for i in 0..bigger.len() {
      if i < smaller.len() {
        new_digits.push(((bigger[i] as u16 + 10 - carry - smaller[i] as u16) % 10) as u8);
        carry = if smaller[i] as u16 + carry > bigger[i] as u16 {
          1
        } else {
          0
        };
      } else {
        new_digits.push(((bigger[i] as u16 + 10 - carry) % 10) as u8);
        carry = if bigger[i] as u16 == 0 && carry == 1 { 1 } else { 0 };
      }
    }

    LargeNum {
      digits: new_digits,
      positive: true,
    }
    .trim_zeros()
  }

  pub fn mul(&self, other: &LargeNum) -> LargeNum {
    let (bigger_num, smaller_num) = {
      if self.clone().abs() > other.clone().abs() {
        (self, other)
      } else {
        (other, self)
      }
    };

    match (bigger_num.signnum(), smaller_num.signnum()) {
      (_, 0) => LargeNum::zero(),
      (0, _) => LargeNum::zero(),
      (1, 1) => bigger_num.unsafe_mul(smaller_num),
      (-1, -1) => bigger_num.unsafe_mul(smaller_num),
      (1, -1) => bigger_num.unsafe_mul(smaller_num).change_sign(),
      (-1, 1) => bigger_num.unsafe_mul(smaller_num).change_sign(),
      _ => panic!(),
    }
  }

  fn unsafe_mul(&self, other: &LargeNum) -> LargeNum {
    let (bigger, smaller) = (&self.digits, &other.digits);

    let mut large_num_sum: LargeNum = LargeNum::zero();

    for (i, &smaller_digit) in smaller.iter().enumerate() {
      let mut new_digits: Vec<u8> = vec![];
      let mut carry: u16 = 0;

      for j in 0..=bigger.len() {
        if j == bigger.len() {
          if carry != 0 {
            new_digits.push(carry as u8);
          }
          break;
        }
        let t = bigger[j] as u16 * smaller_digit as u16 + carry;
        new_digits.push((t % 10) as u8);
        carry = t / 10;
      }

      new_digits.splice(0..0, [0].repeat(i));

      large_num_sum += LargeNum::new(new_digits, true);
    }

    large_num_sum
  }

  pub fn pow(self, power: u128) -> LargeNum {
    let mut num = self;
    let mut power = power;
    let mut result = LargeNum::one();

    while power > 0 {
      if power & 1 == 1 {
        result = (&result).mul(&num);
      }
      power >>= 1;
      num = (&num).mul(&num);
    }
    result
  }
}

impl Display for LargeNum {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}{}",
      if self.positive { "" } else { "-" },
      self.digits.iter().map(|d| d.to_string()).rev().collect::<String>()
    )
  }
}

impl Add for LargeNum {
  type Output = LargeNum;

  fn add(self, rhs: LargeNum) -> LargeNum {
    (&self).add(&rhs)
  }
}

impl AddAssign for LargeNum {
  fn add_assign(&mut self, rhs: LargeNum) {
    let new_num = self.clone() + rhs;
    self.digits = new_num.digits;
    self.positive = new_num.positive;
  }
}

impl Sub for LargeNum {
  type Output = LargeNum;

  fn sub(self, rhs: LargeNum) -> LargeNum {
    (&self).sub(&rhs)
  }
}

impl SubAssign for LargeNum {
  fn sub_assign(&mut self, rhs: LargeNum) {
    let new_num = self.clone() - rhs;
    self.digits = new_num.digits;
    self.positive = new_num.positive;
  }
}

impl Mul for LargeNum {
  type Output = LargeNum;

  fn mul(self, rhs: LargeNum) -> LargeNum {
    (&self).mul(&rhs)
  }
}

impl MulAssign for LargeNum {
  fn mul_assign(&mut self, rhs: LargeNum) {
    let new_num = self.clone() * rhs;
    self.digits = new_num.digits;
    self.positive = new_num.positive;
  }
}

impl PartialEq for LargeNum {
  fn eq(&self, other: &Self) -> bool {
    self.digits == other.digits && self.signnum() == other.signnum()
  }
}

impl PartialOrd for LargeNum {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    match (self.signnum(), other.signnum()) {
      (0, 0) => Some(Equal),
      (1, 0) => Some(Greater),
      (0, 1) => Some(Less),
      (0, -1) => Some(Greater),
      (-1, 0) => Some(Less),
      (1, -1) => Some(Greater),
      (-1, 1) => Some(Less),
      (-1, -1) => other.unsafe_cmp(self),
      (1, 1) => self.unsafe_cmp(other),
      _ => panic!(),
    }
  }
}

pub trait ToLarge {
  fn to_large(&self) -> LargeNum;
}

impl ToLarge for &str {
  fn to_large(&self) -> LargeNum {
    LargeNum::from(self)
  }
}

mod tests {
  use super::*;

  #[test]
  fn add() {
    let a = "989".to_large();
    let b = "12".to_large();
    let c = (&a).add(&b);
    assert_eq!(c, "1001".to_large());
    assert_eq!(c.to_string(), "1001");

    assert_ne!(a, b);
    assert_ne!(a, c);
    assert_ne!(b, c);

    assert!(a > b);
    assert!(b < a);

    assert!(c > a);
    assert!(a < c);

    assert!(c > b);
    assert!(b < c);
  }
}
