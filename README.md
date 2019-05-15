# ft_linear_regression

This 42 project aims to implement a linear function train with a gradient descent algorithm.

## Compiling

You can compile this project using `cargo build`.

You can initialize d3 js dependency using `npm i d3 --save`.

## Usage

### Train

Use `cargo run --bin train -- -f [file]` to train your model.

Theta0 and Theta1 variables will be save in `.env` file.

### Predict

Use `cargo run --bin predict`.

Enter a mileage to get a price using the previously computed model.

### Graph

Open `graph.htlm` with your favorite web browser and load both your data set and model to plot then.