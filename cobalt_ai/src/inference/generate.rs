use burn::prelude::*;
use burn::record::{CompactRecorder, Recorder};
use burn::tensor::backend::Backend;
use burn::tensor::{Int, Tensor, TensorData};
use std::io::Write;
use crate::data::loader::TextDataset;
use crate::model::transformer::{CobaltModel, CobaltModelConfig};

pub fn generate_text<B: Backend<IntElem = i32>>(
    device: B::Device,
    prompt: &str,
    num_tokens: usize,
) {
    let dataset = TextDataset::new("data/input.txt");

    let d_model = 192;
    let seq_len = 64;
    let n_heads = 4;
    let n_layers = 3;

    let config = CobaltModelConfig::new(n_heads, n_layers, d_model, dataset.vocab_size, seq_len);

    let recorder = CompactRecorder::new();
    let record = recorder
        .load("models/cobalt_model".into(), &device)
        .expect("Failed to load model weights. Train first!");

    let model: CobaltModel<B> = config.init(&device).load_record(record);

    let mut tokens = dataset.tokenizer.encode(prompt);

    print!("{}", prompt);
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
            &device,
        );

        let output = model.forward(input_tensor);
        let [_, seq_dim, vocab_dim] = output.dims();

        let logits = output.reshape([seq_dim, vocab_dim]);
        let last_token_logits = logits.slice([seq_dim - 1..seq_dim]);

        let temperature = 0.95;

        let scaled_logits = last_token_logits / temperature;
        
        let repetition_penalty = 1.25;

        let mut logits_vec: Vec<f32> = scaled_logits.to_data().to_vec().unwrap();

        let recent_tokens: Vec<i32> = tokens.iter().rev().take(20).cloned().collect();
        
        for (i, &token_id) in recent_tokens.iter().enumerate() {
            let idx = token_id as usize;
            if idx < logits_vec.len() {
                logits_vec[idx] -= repetition_penalty * (1.0 / (i as f32 + 1.0));
            }
        }

        let penalized_logits = Tensor::<B, 2>::from_data(
            TensorData::new(logits_vec, [1, vocab_dim]), 
            &device
        );

        let probs = burn::tensor::activation::softmax(penalized_logits, 1);

        let next_token_tensor = probs.argmax(1);
        let next_token_id: i32 = next_token_tensor.into_scalar();

        tokens.push(next_token_id);

        let next_char = dataset.tokenizer.decode(&[next_token_id]);
        print!("{}", next_char);
        std::io::stdout().flush().unwrap();
    }
    println!();
}