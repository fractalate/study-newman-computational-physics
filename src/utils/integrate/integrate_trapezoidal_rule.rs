pub fn integrate_trapezoidal_rule<F>(a: f64, b: f64, n: usize, f: F) -> f64
  where F: Fn(f64) -> f64
{
  let h = (b - a) / (n as f64);

  _integrate_trapezoidal_rule(a, b, n, h, f)
}

pub fn _integrate_trapezoidal_rule<F>(a: f64, b: f64, n: usize, h: f64, f: F) -> f64
  where F: Fn(f64) -> f64
{
  let mut total = 0.0;

  total += f(a);

  for i in 1..n {
    let x = a + h*(i as f64);
    total += 2.0*f(x);
  }

  total += f(b);
  total *= h / 2.0;

  total
}

// See section 5.3 titled Choosing the Number of Steps which talks about this adaptive method.
pub fn integrate_trapezoidal_rule_adaptive<F>(a: f64, b: f64, epsilon: f64, f: F) -> f64
  where F: Fn(f64) -> f64
{
  let mut n: usize = 1<<10;
  let mut h = (b - a) / (n as f64);

  // Begin with an initial approximation.
  let mut approximation1 = _integrate_trapezoidal_rule(a, b, n, h, &f);
  let mut approximation2: f64 = 0.0;

  while n <= (1<<28) {
    // Then calculate the next approximation by adding in samples which are between the
    // previous approximation's samples.
    n *= 2;
    h /= 2.0;
    approximation2 = approximation1 / 2.0 + h * adaptive_sum_trapezoidal_rule(a, n, h, &f);

    // If the approximation is within our error bounds, we have our answer.
    // See section 5.2.1, equation (5.28).
    let error = (approximation2 - approximation1) / 3.0;
    if error.abs() < epsilon {
      break;
    }

    approximation1 = approximation2;
  }

  approximation2
}

fn adaptive_sum_trapezoidal_rule<F>(a: f64, n: usize, h: f64, f: F) -> f64
  where F: Fn(f64) -> f64
{
  let mut total = 0.0;

  for i in (1..n).step_by(2) {
    let x = a + h*(i as f64);
    total += f(x);
  }

  total
}

pub fn integrate_romberg_adaptive<F>(a: f64, b: f64, epsilon: f64, f: F) -> f64
  where F: Fn(f64) -> f64
{
  let mut n: usize = 1;
  let mut h = (b - a) / (n as f64);

  // Begin with an initial approximation.
  let mut approximation1 = _integrate_trapezoidal_rule(a, b, n, h, &f);
  let mut rs1: Vec<f64> = Vec::new();
  let mut rs2: Vec<f64> = Vec::new();
  rs1.push(approximation1);

  while n <= (1<<28) {
    // Then calculate the next approximation by adding in samples which are between the
    // previous approximation's samples.
    n *= 2;
    h /= 2.0;
    let approximation2 = approximation1 / 2.0 + h * adaptive_sum_trapezoidal_rule(a, n, h, &f);
    rs2.push(approximation2);

    // Then calculate the Romberg approximations by considering the new approximation and
    // the previous set of Romberg approximations.
    let mut divisor = 0.0;
    for k in 1..=rs1.len() {
      divisor = (divisor + 1.0) * 4.0 - 1.0;
      let rkj = rs2[k-1] + (rs2[k-1] - rs1[k-1]) / divisor;

      rs2.push(rkj);
    }

    // See section 5.4, equation (5.49).
    let error = (rs2[rs2.len() - 2] - rs1[rs2.len() - 2] ) / divisor;

    if error.abs() < epsilon {
      break; // rs2 will have the approximation.
    }

    approximation1 = approximation2;
    std::mem::swap(&mut rs1, &mut rs2);
    rs2.clear();
  }

  rs2[rs2.len() - 1]
}

#[test]
fn test_integrate_trapezoidal_rule_adaptive() {
  let f = |x: f64| x*x*x*x - 2.0*x + 1.0;
  let a = 0.0;
  let b = 2.0;
  let exact = 4.4;

  let mut epsilon = 0.01;
  for _ in 0..10 {
    let approx = integrate_trapezoidal_rule_adaptive(a, b, epsilon, &f);
    assert!((approx - exact).abs() < epsilon);

    epsilon /= 10.0;
  }
}

#[test]
fn test_integrate_romberg_adaptive() {
  let f = |x: f64| x*x*x*x - 2.0*x + 1.0;
  let a = 0.0;
  let b = 2.0;
  let exact = 4.4;

  let mut epsilon = 0.01;
  for _ in 0..10 {
    let approx = integrate_romberg_adaptive(a, b, epsilon, &f);
    assert!((approx - exact).abs() < epsilon);

    epsilon /= 10.0;
  }
}
