pub fn integrate_simpsons_rule<F>(a: f64, b: f64, n: usize, f: F) -> f64
  where F: Fn(f64) -> f64
{
  let h = (b - a) / (n as f64);
  let mut total = 0.0;

  total += f(a);

  for i in 1..n {
    let x = a + h*(i as f64);
    if i % 2 == 0 {
      total += 2.0*f(x);
    } else {
      total += 4.0*f(x);
    }
  }

  total += f(b);
  total *= h / 3.0;

  total
}

pub fn add_odds_from_1<F>(a: f64, h: f64, n: usize, f: F) -> f64
  where F: Fn(f64) -> f64
{
  let mut total = 0.0;

  for i in (1..=n-1).step_by(2) {
    let x = a + h*(i as f64);
    total += f(x);
  }

  total
}

pub fn add_evens_from_2<F>(a: f64, h: f64, n: usize, f: F) -> f64
  where F: Fn(f64) -> f64
{
  let mut total = 0.0;

  for i in (2..=n-2).step_by(2) {
    let x = a + h*(i as f64);
    total += f(x);
  }

  total
}

pub fn integrate_simpsons_rule_adaptive<F>(a: f64, b: f64, epsilon: f64, f: F) -> f64
  where F: Fn(f64) -> f64
{
  let mut n = 1<<10;
  let mut h = (a + b) / (n as f64);

  let mut s1 = (f(a) + f(b) + 2.0 * add_evens_from_2(a, h, n, &f)) / 3.0;
  let mut t1 = add_odds_from_1(a, h, n, &f) * 2.0 / 3.0;
  let mut i1 = h * (s1 + 2.0 * t1);

  while n <= (1<<24) {
    h /= 2.0;
    n *= 2;

    let s2 = s1 + t1;
    let t2 = add_odds_from_1(a, h, n, &f) * 2.0 / 3.0;
    let i2 = h * (s2 + 2.0 * t2);

    let error = (i2 - i1) / 15.0;

    i1 = i2;
    s1 = s2;
    t1 = t2;

    if error.abs() < epsilon {
      break
    }
  }

  i1
}

#[test]
fn test_integrate_simpsons_rule_adaptive() {
  let f = |x: f64| x*x*x*x - 2.0*x + 1.0;
  let a = 0.0;
  let b = 2.0;
  let exact = 4.4;

  let mut epsilon = 0.0001;
  for _ in 0..10 {
    let approx = integrate_simpsons_rule_adaptive(a, b, epsilon, &f);
    assert!((approx - exact).abs() < epsilon);

    epsilon /= 10.0;
  }
}
