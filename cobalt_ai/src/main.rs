use burn::tensor::{Tensor, backend::Backend};
fn computation<B: Backend>() {
    // Create the device where to do the computation
    let device = Default::default();
    let tensor1: Tensor<B, 2> = Tensor::from_floats([[2., 3.], [4., 5.]], &device);
    let tensor2 = Tensor::ones_like(&tensor1);
    // Print the element-wise addition of the two tensors.
    println!("{:}", tensor1 + tensor2);
}
fn main() {
    computation::<burn::backend::Wgpu>();
}