use ndarray::Array1;
use std::f64::consts::PI;

fn legendre_polynomial_roots_and_gaussian_quadrature_weights(n: usize) -> (Array1<f64>, Array1<f64>) {
  let nf64 = n as f64;

  let a = Array1::linspace(3.0, 4.0*nf64 - 1.0, n);
  let a = a / (4.0*nf64 + 2.0);
  
  let t1 = 8.0*nf64*nf64*a.tan();
  let mut x = (PI*a + 1.0 / t1).cos();

  let epsilon = 1.0e-15;
  loop {
    let mut p0 = Array1::ones(n);
    let mut p1 = x.clone();
    for k in 1..n {
      let kf64 = k as f64;

      let p2 = (2.0*kf64 + 1.0)*x.clone()*p1.clone();
      let p2 = p2 - kf64*p0.clone();
      let p2 = p2/(kf64 + 1.0);

      p0.assign(&p1);
      p1.assign(&p2);
    }

    let dp = p0 - x.clone()*p1.clone();
    let dp = (nf64 + 1.0)*(dp)/(1.0-x.pow2());

    let dx = p1/dp.clone();
    let delta = dx.abs().iter().copied().reduce(f64::max).unwrap();

    let xminusdx = x.clone() - dx;
    x.assign(&xminusdx);

    if delta <= epsilon {
      let w = 2.0*(nf64+1.0)*(nf64+1.0)/(nf64*nf64*(1.0-x.pow2())*dp.pow2());
      return (x, w);
    }
  }
}

pub fn integrate_gaussian_quadrature<F>(a: f64, b: f64, n: usize, f: F) -> f64
    where F: Fn(f64) -> f64
{
  let (x, w) = legendre_polynomial_roots_and_gaussian_quadrature_weights(n);
  let xp = 0.5 * (b - a) * x + 0.5 * (a + b);
  let wp = 0.5 * (b - a) * w;
  let fx = xp.map(|&x0| f(x0));
  return (fx * wp).sum();
}

#[test]
fn test_legendre_polynomial_roots_and_gaussian_quadrature_weights() {
  let f = |x: f64| x*x*x*x - 2.0*x + 1.0;
  let a = 0.0;
  let b = 2.0;
  let exact = 4.4;

  let epsilon = 0.000000001;
  let approx = integrate_gaussian_quadrature(a, b, 3, &f);
  println!("approx = {approx}");
  assert!((approx - exact).abs() < epsilon);
}
