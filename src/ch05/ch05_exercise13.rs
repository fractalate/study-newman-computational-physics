use std::f64::consts::PI;

use plotters::prelude::*;
use textplots::{Chart, Plot, Shape};

use rust_newman_computational_physics::utils::integrate::integrate_gaussian_quadrature;

fn factorial(n: usize) -> f64 {
  if n == 0 {
    return 1.0;
  }

  let mut ttl = 1.0;
  let mut n = n;
  while n > 1 {
    ttl *= n as f64;
    n -= 1;
  }

  ttl
}

fn evaluate_hermite_polynomial(n: usize, x: f64) -> f64 {
  if n == 0 {
    return 1.0;
  }

  let mut h0 = 1.0;
  let mut h1 = 2.0 * x;

  let mut n = n - 1;
  while n > 0 {
    let h2 = 2.0 * x * h1 - 2.0 * (n as f64) * h0;

    h0 = h1;
    h1 = h2;
    n -= 1;
  }

  h1
}

fn evaluate_harmonic_oscillator_wave_function(n: usize, x: f64) -> f64 {
  let e_to_the_negative_x_squared_over_two = (-x*x/2.0).exp();
  let two_to_the_power_n = (1 << n) as f64;
  let n_factorial = factorial(n);
  let sqrt_pi = PI.sqrt();
  let denomiator = (two_to_the_power_n * n_factorial * sqrt_pi).sqrt();

  evaluate_hermite_polynomial(n, x) * e_to_the_negative_x_squared_over_two / denomiator
}

fn evaluate_root_mean_squared_integrand(n: usize, z: f64) -> f64 {
  // We're integrating x^2 * |psi_n(x)|^2 dx from negative infinity to positive infinity.
  // To make it amenable to the computer we do the substitution
  //   x = z / (1-z^2)  and  dx = (1+z^2)/(1-z^2)^2 dz
  // We split the differential's fraction for ease of computation into
  //   dx = 1/(1-z^2)^2 + (z/(1-z^2))^2
  // Then we integrate from -1 to 1.

  let one_minus_z_sqaured = 1.0 - z*z;
  let one_over_one_minus_z_squared = one_minus_z_sqaured.recip();
  let x = z / one_minus_z_sqaured;
  let x_squared = x * x;
  let abs_psi_n_of_x = evaluate_harmonic_oscillator_wave_function(n, x).abs();

  x_squared * abs_psi_n_of_x * abs_psi_n_of_x * (one_over_one_minus_z_squared * one_over_one_minus_z_squared + x_squared)
}

fn evaluate_uncertainty(n: usize) -> f64 {
  integrate_gaussian_quadrature(-1.0, 1.0, 100, |x| evaluate_root_mean_squared_integrand(n, x))
}

