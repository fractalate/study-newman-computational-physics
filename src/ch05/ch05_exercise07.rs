use plotters::prelude::*;
use textplots::{Chart, Plot, Shape};
use rust_newman_computational_physics::utils::integrate::integrate_romberg_adaptive;


// This is a copy of the integrate_simpsons_rule_adaptive implementation.
pub fn add_odds_from_1<F>(a: f64, h: f64, n: usize, f: F) -> f64
  where F: Fn(f64) -> f64
{
  let mut total = 0.0;

  for i in (1..=n-1).step_by(2) {
    let x = a + h*(i as f64);
    total += f(x);
  }

  total
}

// This is a copy of the integrate_simpsons_rule_adaptive implementation.
pub fn add_evens_from_2<F>(a: f64, h: f64, n: usize, f: F) -> f64
  where F: Fn(f64) -> f64
{
  let mut total = 0.0;

  for i in (2..=n-2).step_by(2) {
    let x = a + h*(i as f64);
    total += f(x);
  }

  total
}

// This is a copy of the integrate_simpsons_rule_adaptive implementation.
// Now with logging.
pub fn integrate_simpsons_rule_adaptive<F>(a: f64, b: f64, epsilon: f64, f: F) -> f64
  where F: Fn(f64) -> f64
{
  // This base case starts at 2.
  let mut n = 2;
  let mut h = (a + b) / (n as f64);

  let mut s1 = (f(a) + f(b) + 2.0 * add_evens_from_2(a, h, n, &f)) / 3.0;
  let mut t1 = add_odds_from_1(a, h, n, &f) * 2.0 / 3.0;
  let mut i1 = h * (s1 + 2.0 * t1);

  println!("    Number of posts: {}, Estimate of Integral: {}, Estimated Error: --", n, i1);

  while n <= (1<<24) {
    h /= 2.0;
    n *= 2;

    let s2 = s1 + t1;
    let t2 = add_odds_from_1(a, h, n, &f) * 2.0 / 3.0;
    let i2 = h * (s2 + 2.0 * t2);

    let error = (i2 - i1) / 15.0;

    println!("    Number of posts: {}, Estimate of Integral: {}, Estimated Error: {}", n, i2, error);

    i1 = i2;
    s1 = s2;
    t1 = t2;

    if error.abs() < epsilon {
      break
    }
  }

  println!("    Answer {}", i1);

  i1
}

// This is a copy of the integrate_trapezoidal_rule_adaptive implementation.
pub fn _integrate_trapezoidal_rule<F>(a: f64, b: f64, n: usize, h: f64, f: F) -> f64
  where F: Fn(f64) -> f64
{
  let mut total = 0.0;

  total += f(a);

  for i in 1..n {
    let x = a + h*(i as f64);
    total += 2.0*f(x);
  }

  total += f(b);
  total *= h / 2.0;

  total
}

// This is a copy of the integrate_trapezoidal_rule_adaptive implementation.
fn adaptive_sum_trapezoidal_rule<F>(a: f64, n: usize, h: f64, f: F) -> f64
  where F: Fn(f64) -> f64
{
  let mut total = 0.0;

  for i in (1..n).step_by(2) {
    let x = a + h*(i as f64);
    total += f(x);
  }

  total
}

// This is a copy of the integrate_trapezoidal_rule_adaptive implementation.
// Now with logging.
pub fn integrate_trapezoidal_rule_adaptive<F>(a: f64, b: f64, epsilon: f64, f: F) -> f64
  where F: Fn(f64) -> f64
{
  let mut n: usize = 1;
  let mut h = (b - a) / (n as f64);

  // Begin with an initial approximation.
  let mut approximation1 = _integrate_trapezoidal_rule(a, b, n, h, &f);
  let mut approximation2: f64 = 0.0;

  println!("    Number of slices: {}, Estimate of Integral: {}, Estimated Error: --", n, approximation1);

  while n <= (1<<28) {
    // Then calculate the next approximation by adding in samples which are between the
    // previous approximation's samples.
    n *= 2;
    h /= 2.0;
    approximation2 = approximation1 / 2.0 + h * adaptive_sum_trapezoidal_rule(a, n, h, &f);

    // If the approximation is within our error bounds, we have our answer.
    // See section 5.2.1, equation (5.28).
    let error = (approximation2 - approximation1) / 3.0;

    println!("    Number of slices: {}, Estimate of Integral: {}, Estimated Error: {}", n, approximation2, error);

    if error.abs() < epsilon {
      break;
    }

    approximation1 = approximation2;
  }

  println!("    Answer {}", approximation2);

  approximation2
}

