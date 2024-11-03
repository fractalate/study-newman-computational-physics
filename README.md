# Newman Computational Physics in Rust

Hello! Welcome to my self study resources for [Newman Computational Physics (2013)](https://websites.umich.edu/~mejn/cp/). This repository uses Rust to work the examples and exercises from the book.

## Index

* Chapter 5
  - [Exercise 2](./src/ch05/ch05_exercise02.rs)
  - [Exercise 3](./src/ch05/ch05_exercise03.rs)
  - [Exercise 4](./src/ch05/ch05_exercise04.rs)
  - [Exercise 5 (Notebook)](./src/ch05/ch05_exercise05.ipynb)

## Running

For example,

```
cargo run --bin ch05_exercise03
```

## Dependencies

The `plotters` dependency may require you to additionally install `fontconfig` development files.
For me, that required `apt-get install libfontconfig-dev`.

## Python

This project uses a Python virtual environment for Jupyter notebooks. To set this up, from the root of the project:

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

Notebooks are distributed throughout the proejct, so you can open up Jupyter Notebook in the root of this project:

```
jupyter notebook
```
