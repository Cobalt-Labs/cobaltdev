use burn::nn::loss::CrossEntropyLoss;
use burn::optim::decay::WeightDecayConfig;
use burn::optim::{AdamConfig, GradientsParams, Optimizer};
use burn::prelude::*;
use burn::record::CompactRecorder;

use crate::data::loader::TextDataset;
use crate::model::transformer::CobaltModelConfig;

pub fn train<B: AutodiffBackend>(device: B::Device) {
    let mut dataset = TextDataset::new("data/input.txt");
    let mut valid_dataset = TextDataset::new("data/input.txt");

    let batch_size = 12;
    let num_epochs = 14;
    let iterations_per_epoch = 250;
    let eval_every = 50;

    let d_model = 384;
    let seq_len = 128;
    let n_heads = 6;
    let n_layers = 4;
    let learning_rate = 1.5e-4;

    let config = CobaltModelConfig::new(n_heads, n_layers, d_model, dataset.vocab_size, seq_len);
    let mut model: crate::model::transformer::CobaltModel<B> = config.init(&device);

    let optimizer_config =
        AdamConfig::new().with_weight_decay(Some(WeightDecayConfig { penalty: 1e-5 }));
    let mut optimizer = optimizer_config.init();

    let loss_fn = CrossEntropyLoss::new(None, &device);
    
    let mut best_loss = f32::INFINITY;
    let mut patience_counter = 0;

    println!("╔════════════════════════════════════════════════════════╗");
    println!("║              🎓 COBALT AI TRAINING STARTED            ║");
    println!("╠════════════════════════════════════════════════════════╣");
    println!("║ Device: {:?}                                          ", device);
    println!("║ Vocab: {} chars | Dataset: {} chars                   ", dataset.vocab_size, dataset.tokens.len());
    println!("║ Epochs: {} | Batch: {} | Iterations: {}               ", num_epochs, batch_size, iterations_per_epoch);
    println!("║ Model: d_model={}, heads={}, layers={}                ", d_model, n_heads, n_layers);
    println!("╚════════════════════════════════════════════════════════╝");
    println!();

    for epoch in 0..num_epochs {
        let mut epoch_loss = 0.0;
        
        for i in 0..iterations_per_epoch {
            let batch = dataset.get_batch(batch_size, seq_len, &device);

            let (loss, grads) = {
                let outputs = model.forward(batch.tokens);
                let [b, s, v] = outputs.dims();

                let logits = outputs.reshape([b * s, v]);
                let targets = batch.targets.reshape([b * s]);

                let loss = loss_fn.forward(logits, targets);
                let grads = loss.backward();
                
                (loss, grads)
            };

            let grads = GradientsParams::from_grads(grads, &model);
            model = optimizer.step(learning_rate, model, grads);
            
            let loss_val = loss.into_scalar();
            epoch_loss += loss_val;

            if i % 20 == 0 || i == iterations_per_epoch - 1 {
                let progress = (i as f32 / iterations_per_epoch as f32) * 100.0;
                println!("Epoch {:>2}/{} | Iter {:>3}/{} | Loss: {:.4} | {:>5.0}%", 
                    epoch+1, num_epochs, i, iterations_per_epoch, loss_val, progress);
            }
            
            if i % eval_every == 0 && i > 0 {
                let val_batch = valid_dataset.get_batch(batch_size, seq_len, &device);
                let val_outputs = model.forward(val_batch.tokens);
                let [b, s, v] = val_outputs.dims();
                let val_logits = val_outputs.reshape([b * s, v]);
                let val_targets = val_batch.targets.reshape([b * s]);
                let val_loss = loss_fn.forward(val_logits, val_targets).into_scalar();
                
                if val_loss < best_loss {
                    best_loss = val_loss;
                    patience_counter = 0;
                    model.clone()
                        .save_file("models/cobalt_model_best", &CompactRecorder::new())
                        .expect("Failed to save best model");
                    println!("  ✓ New best model! Loss: {:.4}", best_loss);
                } else {
                    patience_counter += 1;
                }
                
                if patience_counter > 10 {
                    println!("\n⚠️ Early stopping triggered! Best loss: {:.4}", best_loss);
                    break;
                }
            }
        }
        
        let avg_epoch_loss = epoch_loss / iterations_per_epoch as f32;
        println!("┌────────────────────────────────────────────────────┐");
        println!("│ Epoch {} completed | Avg Loss: {:.4}               │", epoch+1, avg_epoch_loss);
        println!("└────────────────────────────────────────────────────┘");
        println!();
    }

    model
        .clone()
        .save_file("models/cobalt_model_final", &CompactRecorder::new())
        .expect("Failed to save final model");
    
    println!("✅ Training complete!");
    println!("📁 Models saved to:");
    println!("   - models/cobalt_model_best");
    println!("   - models/cobalt_model_final");
}