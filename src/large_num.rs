use std::ops::{Add, Mul, Sub};

type Digit = u8;
type Bigger = u16;
const BASE: Bigger = 9 as Bigger + 1;

#[derive(Clone, Debug)]

pub struct LargeNum {
  raw: Vec<Digit>,
  positive: bool,
}

impl LargeNum {
  pub fn new(value: Vec<Digit>) -> LargeNum {
    assert!(value.iter().all(|v| (*v as Bigger) < BASE));
    let raw: Vec<Digit> = value;

    LargeNum { raw, positive: true }
  }

  pub fn from(string: &str) -> LargeNum {
    todo!()
  }

  // TODO: put this into a trait and optimize it
  fn greater_than(&self, other: &LargeNum) -> bool {
    if self.raw.len() == other.raw.len() {
      let i = (0..self.raw.len())
        .rev()
        .find(|&i| self.raw[i] != other.raw[i])
        .unwrap_or(0);
      self.raw[i] > other.raw[i]
    } else {
      self.raw.len() > other.raw.len()
    }
  }

  fn trim_zeros(self) -> LargeNum {
    let zero_count = self
      .raw
      .iter()
      .skip(1)
      .rev()
      .take_while(|x| **x == 0)
      .collect::<Vec<&Digit>>()
      .len();

    LargeNum {
      raw: self.raw[0..self.raw.len() - zero_count].to_vec(),
      positive: true,
    }
  }

  pub fn change_sign(self) -> LargeNum {
    let mut cp = self;
    cp.positive = !cp.positive;
    cp
  }

  pub fn add(&self, other: &LargeNum) -> LargeNum {
    if self.positive && !other.positive {
      return self.sub(other);
    } else if !self.positive && other.positive {
      return other.sub(self);
    }

    let result = self.ignore_sign_add(other);
    if !self.positive && !other.positive {
      return result.change_sign();
    }
    result
  }

  fn ignore_sign_add(&self, other: &LargeNum) -> LargeNum {
    let (bigger, smaller) = {
      if self.raw.len() > other.raw.len() {
        (&self.raw, &other.raw)
      } else {
        (&other.raw, &self.raw)
      }
    };

    let mut new_num: Vec<Digit> = vec![];
    let mut carry: Bigger = 0; // 0 or 1

    for i in 0..=bigger.len() {
      if i < smaller.len() {
        let t = bigger[i] as Bigger + smaller[i] as Bigger + carry;
        new_num.push((t % BASE) as Digit);
        carry = t / BASE;
      } else if i == bigger.len() {
        if carry == 1 {
          new_num.push(1);
        }
      } else {
        let t = bigger[i] as Bigger + carry;
        new_num.push((t % BASE) as Digit);
        carry = t / BASE;
      }
    }

    LargeNum {
      raw: new_num,
      positive: true,
    }
  }

  pub fn sub(&self, other: &LargeNum) -> LargeNum {
    let (bigger_num, smaller_num) = {
      if self.greater_than(&other) {
        (self, other)
      } else {
        (self, other)
      }
    };

    if bigger_num.positive && !smaller_num.positive {
      return bigger_num.ignore_sign_add(smaller_num);
    } else if !bigger_num.positive && smaller_num.positive {
      return bigger_num.ignore_sign_add(smaller_num).change_sign();
    }
    let result = self.ignore_sign_sub(other);
    if !self.positive && !other.positive {
      return result.change_sign();
    }
    result
  }

  fn ignore_sign_sub(&self, other: &LargeNum) -> LargeNum {
    let (bigger, smaller) = {
      if self.greater_than(other) {
        (&self.raw, &other.raw)
      } else {
        (&other.raw, &self.raw)
      }
    };

    let mut new_num: Vec<Digit> = vec![];
    let mut carry: Bigger = 0; // 0 or 1

    for i in 0..bigger.len() {
      if i < smaller.len() {
        new_num.push(((bigger[i] as Bigger + BASE - carry - smaller[i] as Bigger) % BASE) as Digit);
        carry = if smaller[i] as Bigger + carry > bigger[i] as Bigger {
          1
        } else {
          0
        };
      } else {
        new_num.push(((bigger[i] as Bigger + BASE - carry) % BASE) as Digit);
        carry = if bigger[i] as Bigger == 0 && carry == 1 { 1 } else { 0 };
      }
    }

    LargeNum {
      raw: new_num,
      positive: true,
    }
    .trim_zeros()
  }

