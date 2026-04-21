use burn::train::LearnerBuilder;
use burn::train::metric::{AccuracyMetric, LossMetric};
use burn::optim::AdamConfig;
use crate::model::transformer::CobaltModelConfig;
use burn::prelude::*;

pub fn train<B: AutodiffBackend>(device: B::Device) {
    let config = CobaltModelConfig::new(8, 6, 512, 5000, 512); // Heads, Layers, D_Model, Vocab, Seq
    let optimizer = AdamConfig::new().init();
    
    let artifacts_path = "./data/checkpoints";

    let learner = LearnerBuilder::new(artifacts_path)
        .metric_train_numeric(LossMetric::new())
        .metric_valid_numeric(LossMetric::new())
        .with_file_checkpointer(1) // Save every epoch
        .devices(vec![device.clone()])
        .num_epochs(10)
        .build(
            config.init(&device),
            optimizer,
            1e-4,
        );

    println!("Trainer initialized on device: {:?}", device);
}