fn ch05_exercise13_a() -> Result<(), Box<dyn std::error::Error>> {
  // First a quick plot in the terminal.
  println!("a) A plot of harmonic oscillator wave functions psi_0(x) to psi_3(x)");
  Chart::new(75, 30, -4.0, 4.0)
      .lineplot(&Shape::Continuous(Box::new(|x| evaluate_harmonic_oscillator_wave_function(0, x.into()) as f32)))
      .lineplot(&Shape::Continuous(Box::new(|x| evaluate_harmonic_oscillator_wave_function(1, x.into()) as f32)))
      .lineplot(&Shape::Continuous(Box::new(|x| evaluate_harmonic_oscillator_wave_function(2, x.into()) as f32)))
      .lineplot(&Shape::Continuous(Box::new(|x| evaluate_harmonic_oscillator_wave_function(3, x.into()) as f32)))
      .display();

  // Then a plot rendered to a file.
  let root = BitMapBackend::new("out_ch05_exercise13_a.png", (800, 600)).into_drawing_area();
  root.fill(&WHITE)?;

  let stroke_width = 3;
  let my_blue = RGBColor(31, 119, 180);
  let my_blue_stroke = ShapeStyle {
    color: my_blue.to_rgba(),
    filled: false,
    stroke_width,
  };
  let my_orange = RGBColor(255, 127, 14);
  let my_orange_stroke = ShapeStyle {
    color: my_orange.to_rgba(),
    filled: false,
    stroke_width,
  };
  let my_green = RGBColor(44, 160, 44);
  let my_green_stroke = ShapeStyle {
    color: my_green.to_rgba(),
    filled: false,
    stroke_width,
  };
  let my_magenta = RGBColor(160, 32, 160);
  let my_magenta_stroke = ShapeStyle {
    color: my_magenta.to_rgba(),
    filled: false,
    stroke_width,
  };

  let mut chart = ChartBuilder::on(&root)
    .caption("Harmonic Oscillator Wave Functions psi_n(x)", ("sans-serif", 40).into_font())
    .margin(10)
    .x_label_area_size(30)
    .y_label_area_size(30)
    .build_cartesian_2d(-4.0..4.0, -1.0..1.0)?;

  chart.configure_mesh().draw()?;

  chart.draw_series(LineSeries::new(
    (0..=50).map(|x| (x as f64 / 50.0 * 8.0 - 4.0, evaluate_harmonic_oscillator_wave_function(0, x as f64 / 50.0 * 8.0 - 4.0))),
    my_blue_stroke,
  ))?
    .label("psi_0(x)")
    .legend(|(x, y)| PathElement::new([(x, y), (x + 20, y)], my_blue_stroke));

  chart.draw_series(LineSeries::new(
    (0..=50).map(|x| (x as f64 / 50.0 * 8.0 - 4.0, evaluate_harmonic_oscillator_wave_function(1, x as f64 / 50.0 * 8.0 - 4.0))),
    my_orange_stroke,
  ))?
    .label("psi_1(x)")
    .legend(|(x, y)| PathElement::new([(x, y), (x + 20, y)], my_orange_stroke));

  chart.draw_series(LineSeries::new(
    (0..=50).map(|x| (x as f64 / 50.0 * 8.0 - 4.0, evaluate_harmonic_oscillator_wave_function(2, x as f64 / 50.0 * 8.0 - 4.0))),
    my_green_stroke,
  ))?
    .label("psi_2(x)")
    .legend(|(x, y)| PathElement::new([(x, y), (x + 20, y)], my_green_stroke));

  chart.draw_series(LineSeries::new(
    (0..=50).map(|x| (x as f64 / 50.0 * 8.0 - 4.0, evaluate_harmonic_oscillator_wave_function(3, x as f64 / 50.0 * 8.0 - 4.0))),
    my_magenta_stroke,
  ))?
    .label("psi_3(x)")
    .legend(|(x, y)| PathElement::new([(x, y), (x + 20, y)], my_magenta_stroke));

  chart.configure_series_labels()
    .label_font(("sans-serif", 20).into_font())
    .position(SeriesLabelPosition::UpperRight)
    .draw()?;

  root.present()?;

  println!("   Please see: out_ch05_exercise13_a.png");
  println!();

  Ok(())
}

fn ch05_exercise13_b() -> Result<(), Box<dyn std::error::Error>> {
  // First a quick plot in the terminal.
  println!("a) A plot of harmonic oscillator wave function psi_30(x)");
  Chart::new(75, 30, -10.0, 10.0)
      .lineplot(&Shape::Continuous(Box::new(|x| evaluate_harmonic_oscillator_wave_function(30, x.into()) as f32)))
      .display();

  // Then a plot rendered to a file.
  let root = BitMapBackend::new("out_ch05_exercise13_b.png", (800, 600)).into_drawing_area();
  root.fill(&WHITE)?;

  let stroke_width = 1;
  let my_blue = RGBColor(31, 119, 180);
  let my_blue_stroke = ShapeStyle {
    color: my_blue.to_rgba(),
    filled: false,
    stroke_width,
  };

  let mut chart = ChartBuilder::on(&root)
    .caption("Harmonic Oscillator Wave Function psi_30(x)", ("sans-serif", 40).into_font())
    .margin(10)
    .x_label_area_size(30)
    .y_label_area_size(30)
    .build_cartesian_2d(-10.0..10.0, -1.0..1.0)?;

  chart.configure_mesh().draw()?;

  chart.draw_series(LineSeries::new(
    (0..=250).map(|x| (x as f64 / 250.0 * 20.0 - 10.0, evaluate_harmonic_oscillator_wave_function(30, x as f64 / 250.0 * 20.0 - 10.0))),
    my_blue_stroke,
  ))?
    .label("psi_30(x)")
    .legend(|(x, y)| PathElement::new([(x, y), (x + 20, y)], my_blue_stroke));

  chart.configure_series_labels()
    .label_font(("sans-serif", 20).into_font())
    .position(SeriesLabelPosition::UpperRight)
    .draw()?;

  root.present()?;

  println!("   Please see: out_ch05_exercise13_b.png");
  println!();

  Ok(())
}

