use anyhow::Result;
use candle_core::{Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::llama::{Llama, Config as LlamaConfig};
use tokenizers::Tokenizer;

pub struct LocalLLM {
    model: Llama,
    tokenizer: Tokenizer,
    device: Device,
    max_length: usize,
}

impl LocalLLM {
    pub fn new(model_path: &str, tokenizer_path: &str) -> Result<Self> {
        let device = Device::Cpu; // or Device::Cuda(0) if available
        let tokenizer = Tokenizer::from_file(tokenizer_path).unwrap();
        
        let config = LlamaConfig {
            vocab_size: 32000,
            hidden_size: 4096,
            intermediate_size: 11008,
            num_hidden_layers: 32,
            num_attention_heads: 32,
            max_position_embeddings: 2048,
            ..Default::default()
        };
        
        let vb = VarBuilder::from_mmaped_safetensors(&[model_path], &device, None)?;
        let model = Llama::load(vb, &config)?;
        
        Ok(Self {
            model,
            tokenizer,
            device,
            max_length: 2048,
        })
    }
    
    pub fn generate(&mut self, prompt: &str) -> Result<String> {
        let tokens = self.tokenizer.encode(prompt, true).unwrap();
        let input_ids = Tensor::new(&[tokens.get_ids()], &self.device)?;
        
        let output = self.model.generate(&input_ids, self.max_length)?;
        let output_tokens = output.to_vec1()?;
        
        Ok(self.tokenizer.decode(&output_tokens, true).unwrap())
    }
    
    pub fn stream_generate(&mut self, prompt: &str) -> impl Iterator<Item = String> {
        // Implement streaming for better UX
        std::iter::empty()
    }
}