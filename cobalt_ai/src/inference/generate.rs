use crate::data::loader::TextDataset;
use crate::model::transformer::{CobaltModel, CobaltModelConfig};
use burn::prelude::*;
use burn::record::{CompactRecorder, Recorder};
use burn::tensor::backend::Backend;
use burn::tensor::{Int, Tensor, TensorData};
use rand::distributions::{Distribution, WeightedIndex};
use rand::prelude::*;
use std::io::Write;

pub fn generate_text<B: Backend<IntElem = i32>>(
    device: B::Device,
    prompt: &str,
    num_tokens: usize,
    temperature: f32,
    top_k: usize,
) {
    let dataset = TextDataset::new("data/input.txt");

    let d_model = 192;
    let seq_len = 64;
    let n_heads = 4;
    let n_layers = 3;

    let config = CobaltModelConfig::new(n_heads, n_layers, d_model, dataset.vocab_size, seq_len);

    let recorder = CompactRecorder::new();
    let record = match recorder.load("models/cobalt_model".into(), &device) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("❌ Failed to load model: {}", e);
            eprintln!("💡 Run 'cargo run -- train' first to train the model!");
            return;
        }
    };

    let model: CobaltModel<B> = config.init(&device).load_record(record);

    let mut tokens = dataset.tokenizer.encode(prompt);

    print!("{}", prompt);
    std::io::stdout().flush().unwrap();

    let mut rng = thread_rng();

    for step in 0..num_tokens {
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

        let mut logits_vec: Vec<f32> = last_token_logits.to_data().to_vec().unwrap();

        if temperature > 0.0 {
            for logit in logits_vec.iter_mut() {
                *logit = *logit / temperature;
            }
        }

        let max_logit = logits_vec.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let exp_sum: f32 = logits_vec.iter().map(|&x| (x - max_logit).exp()).sum();
        let mut probs: Vec<f32> = logits_vec
            .iter()
            .map(|&x| ((x - max_logit).exp() / exp_sum))
            .collect();

        if top_k > 0 && top_k < probs.len() {
            let mut indices: Vec<usize> = (0..probs.len()).collect();
            indices.sort_by(|&a, &b| probs[b].partial_cmp(&probs[a]).unwrap());

            for &idx in &indices[top_k..] {
                probs[idx] = 0.0;
            }

            let sum: f32 = probs.iter().sum();
            for prob in probs.iter_mut() {
                *prob /= sum;
            }
        }

        let valid_indices: Vec<usize> = (0..probs.len()).filter(|&i| probs[i] > 0.0).collect();

        let next_token_id = if valid_indices.is_empty() || temperature == 0.0 {
            probs
                .iter()
                .enumerate()
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .map(|(idx, _)| idx as i32)
                .unwrap_or(0)
        } else {
            match WeightedIndex::new(&probs) {
                Ok(dist) => dist.sample(&mut rng) as i32,
                Err(_) => {
                    // Fallback to argmax if distribution is invalid
                    probs
                        .iter()
                        .enumerate()
                        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                        .map(|(idx, _)| idx as i32)
                        .unwrap_or(0)
                }
            }
        };

        tokens.push(next_token_id);

        let next_char = dataset.tokenizer.decode(&[next_token_id]);
        print!("{}", next_char);
        std::io::stdout().flush().unwrap();

        if step > 20 && next_char == "\n" && tokens.len() > 50 {
            break;
        }
    }
    println!("\n");
}
