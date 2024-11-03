use rust_newman_computational_physics::utils::integrate::integrate_simpsons_rule;

fn main() {
  // f(x) = x^4 - 2x + 1
  // F(x) = (x^5)/5 - x^2 + x + C
  // integral from 0 to 2 of f(x) is 32/5 - 4 + 2 = 4.4
  let integrand = |x: f64| x*x*x*x - 2.0*x + 1.0;
  let a = 0.0;
  let b = 2.0;
  let exact_answer = 4.4;

  let integral_with_10_slices = integrate_simpsons_rule(a, b, 10, integrand);
  let integral_with_20_slices = integrate_simpsons_rule(a, b, 20, integrand);
  // In ch05_exercise05.ipynb we found that our error is (I_2 - I_1)/15 for Simpson's Rule.
  let error = (integral_with_20_slices - integral_with_10_slices).abs() / 15.0;
  let exact_error = (integral_with_20_slices - exact_answer).abs();

  println!("a) Integral of x^4 - 2x + 1 from {} to {} using Simpson's rule", a, b);
  println!("   with 10 slices gives {}.", integral_with_10_slices);
  println!();
  println!("   Integral of x^4 - 2x + 1 from {} to {} using Simpson's rule", a, b);
  println!("   with 20 slices gives {}.", integral_with_20_slices);
  println!();
  println!("   Computed error is {:e}", error);
  println!();
  println!("b) Exact error is    {:e}", exact_error);
  println!();

  let error_error = (error - exact_error).abs();
  let exact_answer_plus_error_error = error_error + exact_answer;
  assert!(exact_answer == exact_answer_plus_error_error);
  println!("c) We see that these errors don't agree perfectly, they differ by {:e}.", error_error);
  println!("   Adding the error to the exact exact answer produces a value of {}", exact_answer_plus_error_error);
  println!("   which is still the same exact answer. The discrepency in errors is");
  println!("   due to the limit of precision in each approximation with Simpson's");
  println!("   Rule.");
  println!();
}

/*
a) Integral of x^4 - 2x + 1 from 0 to 2 using Simpson's rule
   with 10 slices gives 4.400426666666667.

   Integral of x^4 - 2x + 1 from 0 to 2 using Simpson's rule
   with 20 slices gives 4.400026666666667.

   Computed error is 2.666666666666373e-5

b) Exact error is    2.6666666666841365e-5

c) We see that these errors don't agree perfectly, they differ by 1.7763636156638285e-16.
   Adding the error to the exact exact answer produces a value of 4.4
   which is still the same exact answer. The discrepency in errors is
   due to the limit of precision in each approximation with Simpson's
   Rule.

*/
