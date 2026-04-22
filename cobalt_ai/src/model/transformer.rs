use burn::nn::attention::{MhaInput, MultiHeadAttention, MultiHeadAttentionConfig};
use burn::nn::{Embedding, EmbeddingConfig, Linear, LinearConfig, LayerNorm, LayerNormConfig, Gelu};
use burn::prelude::*;

#[derive(Config, Debug)]
pub struct CobaltModelConfig {
    pub n_heads: usize,
    pub n_layers: usize,
    pub d_model: usize,
    pub vocab_size: usize,
    pub max_seq_len: usize,
    #[config(default = "256")]
    pub d_ff: usize,
}

impl CobaltModelConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> CobaltModel<B> {
        let mut blocks = Vec::with_capacity(self.n_layers);
        for _ in 0..self.n_layers {
            blocks.push(TransformerBlockConfig::new(self.d_model, self.n_heads, self.d_ff).init(device));
        }
        
        CobaltModel {
            token_embedding: EmbeddingConfig::new(self.vocab_size, self.d_model).init(device),
            position_embedding: EmbeddingConfig::new(self.max_seq_len, self.d_model).init(device),
            blocks,
            output_layer: LinearConfig::new(self.d_model, self.vocab_size).init(device),
        }
    }
}

#[derive(Module, Debug)]
pub struct CobaltModel<B: Backend> {
    token_embedding: Embedding<B>,
    position_embedding: Embedding<B>,
    blocks: Vec<TransformerBlock<B>>,
    output_layer: Linear<B>,
}

#[derive(Config, Debug)]
pub struct TransformerBlockConfig {
    pub d_model: usize,
    pub n_heads: usize,
    pub d_ff: usize,
}

impl TransformerBlockConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> TransformerBlock<B> {
        TransformerBlock {
            attention: MultiHeadAttentionConfig::new(self.d_model, self.n_heads).init(device),
            norm1: LayerNormConfig::new(self.d_model).init(device),
            norm2: LayerNormConfig::new(self.d_model).init(device),
            ff1: LinearConfig::new(self.d_model, self.d_ff).init(device),
            ff2: LinearConfig::new(self.d_ff, self.d_model).init(device),
            gelu: Gelu::new(),
        }
    }
}

#[derive(Module, Debug)]
pub struct TransformerBlock<B: Backend> {
    attention: MultiHeadAttention<B>,
    norm1: LayerNorm<B>,
    norm2: LayerNorm<B>,
    ff1: Linear<B>,
    ff2: Linear<B>,
    gelu: Gelu,
}

impl<B: Backend> TransformerBlock<B> {
    pub fn forward(&self, input: Tensor<B, 3>) -> Tensor<B, 3> {
        // Pre-norm formulation: x = x + Attention(NormX)
        let norm_x = self.norm1.forward(input.clone());
        let mha_input = MhaInput::new(norm_x.clone(), norm_x.clone(), norm_x);
        let attn_out = self.attention.forward(mha_input);
        let x = input + attn_out.context;
        
        let norm_x2 = self.norm2.forward(x.clone());
        let ff_out = self.ff2.forward(self.gelu.forward(self.ff1.forward(norm_x2)));
        x + ff_out
    }
}

impl<B: Backend> CobaltModel<B> {
    pub fn forward(&self, input: Tensor<B, 2, Int>) -> Tensor<B, 3> {
        let [batch_size, seq_len] = input.dims();
        let device = input.device();
        
        // Ensure positional vector is max seq_len. 
        let positions = Tensor::<B, 1, Int>::arange(0..seq_len as i64, &device)
            .reshape([1, seq_len])
            .repeat_dim(0, batch_size);
        
        let mut x = self.token_embedding.forward(input) + self.position_embedding.forward(positions);
        
        for block in &self.blocks {
            x = block.forward(x);
        }

        self.output_layer.forward(x)
    }
}