  pub fn mul(&self, other: &LargeNum) -> LargeNum {
    return self.ignore_sign_mul(other);
  }

  fn ignore_sign_mul(&self, other: &LargeNum) -> LargeNum {
    let (bigger, smaller) = {
      if self.raw.len() > other.raw.len() {
        (&self.raw, &other.raw)
      } else {
        (&other.raw, &self.raw)
      }
    };

    let mut added_together: LargeNum = LargeNum::default();

    for i in 0..smaller.len() {
      let mut temp_num: Vec<Digit> = vec![];
      let mut carry: Bigger = 0; // 0 or 1

      for j in 0..=bigger.len() {
        if j == bigger.len() {
          if carry != 0 {
            temp_num.push(carry as Digit);
          }
          break;
        }
        let t = bigger[j] as Bigger * smaller[i] as Bigger + carry;
        temp_num.push((t % BASE) as Digit);
        carry = t / BASE;
      }

      temp_num.splice(0..0, [0].repeat(i));

      added_together = added_together
        + LargeNum {
          raw: temp_num,
          positive: true,
        };
    }

    added_together
  }

  pub fn pow(self, power: u128) -> LargeNum {
    let mut num = self;
    let mut power = power;
    let mut result = LargeNum::new(vec![1]);

    while power > 0 {
      if power & 1 == 1 {
        result = (&result).mul(&num);
      }
      power >>= 1;
      num = (&num).mul(&num);
    }
    return result;
  }
}

impl Default for LargeNum {
  fn default() -> LargeNum {
    LargeNum {
      raw: vec![0],
      positive: true,
    }
  }
}

impl ToString for LargeNum {
  fn to_string(&self) -> String {
    // TODO: for all bases
    let mut s = String::new();
    if !self.positive {
      s += "-";
    }
    for num in &self.raw.iter().rev().collect::<Vec<&Digit>>() {
      s += &num.to_string();
      // s += " ";
    }
    s.trim_end().to_owned()
  }
}

impl Add for LargeNum {
  type Output = LargeNum;

  fn add(self, other: LargeNum) -> LargeNum {
    (&self).add(&other)
  }
}

impl Sub for LargeNum {
  type Output = LargeNum;

  fn sub(self, other: LargeNum) -> LargeNum {
    (&self).sub(&other)
  }
}

impl Mul for LargeNum {
  type Output = LargeNum;

  fn mul(self, other: LargeNum) -> LargeNum {
    (&self).mul(&other)
  }
}

impl PartialEq for LargeNum {
  fn eq(&self, other: &Self) -> bool {
    self.raw == other.raw && self.positive == other.positive
  }
}

mod tests {
  use super::*;
  #[test]
  fn add() {
    let a = LargeNum::new(vec![9, 9, 9, 9]);
    let b = LargeNum::new(vec![9, 9, 9]);
    assert_eq!(a + b, LargeNum::new(vec![8, 9, 9, 0, 1]));

    let a = LargeNum::new(vec![9, 9, 9, 9]).change_sign();
    let b = LargeNum::new(vec![9, 9, 9]).change_sign();
    assert_eq!(a + b, LargeNum::new(vec![8, 9, 9, 0, 1]).change_sign());

    let a = LargeNum::new(vec![8, 9, 9, 9]);
    let b = LargeNum::new(vec![9, 9, 9]).change_sign();
    a - b;
    // assert_eq!(a + b, LargeNum::new(vec![9, 9, 9, 8]));
  }
}
