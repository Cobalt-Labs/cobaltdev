use burn::nn::attention::{MhaInput, MultiHeadAttention, MultiHeadAttentionConfig};
use burn::nn::{Linear, LinearConfig};
use burn::prelude::*;

#[derive(Config, Debug)]
pub struct CobaltModelConfig {
    pub n_heads: usize,
    pub n_layers: usize,
    pub d_model: usize,
    pub vocab_size: usize,
    pub max_seq_len: usize,
}

impl CobaltModelConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> CobaltModel<B> {
        CobaltModel {
            output_layer: LinearConfig::new(self.d_model, self.vocab_size).init(device),
            attention: MultiHeadAttentionConfig::new(self.d_model, self.n_heads).init(device),
        }
    }
}

#[derive(Module, Debug)]
pub struct CobaltModel<B: Backend> {
    output_layer: Linear<B>,
    attention: MultiHeadAttention<B>,
}

impl<B: Backend> CobaltModel<B> {
    pub fn forward(&self, input: Tensor<B, 2, Int>) -> Tensor<B, 3> {
        let [batch_size, seq_len] = input.dims();
        let device = input.device();

        // let x: Tensor<B, 3> = Tensor::zeros([batch_size, seq_len, 512], &device);
        let x: Tensor<B, 3> = Tensor::zeros(
            [
                batch_size,
                seq_len,
                <burn::nn::Linear<B> as Clone>::clone(&self.output_layer)
                    .to_device(&device)
                    .weight
                    .dims()[1],
            ],
            &device,
        );
        let mha_input = MhaInput::new(x.clone(), x.clone(), x);
        let output = self.attention.forward(mha_input);

        self.output_layer.forward(output.context)
    }
}
