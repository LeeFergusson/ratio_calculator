/// Represents a list of factors of a given number.
#[derive(Debug, Clone)]
pub struct Factors(pub Vec<u32>);

impl std::fmt::Display for Factors {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "[")?;
    for (i, factor) in self.0.iter().enumerate() {
      if i > 0 {
        write!(f, ", ")?;
      }
      write!(f, "{}", factor)?;
    }
    write!(f, "]")
  }
}

impl IntoIterator for Factors {
  type Item = u32;
  type IntoIter = std::vec::IntoIter<Self::Item>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

#[derive(Debug, Clone, Copy)]
struct Numerator(u32);

impl From<u32> for Numerator {
  fn from(value: u32) -> Self {
    Numerator(value)
  }
}

impl std::fmt::Display for Numerator {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Clone, Copy)]
struct Denominator(u32);

impl From<u32> for Denominator {
  fn from(value: u32) -> Self {
    Denominator(value)
  }
}

impl std::fmt::Display for Denominator {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, Clone, Copy)]
pub struct Ratio {
  numerator: Numerator,
  denominator: Denominator,
}

impl From<(u32, u32)> for Ratio {
  fn from((numerator, denominator): (u32, u32)) -> Self {
    type FloatType = f64;
    let hcf =
      highest_common_factor(factors_of(numerator), factors_of(denominator))
        as FloatType;

    Ratio {
      numerator: Numerator((numerator as FloatType / hcf) as u32),
      denominator: Denominator((denominator as FloatType / hcf) as u32),
    }
  }
}

impl std::fmt::Display for Ratio {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}:{}", self.numerator, self.denominator)
  }
}

/// Returns a list of factors of a given number.
///
/// # Arguments
/// * `n` - The number to find factors of.
///
/// # Returns
/// A `Factors` struct containing a vector of factors.
pub fn factors_of(n: u32) -> Factors {
  let mut factors = Vec::new();

  factors.push(1);
  for i in 2..n {
    let num = n as f64;
    if num % i as f64 == 0.0 {
      factors.push(i);
    }
  }
  factors.push(n);

  Factors(factors)
}

/// Returns a list of common factors of two numbers.
///
/// # Arguments
/// * `a` - The first number.
/// * `b` - The second number.
///
/// # Returns
/// A `Factors` struct containing a vector of common factors.
pub fn common_factors(a: Factors, b: Factors) -> Factors {
  let mut common = Vec::new();

  for factor in a.0 {
    if b.0.contains(&factor) {
      common.push(factor);
    }
  }

  Factors(common)
}

pub fn highest_common_factor(a: Factors, b: Factors) -> u32 {
  common_factors(a, b).into_iter().last().unwrap_or_default()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_factors_of_360() {
    let factors = factors_of(360);
    assert_eq!(
      factors.0,
      vec![
        1, 2, 3, 4, 5, 6, 8, 9, 10, 12, 15, 18, 20, 24, 30, 36, 40, 45, 60, 72,
        90, 120, 180, 360
      ]
    );
  }

  #[test]
  fn test_factors_of_960() {
    let factors = factors_of(960);
    assert_eq!(
      factors.0,
      vec![
        1, 2, 3, 4, 5, 6, 8, 10, 12, 15, 16, 20, 24, 30, 32, 40, 48, 60, 64,
        80, 96, 120, 160, 192, 240, 320, 480, 960
      ]
    );
  }
}
