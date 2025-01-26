use plotters::prelude::*;
use textplots::{Chart, Plot, Shape};
use rust_newman_computational_physics::utils::integrate::integrate_gaussian_quadrature;

const BOLTZMANNS_CONSTANT: f64 = 1.380649e-23; // measured in J/K

fn calculate_heat_capacity(temperature: f64, volume: f64, n: usize, number_density: f64, debye_temperature: f64) -> f64 {
  let integrand = |x: f64| (x*x*x*x * x.exp())/(x.exp() - 1.0).powi(2);
  let a = 0.0;
  let b = debye_temperature / temperature;
  let approx = integrate_gaussian_quadrature(a, b, n, &integrand);

  let cv = 9.0 * volume * number_density * BOLTZMANNS_CONSTANT * (temperature / debye_temperature).powi(3) * approx;

  cv
}

fn ch05_exercise09_a() -> Result<(), Box<dyn std::error::Error>> {
  let number_density = 6.002e28; // Denoted rho, measured in atoms/m^3
  let volume = 1000.0; // Denoted V, measured in cm^3
  let volume = volume / (100.0*100.0*100.0); // Now measured in m^3
  let debye_temperature = 428.0; // Denoted thetaD, measured in K
  let temperature = 290.0; // Denoted T, measured in K
  let n: usize = 50; // Denoted N, number of slices

  let cv = calculate_heat_capacity(temperature, volume, n, number_density, debye_temperature);

  println!("a) The heat capacity of {volume} cubic meters of aluminum at {temperature}K is approximately:");
  println!("    {cv} J/K");
  println!();

  Ok(())
}

fn ch05_exercise09_b() -> Result<(), Box<dyn std::error::Error>> {
  let number_density = 6.002e28; // Denoted rho, measured in atoms/m^3
  let volume = 1000.0; // Denoted V, measured in cm^3
  let volume = volume / (100.0*100.0*100.0); // Now measured in m^3
  let debye_temperature = 428.0; // Denoted thetaD, measured in K
  let n: usize = 50; // Denoted N, number of slices
  let a = 5.0; // Measured in K
  let b = 500.0; // Measured in K

  println!("b) A plot of the specific heat of {volume} cubic meters of aluminum from T={a}K to T={b}K");
  Chart::new(75, 30, a as f32, b as f32)
      .lineplot(&Shape::Continuous(Box::new(
        |t| calculate_heat_capacity(t.into(), volume, n, number_density, debye_temperature) as f32
      )))
      .display();
  println!();

  // Then a plot rendered to a file.
  let root = BitMapBackend::new("out_ch05_exercise09_b.png", (640, 480)).into_drawing_area();
  root.fill(&WHITE)?;

  let mut chart = ChartBuilder::on(&root)
      .caption("Specific Heat of Aluminum", ("sans-serif", 40).into_font())
      .margin(10)
      .x_label_area_size(30)
      .y_label_area_size(60)
      .build_cartesian_2d(a..b, 0.0..2550.0)?;

  chart.configure_mesh()
    .x_label_formatter(&|x| format!("{:} K", *x))
    .y_label_formatter(&|y| format!("{:} J /K", *y))
    .draw()?;

  chart.draw_series(LineSeries::new(
    (0..=200).map(
      |x| {
        let frac = x as f64 / 200.0;
        let t = b * frac + a * (1.0 - frac);

        (t, calculate_heat_capacity(t, volume, n, number_density, debye_temperature))
      }
    ),
    &RED,
  ))?;

  chart.configure_series_labels().draw()?;

  root.present()?;

  println!("   Please see: out_ch05_exercise09_b.png");
  println!();

  Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  ch05_exercise09_a()?;
  ch05_exercise09_b()?;

  Ok(())
}

/*
a) The heat capacity of 0.001 cubic meters of aluminum at 290K is approximately:
    2234.9797723913557 J/K

b) A plot of the specific heat of 0.001 cubic meters of aluminum from T=5K to T=500K
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣀⣀⠤⠤⠤⠤⠒⠒⠒⠒⠒⠒⠒⠉⠉⠉⠉⠉⠉⠁ 2394.9
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⠤⠔⠊⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠔⠊⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⢀⠔⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⡔⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⢀⠎⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⡠⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠤⠤⠊⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀ 0.3
5.0                             500.0


   Please see: out_ch05_exercise09_b.png

*/
