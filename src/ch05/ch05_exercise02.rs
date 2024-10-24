fn integrand(x: f64) -> f64 {
  x*x*x*x - 2.0*x + 1.0
}

// todo make the function take a function object that we can pass
// todo move this function somewhere more common so I can just use it when I need it
fn integrate_simpsons_rule(a: f64, b: f64, n: usize) -> f64 {
  let h = (b - a) / (n as f64);
  let mut total = 0.0;

  total += integrand(a);
  for i in 1..n {
    let x = a + h*(i as f64);
    if i % 2 == 0 {
      total += 2.0*integrand(x);
    } else {
      total += 4.0*integrand(x);
    }
  }
  total += integrand(b);
  total *= h / 3.0;

  total
}

fn main() {
  // f(x) = x^4 - 2x + 1
  // F(x) = (x^5)/5 - x^2 + x + C
  // integral from 0 to 2 of f(x) is 32/5 - 4 + 2 = 4.4
  let exact_answer = 4.4;

  let with_10_slices = integrate_simpsons_rule(0.0, 2.0, 10);
  let err_with_10_slices = (exact_answer - with_10_slices).abs();
  let frac_err_with_10_slices = err_with_10_slices / with_10_slices;
  println!("{} (+/- {} from actual, fraction error is {})", with_10_slices, err_with_10_slices, frac_err_with_10_slices);

  let with_100_slices = integrate_simpsons_rule(0.0, 2.0, 100);
  let err_with_100_slices = (exact_answer - with_100_slices).abs();
  let frac_err_with_100_slices = err_with_100_slices / with_100_slices;
  println!("{} (+/- {} from actual, fraction error is {})", with_100_slices, err_with_100_slices, frac_err_with_100_slices);

  let with_1000_slices = integrate_simpsons_rule(0.0, 2.0, 1000);
  let err_with_1000_slices = (exact_answer - with_1000_slices).abs();
  let frac_err_with_1000_slices = err_with_1000_slices / with_1000_slices;
  println!("{} (+/- {} from actual, fraction error is {})", with_1000_slices, err_with_1000_slices, frac_err_with_1000_slices);

  // todo do problem 5.1 to do the comparisons with trapezoidal rule.
  // todo finish the problem and mark the problem parts a, b, and c.
  // todo clean this whole thing up
}
