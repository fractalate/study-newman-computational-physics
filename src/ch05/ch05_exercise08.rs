// This is a copy of the integrate_simpsons_rule_adaptive implementation.
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

// This is a copy of the integrate_simpsons_rule_adaptive implementation.
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

// This is a copy of the integrate_simpsons_rule_adaptive implementation.
// Now with logging.
pub fn integrate_simpsons_rule_adaptive<F>(a: f64, b: f64, epsilon: f64, f: F) -> f64
  where F: Fn(f64) -> f64
{
  // This base case starts at 2.
  let mut n = 2;
  let mut h = (a + b) / (n as f64);

  let mut s1 = (f(a) + f(b) + 2.0 * add_evens_from_2(a, h, n, &f)) / 3.0;
  let mut t1 = add_odds_from_1(a, h, n, &f) * 2.0 / 3.0;
  let mut i1 = h * (s1 + 2.0 * t1);

  println!("    Number of posts: {}, Estimate of Integral: {}, Estimated Error: --", n, i1);

  while n <= (1<<24) {
    h /= 2.0;
    n *= 2;

    let s2 = s1 + t1;
    let t2 = add_odds_from_1(a, h, n, &f) * 2.0 / 3.0;
    let i2 = h * (s2 + 2.0 * t2);

    let error = (i2 - i1) / 15.0;

    println!("    Number of posts: {}, Estimate of Integral: {}, Estimated Error: {}", n, i2, error);

    i1 = i2;
    s1 = s2;
    t1 = t2;

    if error.abs() < epsilon {
      break
    }
  }

  println!("    Answer {}", i1);

  i1
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  // f(x) = ( sin(sqrt(100*x)) )^2
  // Integral from 0 to 1 is expected to be around 0.45.
  let integrand = |x: f64| (x*100.0).sqrt().sin().powi(2);
  let a = 0.0;
  let b = 1.0;

  println!("a) Integral of ( sin(sqrt(100*x)) )^2 from {} to {} with Simpson's rule:", a, b);
  println!();
  integrate_simpsons_rule_adaptive(a, b, 1.0e-6, integrand);
  println!();
  println!("    Notice that the convergence is quicker than trapezoidal rule, but slower than");
  println!("    Romberg integration from chapter 5 exercise 7.");
  println!();

  Ok(())
}

/*
a) Integral of ( sin(sqrt(100*x)) )^2 from 0 to 1 with Simpson's rule:

    Number of posts: 2, Estimate of Integral: 0.38431604889308213, Estimated Error: --
    Number of posts: 4, Estimate of Integral: 0.5746331650289503, Estimated Error: 0.012687807742391215
    Number of posts: 8, Estimate of Integral: 0.36656898106322056, Estimated Error: -0.013870945597715319
    Number of posts: 16, Estimate of Integral: 0.4391386762335798, Estimated Error: 0.004837979678023951
    Number of posts: 32, Estimate of Integral: 0.45451843128504427, Estimated Error: 0.0010253170034309625
    Number of posts: 64, Estimate of Integral: 0.45574568635801116, Estimated Error: 0.00008181700486445954
    Number of posts: 128, Estimate of Integral: 0.45582702875861086, Estimated Error: 0.0000054228267066469545
    Number of posts: 256, Estimate of Integral: 0.45583218714672064, Estimated Error: 0.00000034389254065144335
    Answer 0.45583218714672064

    Notice that the convergence is quicker than trapezoidal rule, but slower than
    Romberg integration from chapter 5 exercise 7.

*/
