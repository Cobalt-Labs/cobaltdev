use burn::optim::{AdamConfig, Optimizer, GradientsParams};
use burn::tensor::backend::AutodiffBackend;
use burn::prelude::*;
use burn::nn::loss::CrossEntropyLoss;
use burn::record::{CompactRecorder};

use crate::model::transformer::CobaltModelConfig;
use crate::data::loader::TextDataset;

pub fn train<B: AutodiffBackend>(device: B::Device) {
    let mut dataset = TextDataset::new("data/input.txt");
    
    // Hyperparameters
    let batch_size = 32;
    let seq_len = 128;
    let num_epochs = 5;
    let d_model = 256;
    let n_layers = 4;
    let n_heads = 4;
    let learning_rate = 1e-3;
    let iterations_per_epoch = 50; // Keep it super tiny for swift demonstration

    let config = CobaltModelConfig::new(n_heads, n_layers, d_model, dataset.vocab_size, seq_len);
    let mut model: crate::model::transformer::CobaltModel<B> = config.init(&device);
    let mut optimizer = AdamConfig::new().init();

    let loss_fn = CrossEntropyLoss::new(None, &device);

    println!("Training on device: {:?}", device);
    println!("Dataset size: {} chars, Vocab size: {}", dataset.tokens.len(), dataset.vocab_size);

    for epoch in 0..num_epochs {
        for i in 0..iterations_per_epoch {
            let batch = dataset.get_batch(batch_size, seq_len, &device);
            
            let outputs = model.forward(batch.tokens);
            
            let [batch_dim, seq_dim, vocab_dim] = outputs.dims();

            let logits = outputs.reshape([batch_dim * seq_dim, vocab_dim]);
            let targets = batch.targets.reshape([batch_dim * seq_dim]);

            let loss = loss_fn.forward(logits, targets);

            let grads = loss.backward();
            let grads = GradientsParams::from_grads(grads, &model);
            
            model = optimizer.step(learning_rate, model, grads);

            if i == iterations_per_epoch - 1 {
                println!("Epoch {} completed - Final Batch Loss: {}", epoch, loss);
            }
        }
    }
    
    std::fs::create_dir_all("models").unwrap();
    let recorder = CompactRecorder::new();
    model.save_file("models/cobalt_model", &recorder).expect("Failed to save model");
    println!("Saved model to models/cobalt_model");
}