use rust_newman_computational_physics::utils::integrate_simpsons_rule::integrate_simpsons_rule;

fn main() {
  // f(x) = x^4 - 2x + 1
  // F(x) = (x^5)/5 - x^2 + x + C
  // integral from 0 to 2 of f(x) is 32/5 - 4 + 2 = 4.4
  let integrand = |x: f64| x*x*x*x - 2.0*x + 1.0;
  let a = 0.0;
  let b = 2.0;
  let exact_answer = 4.4;

  let with_10_slices = integrate_simpsons_rule(a, b, 10, integrand);
  let err_with_10_slices = (exact_answer - with_10_slices).abs();
  let frac_err_with_10_slices = err_with_10_slices / with_10_slices;
  println!("{} (+/- {} from actual, fraction error is {})", with_10_slices, err_with_10_slices, frac_err_with_10_slices);

  let with_100_slices = integrate_simpsons_rule(a, b, 100, integrand);
  let err_with_100_slices = (exact_answer - with_100_slices).abs();
  let frac_err_with_100_slices = err_with_100_slices / with_100_slices;
  println!("{} (+/- {} from actual, fraction error is {})", with_100_slices, err_with_100_slices, frac_err_with_100_slices);

  let with_1000_slices = integrate_simpsons_rule(a, b, 1000, integrand);
  let err_with_1000_slices = (exact_answer - with_1000_slices).abs();
  let frac_err_with_1000_slices = err_with_1000_slices / with_1000_slices;
  println!("{} (+/- {} from actual, fraction error is {})", with_1000_slices, err_with_1000_slices, frac_err_with_1000_slices);

  // todo do problem 5.1 to do the comparisons with trapezoidal rule.
  // todo finish the problem and mark the problem parts a, b, and c.
  // todo clean this whole thing up
}
