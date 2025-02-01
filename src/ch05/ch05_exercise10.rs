use plotters::prelude::*;
use textplots::{Chart, Plot, Shape};
use rust_newman_computational_physics::utils::integrate::integrate_gaussian_quadrature;

const SQRT8: f64 = 2.8284271247461903;

fn calculate_period_of_oscillation(a: f64) -> f64 {
  let potential = |x: f64| x.powi(4);
  let va = potential(a);
  let integrand = |x: f64| (va - potential(x)).sqrt().recip();
  let n = 20;
  let unscaled_approx = integrate_gaussian_quadrature(0.0, a, n, &integrand);

  unscaled_approx * SQRT8
}

fn ch05_exercise10_b() -> Result<(), Box<dyn std::error::Error>> {
  let a = 0.0;
  let b = 2.0;

  println!("b) A plot of the period of the anharmonic oscillation from a={a} to a={b}");
  Chart::new(75, 30, a as f32, b as f32)
      .lineplot(&Shape::Continuous(Box::new(
        |t| calculate_period_of_oscillation(t.into()) as f32
      )))
      .display();

  // Then a plot rendered to a file.
  let root = BitMapBackend::new("out_ch05_exercise10_b.png", (640, 480)).into_drawing_area();
  root.fill(&WHITE)?;

  let mut chart = ChartBuilder::on(&root)
      .caption("Period of the Anharmonic Oscillation", ("sans-serif", 40).into_font())
      .margin(10)
      .x_label_area_size(30)
      .y_label_area_size(60)
      .build_cartesian_2d(a..b, 0.0..350.0)?;

  chart.configure_mesh()
    .draw()?;

  chart.draw_series(LineSeries::new(
    (0..=200).map(
      |x| {
        let frac = x as f64 / 200.0;
        let t = b * frac + a * (1.0 - frac);

        (t, calculate_period_of_oscillation(t))
      }
    ),
    &RED,
  ))?;

  chart.configure_series_labels().draw()?;

  root.present()?;

  println!("   Please see: out_ch05_exercise10_b.png");
  println!();

  Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  ch05_exercise10_b()?;

  Ok(())
}

/*
b) A plot of the period of the anharmonic oscillation from a=0 to a=2
⣹⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀ 136.8
⢼⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⢺⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⡹⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠄⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠂⠸⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⡁⠀⠈⠢⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠄⠀⠀⠀⠀⠈⠉⠉⠒⠒⠒⠒⠒⠒⠒⠒⠒⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠄ 1.8
0.0                               2.0

   Please see: out_ch05_exercise10_b.png

*/