pub fn integrate_romberg<F>(a: f64, b: f64, epsilon: f64, f: F) -> f64
  where F: Fn(f64) -> f64
{
  let mut n: usize = 1;
  let mut h = (b - a) / (n as f64);

  // Begin with an initial approximation.
  let mut approximation1 = _integrate_trapezoidal_rule(a, b, n, h, &f);
  let mut rs1: Vec<f64> = Vec::new();
  let mut rs2: Vec<f64> = Vec::new();
  rs1.push(approximation1);

  println!("    Number of slices: {}, Estimated Error: --", n);

  while n <= (1<<28) {
    // Then calculate the next approximation by adding in samples which are between the
    // previous approximation's samples.
    n *= 2;
    h /= 2.0;
    let approximation2 = approximation1 / 2.0 + h * adaptive_sum_trapezoidal_rule(a, n, h, &f);
    rs2.push(approximation2);
    
    print!("    ");
    for k in 1..=rs1.len() {
      let rkj = rs2[k-1] + (rs2[k-1] - rs1[k-1]) / ((4 as i64).pow(k as u32) - 1) as f64;
      print!("{} ", rkj);
      rs2.push(rkj);
    }
    println!();

    // If the approximation is within our error bounds, we have our answer.
    // See section 5.4, equation (5.49).
    let error = (rs2[rs2.len() - 2] - rs1[rs2.len() - 2] ) / ((4 as i64).pow(rs1.len() as u32) - 1) as f64;

    println!("    Number of slices: {}, Estimated Error: {}", n, error);

    if error.abs() < epsilon {
      break; // rs2 will have the final approximations.
    }

    std::mem::swap(&mut rs1, &mut rs2);
    rs2.clear();
    approximation1 = approximation2;
  }

  println!("    Answer {}", rs2[rs2.len() - 1]);

  rs2[rs2.len() - 1]
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
  // f(x) = ( sin(sqrt(100*x)) )^2
  // Integral from 0 to 1 is expected to be around 0.45.
  let integrand = |x: f64| (x*100.0).sqrt().sin().powi(2);
  let a = 0.0;
  let b = 1.0;

  println!("bonus) A plot of ( sin(sqrt(100*x)) )^2");
  Chart::new(75, 30, a as f32, b as f32)
      .lineplot(&Shape::Continuous(Box::new(|t| integrand(t.into()) as f32)))
      .display();
  println!();

  // Then a plot rendered to a file.
  let root = BitMapBackend::new("out_ch05_exercise07_bonus.png", (640, 480)).into_drawing_area();
  root.fill(&WHITE)?;

  let mut chart = ChartBuilder::on(&root)
      .caption("( sin(sqrt(100*x)) )^2", ("sans-serif", 40).into_font())
      .margin(10)
      .x_label_area_size(30)
      .y_label_area_size(30)
      .build_cartesian_2d(a..b, 0.0..1.0)?;

  chart.configure_mesh().draw()?;

  chart.draw_series(LineSeries::new(
    (0..=200).map(|x| (x as f64 / 200.0, integrand(x as f64 / 200.0))),
    &RED,
  ))?;

  chart.configure_series_labels().draw()?;

  root.present()?;

  println!("   Please see: out_ch05_exercise07_bonus.png");
  println!();


  println!("a) Integral of ( sin(sqrt(100*x)) )^2 from {} to {} with trapezoidal rule:", a, b);
  println!();
  integrate_trapezoidal_rule_adaptive(a, b, 1.0e-6, integrand);
  println!();
  println!("    (bonus) with Simpson's Rule:");
  println!();
  integrate_simpsons_rule_adaptive(a, b, 1.0e-6, integrand);
  println!();

  println!("b) Integral of ( sin(sqrt(100*x)) )^2 from {} to {} with Romberg integration:", a, b);
  println!();
  integrate_romberg(a, b, 1.0e-6, integrand);
  println!();
  println!("    Compare to Library {}", integrate_romberg_adaptive(a, b, 1.0e-6, integrand));

  Ok(())
}

