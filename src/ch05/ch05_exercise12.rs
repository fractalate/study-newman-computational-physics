use std::f64::consts::PI;

use rust_newman_computational_physics::utils::integrate::integrate_gaussian_quadrature::{
  integrate_gaussian_quadrature,
  integrate_gaussian_quadrature_adaptive,
};

const BOLTZMANNS_CONSTANT: f64 = 1.380649e-23; // measured in J/K
const SPEED_LIGHT: f64 = 299792458.0; // measured in m/s
const PLANCK_CONSTANT_BAR: f64 = 1.054571817e-34; // J*s

// We're evaluating the integral of x^3/(e^x-1) from 0 to infinity, but we'll do a substitutions
// to control the domain of integration so we can use Gaussian quadrature.
//
// Let x = t/(1-t), then dx = 1/(1-t)^2 and our integral goes from 0 to 1.
//
// Then instead of the original integrand, we integrate
//
// (t/(1-t))^3 / ( (e(t/(1-t))-1) * (1-t)^2 )
fn integrand(t: f64) -> f64 {
  let one_minus_t = 1.0 - t;
  let t_over_one_minus_t = t / one_minus_t;
  let e_to_the_x_minus_one = t_over_one_minus_t.exp() - 1.0;

  t_over_one_minus_t.powi(3) / (e_to_the_x_minus_one * one_minus_t * one_minus_t)
}

fn evaluate_integral(n: usize) -> f64 {
  integrate_gaussian_quadrature(0.0, 1.0, n, integrand)
}

fn evaluate_integral_adaptive(epsilon: f64) -> f64 {
  integrate_gaussian_quadrature_adaptive(0.0, 1.0, epsilon, integrand)
}

fn approximate_stefan_boltzmann_constant(n: usize) -> f64 {
  let pi_squared = PI * PI;
  let c_squared = SPEED_LIGHT * SPEED_LIGHT;
  let h_bar_cubed = PLANCK_CONSTANT_BAR * PLANCK_CONSTANT_BAR * PLANCK_CONSTANT_BAR;
  let kb_squared = BOLTZMANNS_CONSTANT * BOLTZMANNS_CONSTANT;
  let kb_tetrated = kb_squared * kb_squared;

  kb_tetrated / (4.0 * pi_squared * c_squared * h_bar_cubed) * evaluate_integral(n)
}

fn ch05_exercise12_b() -> Result<(), Box<dyn std::error::Error>> {
  let n = 50;
  let exact = PI.powi(4) / 15.0; // We know what it should be!

  let result = evaluate_integral(n);
  let result_next = evaluate_integral(2*n); // Just for the error analysis.

  let expected_error = (result_next - result).abs();

  println!("b) The integral of x^3/(e^x-1) from 0 to infinity is:");
  println!();
  println!("      {result}");
  println!();
  println!("    To evaluate, we used a substition to make a finite domain of integration");
  println!("    ranging from 0 to 1. And then we evaluated the integral with Gaussian quadrature");
  println!("    with {n} samples.");
  println!();
  println!("    Our value is off by about {expected_error:e} based on successive approximations.");
  println!();

  let error = (exact - result).abs();

  println!("    Our value is actually off by {error:e} based on the exact value pi^4/15.");
  println!();

  let result_adapative = evaluate_integral_adaptive(1.0e-15);
  let error_adaptive = (exact - result_adapative).abs();

  println!("    Using an adaptive method, we get:");
  println!();
  println!("      {result_adapative}");
  println!();
  println!("    This new value is off by {error_adaptive:e} based on the exact value pi^4/15.");
  println!();

  Ok(())
}

fn ch05_exercise12_c() -> Result<(), Box<dyn std::error::Error>> {
  let n = 50;
  let sigma = approximate_stefan_boltzmann_constant(n);
  let actual = 5.670374419e-8;
  let error = (sigma - actual).abs();

  println!("c) Stefan-Boltzmann Constant (calculated with {n} slices):");
  println!();
  println!("      {sigma:e} J/s/m^2/K^4");
  println!();
  println!("    The actual value is listed as {actual} on Wikipedia, which is off by {error:e}.");
  println!();

  Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  ch05_exercise12_b()?;
  ch05_exercise12_c()?;

  Ok(())
}

/*
b) The integral of x^3/(e^x-1) from 0 to infinity is:

      6.493939400514871

    To evaluate, we used a substition to make a finite domain of integration
    ranging from 0 to 1. And then we evaluated the integral with Gaussian quadrature
    with 50 samples.

    Our value is off by about 1.7519576900326683e-9 based on successive approximations.

    Our value is actually off by 1.7519576900326683e-9 based on the exact value pi^4/15.

    Using an adaptive method, we get:

      6.49393940226683

    This new value is off by 8.881784197001252e-16 based on the exact value pi^4/15.

c) Stefan-Boltzmann Constant (calculated with 50 slices):

      5.6703744280776996e-8 J/s/m^2/K^4

    The actual value is listed as 0.00000005670374419 on Wikipedia, which is off by 9.07769974132762e-17.

*/
