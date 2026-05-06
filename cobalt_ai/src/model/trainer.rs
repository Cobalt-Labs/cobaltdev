use burn::nn::loss::CrossEntropyLoss;
use burn::optim::decay::WeightDecayConfig;
use burn::optim::{AdamConfig, GradientsParams, Optimizer};
use burn::prelude::*;
use burn::record::CompactRecorder;
use burn::tensor::backend::AutodiffBackend;

use crate::data::loader::TextDataset;
use crate::model::transformer::CobaltModelConfig;

pub fn train<B: AutodiffBackend>(device: B::Device) {
    let mut dataset = TextDataset::new("data/input.txt");

    let batch_size = 12;
    let num_epochs = 12; //max15
    let iterations_per_epoch = 200; //200-350

    let d_model = 192;
    let seq_len = 64;
    let n_heads = 4;
    let n_layers = 3;
    let learning_rate = 3e-4;

    let config = CobaltModelConfig::new(n_heads, n_layers, d_model, dataset.vocab_size, seq_len);
    let mut model: crate::model::transformer::CobaltModel<B> = config.init(&device);

    let optimizer_config =
        AdamConfig::new().with_weight_decay(Some(WeightDecayConfig { penalty: 1e-5 }));
    let mut optimizer = optimizer_config.init();

    let loss_fn = CrossEntropyLoss::new(None, &device);

    println!("Training started on {:?}", device);
    println!(
        "Vocab size: {} | Dataset chars: {}",
        dataset.vocab_size,
        dataset.tokens.len()
    );

    for epoch in 0..num_epochs {
        for i in 0..iterations_per_epoch {
            let batch = dataset.get_batch(batch_size, seq_len, &device);

            let (loss, grads) = {
                let outputs = model.forward(batch.tokens);
                let [b, s, v] = outputs.dims();

                let logits = outputs.reshape([b * s, v]);
                let targets = batch.targets.reshape([b * s]);

                let loss = loss_fn.forward(logits, targets);

                let grads = loss.backward();
                let grads = GradientsParams::from_grads(grads, &model);

                (loss, grads)
            };

            model = optimizer.step(learning_rate, model, grads);

            if i % 20 == 0 || i == iterations_per_epoch - 1 {
                println!("Epoch {:>2} | Iter {:>4} | Loss: {:.4}", epoch, i, loss);
            }
        }
    }

    model
        .clone()
        .save_file("models/cobalt_model", &CompactRecorder::new())
        .expect("Failed to save final model");
    println!("Training finished! Final model saved.");
}
