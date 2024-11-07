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