/*
bonus) A plot of ( sin(sqrt(100*x)) )^2
⣱⡇⠀⠀⠀⠀⠀⡰⠉⢆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡠⠊⠉⠢⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀ 1.0
⠜⢱⠀⠀⠀⠀⢰⠁⠀⠘⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⡰⠁⠀⠀⠀⠘⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠂⢸⠀⠀⠀⠀⡎⠀⠀⠀⢱⠀⠀⠀⠀⠀⠀⠀⠀⢰⠁⠀⠀⠀⠀⠀⠘⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⡁⠘⡄⠀⠀⢠⠃⠀⠀⠀⠈⡆⠀⠀⠀⠀⠀⠀⢀⠇⠀⠀⠀⠀⠀⠀⠀⠱⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠄⠀⡇⠀⠀⡸⠀⠀⠀⠀⠀⢸⠀⠀⠀⠀⠀⠀⡎⠀⠀⠀⠀⠀⠀⠀⠀⠀⢱⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠂⠀⢣⠀⠀⡇⠀⠀⠀⠀⠀⠀⢇⠀⠀⠀⠀⡸⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢣⠀⠀⠀⠀⠀⠀⠀⡀
⡁⠀⠸⡀⢸⠀⠀⠀⠀⠀⠀⠀⠘⡄⠀⠀⡰⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠱⡀⠀⠀⠀⢀⠜⠀
⠄⠀⠀⠣⠇⠀⠀⠀⠀⠀⠀⠀⠀⠈⠢⠔⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠢⠤⠔⠁⠀⠀ 0.0
0.0                               1.0


   Please see: out_ch05_exercise07_bonus.png

a) Integral of ( sin(sqrt(100*x)) )^2 from 0 to 1 with trapezoidal rule:

    Number of slices: 1, Estimate of Integral: 0.147979484546652, Estimated Error: --
    Number of slices: 2, Estimate of Integral: 0.3252319078064746, Estimated Error: 0.05908414108660753
    Number of slices: 4, Estimate of Integral: 0.5122828507233315, Estimated Error: 0.06235031430561896
    Number of slices: 8, Estimate of Integral: 0.4029974484782483, Estimated Error: -0.03642846741502772
    Number of slices: 16, Estimate of Integral: 0.43010336929474696, Estimated Error: 0.009035306938832885
    Number of slices: 32, Estimate of Integral: 0.4484146657874699, Estimated Error: 0.0061037654975743165
    Number of slices: 64, Estimate of Integral: 0.4539129312153758, Estimated Error: 0.0018327551426352933
    Number of slices: 128, Estimate of Integral: 0.45534850437280205, Estimated Error: 0.000478524385808754
    Number of slices: 256, Estimate of Integral: 0.455711266453241, Estimated Error: 0.00012092069347964991
    Number of slices: 512, Estimate of Integral: 0.45580219965166413, Estimated Error: 0.000030311066141042176
    Number of slices: 1024, Estimate of Integral: 0.45582494813241997, Estimated Error: 0.000007582826918613635
    Number of slices: 2048, Estimate of Integral: 0.45583063620164654, Estimated Error: 0.000001896023075524204
    Number of slices: 4096, Estimate of Integral: 0.455832058278271, Estimated Error: 0.0000004740255414859007
    Answer 0.455832058278271

    (bonus) with Simpson's Rule:

    Number of posts: 2, Estimate of Integral: 0.38431604889308213, Estimated Error: --
    Number of posts: 4, Estimate of Integral: 0.5746331650289503, Estimated Error: 0.012687807742391215
    Number of posts: 8, Estimate of Integral: 0.36656898106322056, Estimated Error: -0.013870945597715319
    Number of posts: 16, Estimate of Integral: 0.4391386762335798, Estimated Error: 0.004837979678023951
    Number of posts: 32, Estimate of Integral: 0.45451843128504427, Estimated Error: 0.0010253170034309625
    Number of posts: 64, Estimate of Integral: 0.45574568635801116, Estimated Error: 0.00008181700486445954
    Number of posts: 128, Estimate of Integral: 0.45582702875861086, Estimated Error: 0.0000054228267066469545
    Number of posts: 256, Estimate of Integral: 0.45583218714672064, Estimated Error: 0.00000034389254065144335
    Answer 0.45583218714672064

b) Integral of ( sin(sqrt(100*x)) )^2 from 0 to 1 with Romberg integration:

    Number of slices: 1, Estimated Error: --
    0.38431604889308213 
    Number of slices: 2, Estimated Error: 0.05908414108660753
    0.5746331650289505 0.5873209727713417 
    Number of slices: 4, Estimated Error: 0.012687807742391222
    0.3665689810632206 0.35269803546550527 0.34897386185747614 
    Number of slices: 8, Estimated Error: -0.003724173608029149
    0.4391386762335798 0.4439766559116038 0.4454255229028117 0.4458037647108326 
    Number of slices: 16, Estimated Error: 0.0003782418080209237
    0.4545184312850442 0.45554374828847516 0.4557273529293779 0.4557677522628154 0.4557774922310969 
    Number of slices: 32, Estimated Error: 0.000009739968281508114
    0.4557456863580111 0.4558275033628756 0.45583200741167557 0.45583241782141004 0.45583248103309976 0.45583249446137863 
    Number of slices: 64, Estimated Error: 0.000000013428278877370225
    Answer 0.45583249446137863

    Compare to Library 0.45583249446137863

*/
