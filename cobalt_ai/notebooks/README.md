# Cobalt AI — Jupyter Notebooks 📓

This directory contains a suite of interactive tools for analyzing, visualizing, and interacting with the Cobalt Transformer model.

## 🚀 Quick Start

1. **Install Dependencies**:
   ```bash
   pip install -r requirements.txt
   ```

2. **Launch Jupyter**:
   ```bash
   jupyter lab
   # or
   jupyter notebook
   ```

## 📂 Notebook Catalog

| Notebook | Description |
| :--- | :--- |
| [`cobalt_transformer_demo.ipynb`](./cobalt_transformer_demo.ipynb) | **Main Demo**. End-to-end flow from config to live Rust inference. |
| [`training_metrics.ipynb`](./training_metrics.ipynb) | **Training Analytics**. Deep dive into loss curves and convergence rates. |
| [`model_architecture.ipynb`](./model_architecture.ipynb) | **Deep Dive**. Visualizing attention masks, positional encodings, and residual flows. |

## 🛠️ Shared Utilities

All notebooks import [`cobalt_utils.py`](./cobalt_utils.py), which provides:
- **On-Brand Plotting**: Consistent dark-mode aesthetics for all charts.
- **Config Management**: Authorsitative loading of model hyperparameters.
- **Rust Bridge**: Subprocess wrappers to call the `cobalt_ai` binary for inference.
- **Analytics**: Math helpers for parameter counting and log processing.

## 🦀 Rust Integration

The notebooks call the compiled Rust binary at `../../target/release/cobalt_ai`. 
Ensure you have built the project before running inference:
```bash
cargo build --release --manifest-path ../Cargo.toml
```

---
**Cobalt Labs** • *Built in Rust. Trained from scratch.*
