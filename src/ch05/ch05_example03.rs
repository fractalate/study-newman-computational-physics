use rust_newman_computational_physics::utils::integrate::integrate_gaussian_quadrature;

const N: usize = 50;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  // This integrand has had x = z/(1-z) as a substitution.
  let integrand = |z: f64| {
    let one_minus_z_quantity_squared = (1.0 - z).powi(2);
    let z_squared = z.powi(2);
    let exponential = (-z_squared / one_minus_z_quantity_squared).exp();
    exponential / one_minus_z_quantity_squared
  };

  // Our domain of integration from 0 to inf is equivalent to 0 to 1 after the substitution.
  let result = integrate_gaussian_quadrature(0.0, 1.0, N, integrand);

  println!("Integral of e^(-t^2) from 0 to infinity is {result}");
  println!();

  Ok(())
}

/*
Integral of e^(-t^2) from 0 to infinity is 0.8862269254528349

*/
