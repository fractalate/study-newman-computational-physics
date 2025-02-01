use std::f64::consts::PI;

use plotters::prelude::*;
use textplots::{Chart, Plot, Shape};
use rust_newman_computational_physics::utils::integrate::integrate_gaussian_quadrature;

const N: usize = 50;

fn integrate_cos_half_phi_t_squared(u: f64) -> f64 {
  let integrand = |t: f64| (PI / 2.0 * t*t).cos();

  integrate_gaussian_quadrature(0.0, u, N, integrand)
}

fn integrate_sin_half_phi_t_squared(u: f64) -> f64 {
  let integrand = |t: f64| (PI / 2.0 * t*t).sin();

  integrate_gaussian_quadrature(0.0, u, N, integrand)
}

fn calculate_fractional_diffractional_intensity(x: f64, z: f64, wavelength: f64) -> f64 {
  let u = x * (2.0 / z / wavelength).sqrt();

  let cos_term = (2.0 * integrate_cos_half_phi_t_squared(u) + 1.0).powi(2);
  let sin_term = (2.0 * integrate_sin_half_phi_t_squared(u) + 1.0).powi(2);

  (cos_term + sin_term) / 8.0
}

fn ch05_exercise11() -> Result<(), Box<dyn std::error::Error>> {
  let a = -5.0;
  let b = 5.0;
  let z = 3.0; // meters
  let wavelength = 1.0; // meters

  println!("A plot of the fractional diffractional intensity from x={a} to x={b}");
  Chart::new(75, 30, a as f32, b as f32)
      .lineplot(&Shape::Continuous(Box::new(
        |t| calculate_fractional_diffractional_intensity(t.into(), z, wavelength) as f32
      )))
      .display();

  // Then a plot rendered to a file.
  let root = BitMapBackend::new("out_ch05_exercise11.png", (640, 480)).into_drawing_area();
  root.fill(&WHITE)?;

  let mut chart = ChartBuilder::on(&root)
      .caption("Fractional Diffractional Intensity I / I0", ("sans-serif", 40).into_font())
      .margin(10)
      .x_label_area_size(30)
      .y_label_area_size(60)
      .build_cartesian_2d(a..b, 0.0..1.5)?;

  chart.configure_mesh()
    .draw()?;

  chart.draw_series(LineSeries::new(
    (0..=200).map(
      |x| {
        let frac = x as f64 / 200.0;
        let t = b * frac + a * (1.0 - frac);

        (t, calculate_fractional_diffractional_intensity(t.into(), z, wavelength))
      }
    ),
    &RED,
  ))?;

  chart.configure_series_labels().draw()?;

  root.present()?;

  println!("   Please see: out_ch05_exercise11.png");
  println!();

  Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  ch05_exercise11()?;

  Ok(())
}

/*
A plot of the fractional diffractional intensity from x=-5 to x=5
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢈⠀⠀⠀⠀⡰⠉⡆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀ 1.4
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠠⠀⠀⠀⢰⠁⠀⠸⡀⠀⢠⠊⡆⠀⡠⢆⠀⡔⡄⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠐⠀⠀⢀⠇⠀⠀⠀⢣⠀⡎⠀⠸⣠⠃⠘⢴⠁⠣⠄
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢈⠀⠀⡜⠀⠀⠀⠀⠀⠙⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠠⠀⡰⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠐⡠⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⢜⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠒⠒⠒⠉⠀⠠⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀ 0.0
-5.0                              5.0

   Please see: out_ch05_exercise11.png

*/
