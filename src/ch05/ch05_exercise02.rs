use rust_newman_computational_physics::utils::integrate_simpsons_rule::integrate_simpsons_rule;
use rust_newman_computational_physics::utils::integrate_trapazoidal_rule::integrate_trapazoidal_rule;

fn main() {
  // f(x) = x^4 - 2x + 1
  // F(x) = (x^5)/5 - x^2 + x + C
  // integral from 0 to 2 of f(x) is 32/5 - 4 + 2 = 4.4
  let integrand = |x: f64| x*x*x*x - 2.0*x + 1.0;
  let a = 0.0;
  let b = 2.0;
  let exact_answer = 4.4;

  let integral_with_10_slices = integrate_simpsons_rule(a, b, 10, integrand);
  println!("a) Integral of x^4 - 2x + 1 from {} to {} using Simpson's rule", a, b);
  println!("   with 10 slices gives {}.", integral_with_10_slices);
  println!();

  let fraction_error_with_10_slices = (integral_with_10_slices - exact_answer).abs() / exact_answer;
  println!("b) The exact answer should be {}, but our approximation with 10 slices", exact_answer);
  println!("   has a fraction error of {:e}.", fraction_error_with_10_slices);
  println!();

  let integral_with_100_slices_simpsons = integrate_simpsons_rule(a, b, 100, integrand);
  let fraction_error_with_100_slices_simpsons = (integral_with_100_slices_simpsons - exact_answer).abs() / exact_answer;
  let integral_with_100_slices_trapazoidal = integrate_trapazoidal_rule(a, b, 100, integrand);
  let fraction_error_with_100_slices_trapazoidal = (integral_with_100_slices_trapazoidal - exact_answer).abs() / exact_answer;
  println!("c) 100 slices:");
  println!();
  println!("     Simpson's Rule: {}", integral_with_100_slices_simpsons);
  println!("     Fraction Error: {:e}", fraction_error_with_100_slices_simpsons);
  println!("   Trapazoidal Rule: {}", integral_with_100_slices_trapazoidal);
  println!("     Fraction Error: {:e}", fraction_error_with_100_slices_trapazoidal);
  println!();

  let integral_with_1000_slices_simpsons = integrate_simpsons_rule(a, b, 1000, integrand);
  let fraction_error_with_1000_slices_simpsons = (integral_with_1000_slices_simpsons - exact_answer).abs() / exact_answer;
  let integral_with_1000_slices_trapazoidal = integrate_trapazoidal_rule(a, b, 1000, integrand);
  let fraction_error_with_1000_slices_trapazoidal = (integral_with_1000_slices_trapazoidal - exact_answer).abs() / exact_answer;
  println!("   1000 Slices:");
  println!();
  println!("     Simpson's Rule: {}", integral_with_1000_slices_simpsons);
  println!("     Fraction Error: {:e}", fraction_error_with_1000_slices_simpsons);
  println!("   Trapazoidal Rule: {}", integral_with_1000_slices_trapazoidal);
  println!("     Fraction Error: {:e}", fraction_error_with_1000_slices_trapazoidal);
  println!();
}

/*
a) Integral of x^4 - 2x + 1 from 0 to 2 using Simpson's rule
   with 10 slices gives 4.400426666666667.

b) The exact answer should be 4.4, but our approximation with 10 slices
   has a fraction error of 9.696969696972666e-5.

c) 100 slices:

     Simpson's Rule: 4.400000042666668
     Fraction Error: 9.696969893724372e-9
   Trapazoidal Rule: 4.401066656
     Fraction Error: 2.4242181818179273e-4

   1000 Slices:

     Simpson's Rule: 4.400000000004266
     Fraction Error: 9.695274885953866e-13
   Trapazoidal Rule: 4.4000106666656
     Fraction Error: 2.4242421817452255e-6

*/