fn ch05_exercise13_c() -> Result<(), Box<dyn std::error::Error>> {
  let rms = evaluate_uncertainty(5).sqrt();
  
  println!("c) The quantum uncertainty is calculated to be:");
  println!("    {rms}");
  println!();

  Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  ch05_exercise13_a()?;
  ch05_exercise13_b()?;
  ch05_exercise13_c()?;

  Ok(())
}

/*
a) A plot of harmonic oscillator wave functions psi_0(x) to psi_3(x)
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⡀⠀⠀⢀⡠⠒⢉⠉⠒⢄⣀⠤⢄⢀⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀ 0.8
⠀⠀⠀⠀⠀⠀⠀⠀⢀⠔⠉⠀⠈⢆⠔⡡⠢⡀⠠⠀⢀⠔⠉⢢⡔⠉⣦⠋⠫⡑⢢⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⢀⠔⠁⠀⠀⢀⠔⠊⣞⠀⠀⢱⠐⢀⠎⠀⠀⡜⠑⢲⡁⠑⢄⠈⠢⡑⢄⠀⠀⠀⠀⠀
⠀⠀⢀⣀⡠⠔⠁⣀⣀⠤⠒⠁⠀⡜⠘⡄⠀⠀⢏⡜⠀⠀⡸⠀⢠⠃⠈⠒⠤⣉⣢⢌⣢⣕⣢⣄⣀⠀
⠛⠛⠫⢟⠛⠛⠯⣐⠀⠂⠐⠀⢲⠑⠀⠺⡐⠀⡾⡔⠀⢲⠑⠀⡎⠐⠀⠂⠐⠀⠂⠙⠉⠋⠙⠉⠋⠑
⠀⠀⠀⠀⠉⢢⠀⠀⠑⢄⠀⢀⠇⠀⠀⠀⢣⡰⠑⠸⣀⠇⠀⡜⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠑⢄⠀⠀⢑⢎⠀⠀⠀⠀⡰⠣⣈⣀⠟⢄⡔⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠉⠒⠁⠀⠑⠢⠤⠚⠀⠀⠠⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀ -0.6
-4.0                              4.0

   Please see: out_ch05_exercise13_a.png

a) A plot of harmonic oscillator wave function psi_30(x)
⠀⠀⠀⠀⡰⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢈⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡞⡄⠀⠀⠀⠀ 0.5
⠀⠀⠀⠀⡇⢇⠀⢀⡄⢀⡄⠀⠀⠀⠀⡀⢀⠀⡠⢀⠀⡀⢀⠀⠀⠀⠀⣠⠀⣠⠀⠀⡇⢇⠀⠀⠀⠀
⠀⠀⠀⢸⠀⢸⠀⡜⡇⢸⡇⣾⠀⣾⢸⡇⣿⢸⡗⣿⢸⡇⣿⢰⡇⢰⡇⣿⠀⡏⡆⢸⠀⢸⠀⠀⠀⠀
⠀⠀⢀⠎⠀⢸⠀⡇⡇⡜⡇⡏⣆⢿⢸⡇⣿⢸⣏⣿⢸⡇⣿⢸⢣⡜⡇⡏⡆⡇⡇⢸⠀⠀⢣⠀⠀⠀
⠒⠒⠊⠂⠐⢸⠂⡗⡇⡇⡗⡇⣿⢸⢸⡇⣿⢸⡧⣿⢸⡇⡿⣸⢺⡗⡇⡇⡗⡇⡇⢸⠀⠂⠐⠑⠒⠒
⠀⠀⠀⠀⠀⠀⡇⡇⢱⡇⢣⡇⣿⢸⡇⢧⠛⠼⠳⠟⠼⠱⡇⣿⢸⡇⣇⠇⣧⠃⡇⡎⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⣷⠁⢸⡇⠈⠃⢿⠈⠃⠀⠀⠀⢈⠀⠀⠀⠀⠙⠸⡇⠙⠀⣿⠀⢸⡇⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠻⠀⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠠⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠀⠘⠇⠀⠀⠀⠀⠀⠀ -0.4
-10.0                            10.0

   Please see: out_ch05_exercise13_b.png

c) Root mean squared of position:
    2.3452078737858173

*/
