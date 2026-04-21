use burn::optim::{AdamConfig, Optimizer, GradientsParams};
use burn::tensor::backend::AutodiffBackend;
use burn::prelude::*;
use burn::tensor::Int;
use burn::nn::loss::CrossEntropyLoss;

use crate::model::transformer::CobaltModelConfig;

pub fn train<B: AutodiffBackend>(device: B::Device) {
    let config = CobaltModelConfig::new(8, 6, 512, 5000, 512);

    let mut model = config.init(&device);
    let mut optimizer = AdamConfig::new().init();

    let loss_fn = CrossEntropyLoss::new(None, &device);

    let num_epochs = 10;
    let learning_rate = 1e-4;

    println!("Training on device: {:?}", device);

    for epoch in 0..num_epochs {
        let inputs = Tensor::<B, 2, Int>::zeros([32, 512], &device);
        let targets = Tensor::<B, 2, Int>::zeros([32, 512], &device);

        let outputs = model.forward(inputs);

        let [batch, seq, vocab] = outputs.dims();

        let logits = outputs.reshape([batch * seq, vocab]);
        let targets = targets.reshape([batch * seq]);

        let loss = loss_fn.forward(logits, targets);

        let grads = loss.backward();

        let grads = GradientsParams::from_grads(grads, &model);

        model = optimizer.step(learning_rate, model, grads);

        println!("Epoch {} - Loss: {:?}", epoch, loss);
    }
}