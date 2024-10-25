use std::f64::consts::PI;

use plotters::prelude::*;
use textplots::{Chart, Plot, Shape};

use rust_newman_computational_physics::utils::integrate::integrate_simpsons_rule;

fn bessel(m: i32, x: f64) -> f64 {
  let m = m as f64;
  integrate_simpsons_rule(0.0, PI, 1000,
    |theta| (m*theta - x*theta.sin()).cos()
  ) / PI
}

fn ch05_exercise04_a() -> Result<(), Box<dyn std::error::Error>> {
  // First a quick plot in the terminal.
  println!("a) A plot of J_1(x), J_2(x), and J_3(x)");
  Chart::new(75, 30, 0.0, 20.0)
      .lineplot(&Shape::Continuous(Box::new(|x| bessel(1, x.into()) as f32)))
      .lineplot(&Shape::Continuous(Box::new(|x| bessel(2, x.into()) as f32)))
      .lineplot(&Shape::Continuous(Box::new(|x| bessel(3, x.into()) as f32)))
      .display();

  // Then a plot rendered to a file.
  let root = BitMapBackend::new("out_ch05_exercise04_a.png", (800, 600)).into_drawing_area();
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

  let mut chart = ChartBuilder::on(&root)
    .caption("Bessel Functions J_m(x)", ("sans-serif", 40).into_font())
    .margin(10)
    .x_label_area_size(30)
    .y_label_area_size(30)
    .build_cartesian_2d(0.0..20.0, -1.0..1.0)?;

  chart.configure_mesh().draw()?;

  chart.draw_series(LineSeries::new(
    (0..=75).map(|x| (x as f64 / 75.0 * 20.0, bessel(1, x as f64 / 75.0 * 20.0))),
    my_blue_stroke,
  ))?
    .label("J_1(x)")
    .legend(|(x, y)| PathElement::new([(x, y), (x + 20, y)], my_blue_stroke));


  chart.draw_series(LineSeries::new(
    (0..=75).map(|x| (x as f64 / 75.0 * 20.0, bessel(2, x as f64 / 75.0 * 20.0))),
    my_orange_stroke,
  ))?
    .label("J_2(x)")
    .legend(|(x, y)| PathElement::new([(x, y), (x + 20, y)], my_orange_stroke));

  chart.draw_series(LineSeries::new(
    (0..=75).map(|x| (x as f64 / 75.0 * 20.0, bessel(3, x as f64 / 75.0 * 20.0))),
    my_green_stroke,
  ))?
    .label("J_3(x)")
    .legend(|(x, y)| PathElement::new([(x, y), (x + 20, y)], my_green_stroke));

  chart.configure_series_labels()
    .label_font(("sans-serif", 20).into_font())
    .position(SeriesLabelPosition::UpperRight)
    .draw()?;

  root.present()?;

  println!("   Please see: out_ch05_exercise04_a.png");
  println!();

  Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  ch05_exercise04_a()?;

  Ok(())
}
