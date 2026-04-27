use burn::tensor::backend::Backend;
use burn::tensor::{Int, Tensor, TensorData};
use burn::record::{CompactRecorder, Recorder};
use burn::prelude::*;

use crate::model::transformer::{CobaltModelConfig, CobaltModel};
use crate::data::loader::TextDataset;

pub fn generate_text<B: Backend>(device: B::Device, prompt: &str, num_tokens: usize) {
    let dataset = TextDataset::new("data/input.txt"); 

    let seq_len = 128;
    let d_model = 256;
    let n_layers = 4;
    let n_heads = 4;

    let config = CobaltModelConfig::new(n_heads, n_layers, d_model, dataset.vocab_size, seq_len);
    
    let recorder = CompactRecorder::new();
    let record = recorder.load("models/cobalt_model".into(), &device).expect("Failed to load model weights. Did you run train first?");
    let model: CobaltModel<B> = config.init(&device).load_record(record);

    let mut tokens = dataset.tokenizer.encode(prompt);
    
    print!("{}", prompt);
    use std::io::Write;
    std::io::stdout().flush().unwrap();

    for _ in 0..num_tokens {
        let (context_start, actual_seq_len) = if tokens.len() > seq_len {
            (tokens.len() - seq_len, seq_len)
        } else {
            (0, tokens.len())
        };
        let context = &tokens[context_start..];
        
        let input_tensor: Tensor<B, 2, Int> = Tensor::from_data(
            TensorData::new(context.to_vec(), [1, actual_seq_len]),
            &device
        );

        let output = model.forward(input_tensor);
        
        let [_, seq_dim, vocab_dim] = output.dims();
        let logits = output.reshape([seq_dim, vocab_dim]);
        
        let last_token_logits = logits.slice([seq_dim - 1..seq_dim]);
        let next_token_tensor = last_token_logits.argmax(1);
        
        // This relies on the internal primitive types, casted by format/parse trick as fallback
        let next_token_str = format!("{}", next_token_tensor.into_scalar());
        let next_token_id: i32 = next_token_str.parse().unwrap_or(0);
        
        tokens.push(next_token_id);
        
        let next_char = dataset.tokenizer.decode(&[next_token_id]);
        print!("{}", next_char);
        std::io::stdout().flush().unwrap();
    }
    println!();
}
