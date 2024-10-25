use plotters::prelude::*;
use textplots::{Chart, Plot, Shape};

use rust_newman_computational_physics::utils::integrate::integrate_simpsons_rule;

// e^(-t^2)
fn exp_neg_t2(t: f64) -> f64 {
  (-t*t).exp()
}

// We have a scaled variant without the typical leading coefficient:
//   scaled_erf(x) = integral from 0 to x of e^(-t^2) dt
fn scaled_erf(x: f64, n: usize) -> f64 {
  integrate_simpsons_rule(0.0, x, n, exp_neg_t2)
}

fn ch05_exercise03_b() -> Result<(), Box<dyn std::error::Error>> {
  // First a quick plot in the terminal.
  println!("b) A plot of E(x)");
  Chart::new(75, 30, -4.0, 4.0)
      .lineplot(&Shape::Continuous(Box::new(|x| scaled_erf(x.into(), 50) as f32)))
      .display();
    
  // Then a plot rendered to a file.
  let root = BitMapBackend::new("out_ch05_exercise03_b.png", (640, 480)).into_drawing_area();
  root.fill(&WHITE)?;

  let mut chart = ChartBuilder::on(&root)
      .caption("E(x)", ("sans-serif", 40).into_font())
      .margin(10)
      .x_label_area_size(30)
      .y_label_area_size(30)
      .build_cartesian_2d(-4.0..4.0, -1.0..1.0)?;

  chart.configure_mesh().draw()?;

  chart.draw_series(LineSeries::new(
    (0..=100).map(|x| (x as f64 / 100.0 * 8.0 - 4.0, scaled_erf(x as f64 / 100.0 * 8.0 - 4.0, 50))),
    &RED,
  ))?;

  chart.configure_series_labels().draw()?;

  root.present()?;

  println!("   Please see: out_ch05_exercise03_b.png");
  println!();

  Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  println!("let E(x) = integral(0.0, x, t -> e^(-t^2)");
  println!();

  println!("a)");
  let x_eps = 0.001;
  let mut x = 0.0;
  while x - x_eps < 3.0 {
    println!("  E({:.1}) = {}", x, scaled_erf(x, 50));
    x += 0.1;
  }
  println!();

  ch05_exercise03_b()?;

  println!("bonus) A plot of e^(-t^2)");
  Chart::new(75, 30, -4.0, 4.0)
      .lineplot(&Shape::Continuous(Box::new(|t| exp_neg_t2(t.into()) as f32)))
      .display();
  println!();

  Ok(())
}

/*
let E(x) = integral(0.0, x, t -> e^(-t^2)

a)
  E(0.0) = 0
  E(0.1) = 0.09966766429044124
  E(0.2) = 0.19736503092956303
  E(0.3) = 0.2912378826792343
  E(0.4) = 0.3796528397836295
  E(0.5) = 0.46128100662914717
  E(0.6) = 0.5351535272479309
  E(0.7) = 0.6006856688223607
  E(0.8) = 0.6576698573852044
  E(0.9) = 0.7062415162525462
  E(1.0) = 0.7468241341203176
  E(1.1) = 0.7800614335662921
  E(1.2) = 0.8067447583730762
  E(1.3) = 0.8277429884646518
  E(1.4) = 0.8439407113591214
  E(1.5) = 0.8561883893502834
  E(1.6) = 0.8652662198251608
  E(1.7) = 0.8718615855995673
  E(1.8) = 0.8765586250962081
  E(1.9) = 0.8798375996655153
  E(2.0) = 0.8820813803420905
  E(2.1) = 0.8835864316090801
  E(2.2) = 0.8845760112349077
  E(2.3) = 0.8852138028381914
  E(2.4) = 0.8856167383140137
  E(2.5) = 0.8858662672681918
  E(2.6) = 0.8860177402654367
  E(2.7) = 0.8861078721502663
  E(2.8) = 0.8861604437040144
  E(2.9) = 0.8861905011796063
  E(3.0) = 0.8862073466746058

b) A plot of E(x)
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢈⠀⠀⠀⠀⡠⠔⠒⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠁ 0.9
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠠⠀⠀⡠⠊⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠐⠀⡜⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⡀⢀⠀⡀⢀⠀⡀⢀⠀⡀⢀⠀⡀⢀⠀⡀⢀⠀⣈⢎⠀⡀⢀⠀⡀⢀⠀⡀⢀⠀⡀⢀⠀⡀⢀⠀⡀⢀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠮⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⠃⠐⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠔⠁⠀⢈⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠤⠒⠊⠁⠀⠀⠀⠠⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀ -0.9
-4.0                              4.0

   Please see: out_ch05_exercise03_b.png

bonus) A plot of e^(-t^2)
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡠⢊⠑⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀ 1.0
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡰⠁⠠⠀⠘⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⠃⠀⠐⠀⠀⠱⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠇⠀⠀⢈⠀⠀⠀⢣⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡜⠀⠀⠀⠠⠀⠀⠀⠈⢆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡜⠀⠀⠀⠀⠐⠀⠀⠀⠀⠈⢆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠜⠀⠀⠀⠀⠀⢈⠀⠀⠀⠀⠀⠈⠢⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠤⠤⠤⠤⠤⠤⠤⠤⠤⠔⠒⠁⠀⠀⠀⠀⠀⠀⠠⠀⠀⠀⠀⠀⠀⠀⠈⠒⠢⠤⠤⠤⠤⠤⠤⠤⠤⠄ 0.0
-4.0                              4.0

*/
