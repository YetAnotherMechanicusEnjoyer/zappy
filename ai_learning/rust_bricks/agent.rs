use serde::{Deserialize, Serialize};
use std::fs;
use ndarray::Array1;
use ndarray_npy::read_npy;

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelConfig {
    pub layer_dims: Vec<usize>,
    pub state_dim: usize,
    pub n_actions: usize,
    pub team: String,
}

pub struct NeuralAgent {
    pub config: ModelConfig,
    pub genome: Vec<f32>,
}

// Json arch load

pub fn load_config(path: &str) -> anyhow::Result<ModelConfig> {
    let content = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&content)?)
}

// Npy genome file load

pub fn load_genome(path: &str) -> anyhow::Result<Vec<f32>> {
    let arr: Array1<f32> = read_npy(path)?;
    Ok(arr.to_vec())
}

// MLP defnition
fn linear(weights: &[f32],
          bias: &[f32],
          input: &[f32],
          out_dim: usize,
          in_dim: usize,
         ) -> Vec<f32> {
    let mut output = vec![0.0; out_dim];

    for o in 0..out_dim {
        let mut sum = bias[o];

        for i in 0..in_dim {
            sum += weights[o * in_dim + i] * input[i];
        }
        output[o] = sum;
    }
    output
}

fn relu(v: &mut [f32]) {
    for x in v {
        *x = x.max(0.0);
    }
}

// Occult non valid action base on valid mask

pub fn masked_argmax(
    logits: &[f32],
    valid_mask: &[bool],
) -> usize {

    let mut best_idx = 0;
    let mut best_score = f32::NEG_INFINITY;

    for i in 0..logits.len() {
        let score = if valid_mask[i] {
            logits[i]
        } else {
            f32::NEG_INFINITY
        };

        if score > best_score {
            best_score = score;
            best_idx = i;
        }
    }

    best_idx
}

// verify genome size (if compatible with layers put in the arch file)
fn expected_genome_size(dims: &[usize]) -> usize {
    dims.windows(2)
        .map(|w| w[0] * w[1] + w[1])
        .sum()
}

impl NeuralAgent {
    pub fn load(json_path: &str, npy_path: &str) -> anyhow::Result<Self> {
        let config = load_config(json_path)?;
        let genome = load_genome(npy_path)?;

        let expected = expected_genome_size(&config.layer_dims);
        if genome.len() != expected {
            anyhow::bail!(
                "Expected genome size {}, got {}",
                expected,
                genome.len()
            );
        }

        Ok(Self {config, genome})
    }

    pub fn forward(&self, state: &[f32]) -> Vec<f32> {
        let mut x = state.to_vec();
        let mut idx = 0;

        for layer in 0..self.config.layer_dims.len() - 1 {
            let in_dim = self.config.layer_dims[layer];
            let out_dim = self.config.layer_dims[layer + 1];

            let weight_count = in_dim * out_dim;
            let weights = &self.genome[idx..idx + weight_count];
            idx += weight_count;

            let bias = &self.genome[idx..idx + out_dim];
            idx += out_dim;

            let mut y = linear(weights, bias, &x, out_dim, in_dim);

            let is_last = layer == (self.config.layer_dims.len() - 2);

            if !is_last {
                relu(&mut y);
            }
            x = y;
        }
        x
    }

    pub fn act(&self, state: &[f32], valid_mask: &[bool]) -> usize {
        let logits = self.forward(state);

        masked_argmax(&logits, valid_mask)
    }
}

// compute all example
let ai = NeuralAgent::load(
    "arch.json",
    "best_genome.npy",
)?;
