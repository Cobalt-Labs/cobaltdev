use burn::data::dataloader::batcher::Batcher;
use burn::tensor::backend::Backend;
use burn::tensor::{Int, Tensor};

#[derive(Clone)]
pub struct CobaltBatch<B: Backend> {
    pub tokens: Tensor<B, 2, Int>,
    pub targets: Tensor<B, 2, Int>,
}

pub struct CobaltBatcher<B: Backend> {
    device: B::Device,
}

impl<B: Backend> CobaltBatcher<B> {
    pub fn new(device: B::Device) -> Self {
        Self { device }
    }
}

impl<B: Backend> Batcher<B, String, CobaltBatch<B>> for CobaltBatcher<B> {
    fn batch(&self, _items: Vec<String>, device: &B::Device) -> CobaltBatch<B> {
        let tokens = Tensor::zeros([1, 512], device);
        let targets = Tensor::zeros([1, 512], device);

        CobaltBatch { tokens, targets }
    }
}