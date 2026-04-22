use burn::tensor::backend::Backend;
use burn::tensor::{Int, Tensor, TensorData};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
pub struct CobaltBatch<B: Backend> {
    pub tokens: Tensor<B, 2, Int>,
    pub targets: Tensor<B, 2, Int>,
}

#[derive(Clone, Debug)]
pub struct CharTokenizer {
    pub char_to_id: HashMap<char, i32>,
    pub id_to_char: HashMap<i32, char>,
    pub vocab_size: usize,
}

impl CharTokenizer {
    pub fn new(text: &str) -> Self {
        let mut chars: Vec<char> = text.chars().collect::<HashSet<_>>().into_iter().collect();
        chars.sort(); 
        
        let mut char_to_id = HashMap::new();
        let mut id_to_char = HashMap::new();
        
        for (i, &c) in chars.iter().enumerate() {
            let id = i as i32;
            char_to_id.insert(c, id);
            id_to_char.insert(id, c);
        }
        
        Self {
            vocab_size: chars.len(),
            char_to_id,
            id_to_char,
        }
    }

    pub fn encode(&self, text: &str) -> Vec<i32> {
        text.chars().map(|c| *self.char_to_id.get(&c).unwrap_or(&0)).collect()
    }

    pub fn decode(&self, ids: &[i32]) -> String {
        ids.iter().map(|id| *self.id_to_char.get(id).unwrap_or(&'?')).collect()
    }
}

pub struct TextDataset {
    pub tokens: Vec<i32>,
    pub vocab_size: usize,
    pub tokenizer: CharTokenizer,
    seed: usize,
}

impl TextDataset {
    pub fn new(file_path: &str) -> Self {
        let content = std::fs::read_to_string(file_path).unwrap_or_else(|_| "hello world, this provides some basic char data".to_string());
        
        let tokenizer = CharTokenizer::new(&content);
        let tokens = tokenizer.encode(&content);
        let vocab_size = tokenizer.vocab_size;

        Self { 
            tokens,
            vocab_size,
            tokenizer,
            seed: 12345,
        }
    }

    pub fn get_batch<B: Backend>(&mut self, batch_size: usize, seq_len: usize, device: &B::Device) -> CobaltBatch<B> {
        let max_idx = self.tokens.len().saturating_sub(seq_len + 1);
        if max_idx == 0 {
            let inputs = Tensor::<B, 2, Int>::zeros([batch_size, seq_len], device);
            let targets = Tensor::<B, 2, Int>::zeros([batch_size, seq_len], device);
            return CobaltBatch { tokens: inputs, targets };
        }
        
        let mut input_data = Vec::with_capacity(batch_size * seq_len);
        let mut target_data = Vec::with_capacity(batch_size * seq_len);
        
        for _ in 0..batch_size {
            // Cheap psuedorandom seed jump
            self.seed = self.seed.wrapping_add(1308721);
            let start_idx = self.seed % max_idx;
            let end_idx = start_idx + seq_len;
            
            input_data.extend_from_slice(&self.tokens[start_idx..end_idx]);
            target_data.extend_from_slice(&self.tokens[start_idx+1..end_idx+1]);
        }

        let inputs = Tensor::<B, 1, Int>::from_data(TensorData::new(input_data, [batch_size * seq_len]), device).reshape([batch_size, seq_len]);
        let targets = Tensor::<B, 1, Int>::from_data(TensorData::new(target_data, [batch_size * seq_len]), device).reshape([batch_size, seq_len]);
        
        CobaltBatch {
            tokens: inputs,
            targets,
        }
    }
}