# Newman Computational Physics in Rust

Hello! Welcome to my self study resources for [Newman Computational Physics (2013)](https://websites.umich.edu/~mejn/cp/). This repository uses Rust and Jupyter notebooks (powered by Python) to work the examples and exercises from the book.

<!-- TODO: include assets generated by problems in the repository and have links to them here. -->

## Index

* Chapter 5
  - [Exercise 2](./src/ch05/ch05_exercise02.rs)
  - [Exercise 3](./src/ch05/ch05_exercise03.rs) (Outputs: [3b](./out_ch05_exercise03_b.png))
  - [Exercise 4](./src/ch05/ch05_exercise04.rs)  (Outputs: [4a](./out_ch05_exercise04_a.png), [4b](./out_ch05_exercise04_b.png))
  - [Exercise 5 (Notebook)](./src/ch05/ch05_exercise05.ipynb)
  - [Exercise 6](./src/ch05/ch05_exercise06.rs)
  - [Exercise 7](./src/ch05/ch05_exercise07.rs) (Outputs: [bonus](./out_ch05_exercise07_bonus.png))

## Running

For example,

```
cargo run --bin ch05_exercise03
```

## Numerical Methods

The following numerical methods are implemented as part of this codebase:

* Module `rust_newman_computational_physics::utils::integrate`
  - [`integrate_simpsons_rule`](./src/utils/integrate/integrate_simpsons_rule.rs)
  - [`integrate_simpsons_rule_adaptive`](./src/utils/integrate/integrate_simpsons_rule.rs)
  - [`integrate_trapezoidal_rule`](./src/utils/integrate/integrate_trapezoidal_rule.rs)
  - [`integrate_trapezoidal_rule_adaptive`](./src/utils/integrate/integrate_trapezoidal_rule.rs)
  - [`integrate_romberg_adaptive`](./src/utils/integrate/integrate_trapezoidal_rule.rs)

## Rust Dependencies

The `plotters` dependency may require you to additionally install `fontconfig` development files.
For me, that required `apt-get install libfontconfig-dev`.

## Python Dependencies

To set up a Python virtual environment for Jupyter notebooks, from the root of this project:

```
python3 -m venv venv
source venv/bin/activate
```

Then install dependencies:

```
pip install jupyter matplotlib numpy scipy
```

When returning to the project, activate your Python virtual environment again:

```
source venv/bin/activate
```

Notebooks are distributed throughout the project, so you can open up Jupyter in the root of this project:

```
jupyter notebook
```
