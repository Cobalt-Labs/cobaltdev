use scratch_model::LinearModel;

fn main() {
    let x = vec![1.0, 2.0, 3.0, 4.0];
    let y = vec![2.0, 4.0, 6.0, 8.0];

    let mut model = LinearModel::new();
    model.train(x, y, 0.01, 1000);

    println!("Weight: {}", model.weight);
    println!("Bias: {}", model.bias);

    println!("Prediction for 5: {}", model.predict(5.0));
}