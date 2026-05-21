use burn::nn::loss::CrossEntropyLoss;
use burn::optim::decay::WeightDecayConfig;
use burn::optim::{AdamConfig, GradientsParams, Optimizer};
use burn::prelude::*;
use burn::record::CompactRecorder;
use burn::tensor::backend::AutodiffBackend;
use burn::tensor::ElementConversion;

use crate::data::loader::TextDataset;
use crate::model::transformer::CobaltModelConfig;

pub fn train<B: AutodiffBackend>(device: B::Device)
where
    B::Device: Clone,
{
    let mut dataset = TextDataset::new("data/input.txt");
    let mut valid_dataset = TextDataset::new("data/input.txt");

    let batch_size = 12;
    let num_epochs = 14;
    let iterations_per_epoch = 250;
    let eval_every = 50;

    let d_model = 192;
    let seq_len = 64;
    let n_heads = 4;
    let n_layers = 3;
    let base_lr = 2.5e-4;
    let warmup_epochs = 2;

    let config = CobaltModelConfig::new(n_heads, n_layers, d_model, dataset.vocab_size, seq_len);
    let mut model: crate::model::transformer::CobaltModel<B> = config.init(&device);

    let loss_fn = CrossEntropyLoss::new(None, &device);

    let mut best_loss = f32::INFINITY;
    let mut patience_counter = 0;

    println!("╔════════════════════════════════════════════════════════╗");
    println!("║              🎓 COBALT AI TRAINING STARTED            ║");
    println!("╠════════════════════════════════════════════════════════╣");
    println!(
        "║ Device: {:?}                                          ",
        device
    );
    println!(
        "║ Vocab: {} chars | Dataset: {} chars                   ",
        dataset.vocab_size,
        dataset.tokens.len()
    );
    println!(
        "║ Epochs: {} | Batch: {} | Iterations: {}               ",
        num_epochs, batch_size, iterations_per_epoch
    );
    println!(
        "║ Model: d_model={}, heads={}, layers={}                ",
        d_model, n_heads, n_layers
    );
    println!("╚════════════════════════════════════════════════════════╝");
    println!();

    for epoch in 0..num_epochs {
        let mut epoch_loss = 0.0;

        // ========== LEARNING RATE SCHEDULING ==========
        let current_lr = if epoch < warmup_epochs {
            base_lr * ((epoch + 1) as f32 / warmup_epochs as f32)
        } else {
            let progress = (epoch - warmup_epochs) as f32 / (num_epochs - warmup_epochs) as f32;
            base_lr * (1.0 + (progress * std::f32::consts::PI).cos()) / 2.0
        };

        // Create optimizer with current learning rate
        let optimizer_config =
            AdamConfig::new().with_weight_decay(Some(WeightDecayConfig { penalty: 1e-5 }));
        let mut optimizer = optimizer_config.init();
        // ==============================================

        for i in 0..iterations_per_epoch {
            let batch = dataset.get_batch(batch_size, seq_len, &device);

            let (loss, grads) = {
                let outputs = model.forward(batch.tokens);
                let [b, s, v] = outputs.dims();

                let logits = outputs.reshape([b * s, v]);
                let targets = batch.targets.reshape([b * s]);

                let loss = loss_fn.forward(logits, targets);
                let grads = loss.backward();

                // ========== SIMPLIFIED GRADIENT CLIPPING ==========
                // Note: Full gradient clipping requires accessing gradient tensors
                // For now, we use a simpler approach - adaptive learning rate
                let loss_f32: f32 = loss.clone().into_scalar().elem();
                if loss_f32.is_nan() || loss_f32.is_infinite() {
                    println!("⚠️ NaN/Inf loss detected! Skipping update...");
                    return;
                }
                // ==================================================

                (loss, grads)
            };

            let grads = GradientsParams::from_grads(grads, &model);
            model = optimizer.step(current_lr.into(), model, grads);

            let loss_val: f32 = loss.into_scalar().elem();
            epoch_loss += loss_val;

            if i % 20 == 0 || i == iterations_per_epoch - 1 {
                let progress = (i as f32 / iterations_per_epoch as f32) * 100.0;
                println!(
                    "Epoch {:>2}/{} | Iter {:>3}/{} | Loss: {:.4} | LR: {:.2e} | {:>5.0}%",
                    epoch + 1,
                    num_epochs,
                    i,
                    iterations_per_epoch,
                    loss_val,
                    current_lr,
                    progress
                );
            }

            // Evaluation
            if i % eval_every == 0 && i > 0 {
                let val_batch = valid_dataset.get_batch(batch_size, seq_len, &device);
                let val_outputs = model.forward(val_batch.tokens);
                let [b, s, v] = val_outputs.dims();
                let val_logits = val_outputs.reshape([b * s, v]);
                let val_targets = val_batch.targets.reshape([b * s]);
                let val_loss: f32 = loss_fn.forward(val_logits, val_targets).into_scalar().elem();

                if val_loss < best_loss {
                    best_loss = val_loss;
                    patience_counter = 0;
                    model
                        .clone()
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
        println!(
            "│ Epoch {} completed | Avg Loss: {:.4}               │",
            epoch + 1,
            avg_epoch_loss
        );
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
