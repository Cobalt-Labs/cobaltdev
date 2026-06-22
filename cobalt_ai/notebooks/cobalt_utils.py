"""
cobalt_utils.py
───────────────────────────────────────────────────────────────────────────────
Shared utilities for all Cobalt AI notebooks.
Handles: config loading, training log parsing, Rust inference calls,
         pretty printing, and plotting helpers.
───────────────────────────────────────────────────────────────────────────────
"""

import json
import subprocess
import sys
from pathlib import Path

import matplotlib.pyplot as plt
import matplotlib.ticker as mticker
import numpy as np


# ── Paths ──────────────────────────────────────────────────────────────────
NOTEBOOK_DIR = Path(__file__).resolve().parent
ROOT_DIR     = NOTEBOOK_DIR.parent.parent          # cobaltdev/
AI_DIR       = NOTEBOOK_DIR.parent                 # cobalt_ai/
MODEL_PATH   = AI_DIR / "models" / "cobalt_model.mpk"
CONFIG_PATH  = AI_DIR / "models" / "config.json"
LOGS_PATH    = AI_DIR / "experiments" / "training_logs.json"
BINARY_PATH  = ROOT_DIR / "target" / "release" / "cobalt_ai"


# ── Colour Palette ─────────────────────────────────────────────────────────
COBALT_BLUE   = "#0057FF"
COBALT_CYAN   = "#00C8FF"
COBALT_PURPLE = "#7B2FFF"
COBALT_BG     = "#0D0F1A"
COBALT_PANEL  = "#151828"
COBALT_TEXT   = "#E0E6FF"
COBALT_GRID   = "#1E2340"


# ── Plotting theme ──────────────────────────────────────────────────────────
def apply_cobalt_theme() -> None:
    """Apply a dark, on-brand Cobalt theme to all matplotlib plots."""
    plt.rcParams.update({
        "figure.facecolor":  COBALT_BG,
        "axes.facecolor":    COBALT_PANEL,
        "axes.edgecolor":    COBALT_GRID,
        "axes.labelcolor":   COBALT_TEXT,
        "axes.titlecolor":   COBALT_TEXT,
        "axes.titlesize":    15,
        "axes.labelsize":    12,
        "axes.grid":         True,
        "grid.color":        COBALT_GRID,
        "grid.linewidth":    0.7,
        "xtick.color":       COBALT_TEXT,
        "ytick.color":       COBALT_TEXT,
        "text.color":        COBALT_TEXT,
        "legend.facecolor":  COBALT_PANEL,
        "legend.edgecolor":  COBALT_GRID,
        "legend.fontsize":   10,
        "figure.dpi":        130,
        "lines.linewidth":   2.2,
        "font.family":       "monospace",
    })


# ── Config ──────────────────────────────────────────────────────────────────
def load_config(path: Path = CONFIG_PATH) -> dict | None:
    """Load and return the model config.json."""
    if not path.exists():
        print(f"⚠  config.json not found at {path}")
        return None
    with open(path) as f:
        cfg = json.load(f)
    return cfg


def print_config(cfg: dict) -> None:
    """Pretty-print model configuration."""
    print("╔══════════════════════════════════════════════╗")
    print(f"║  {'Cobalt AI — Model Configuration':^44} ║")
    print("╠══════════════════════════════════════════════╣")
    arch = cfg.get("architecture", {})
    train = cfg.get("training", {})
    rows = [
        ("Model",        cfg.get("model_name", "?")),
        ("Version",      cfg.get("version", "?")),
        ("Framework",    cfg.get("framework", "?")),
        ("─" * 20, "─" * 22),
        ("n_heads",      arch.get("n_heads")),
        ("n_layers",     arch.get("n_layers")),
        ("d_model",      arch.get("d_model")),
        ("d_ff",         arch.get("d_ff")),
        ("max_seq_len",  arch.get("max_seq_len")),
        ("─" * 20, "─" * 22),
        ("optimizer",    train.get("optimizer")),
        ("lr",           train.get("learning_rate")),
        ("epochs",       train.get("num_epochs")),
        ("batch_size",   train.get("batch_size")),
        ("loss",         train.get("loss_function")),
        ("activation",   train.get("activation")),
    ]
    for k, v in rows:
        print(f"║  {str(k):<20}  {str(v):<22} ║")
    print("╚══════════════════════════════════════════════╝")


# ── Training Logs ───────────────────────────────────────────────────────────
def load_training_logs(path: Path = LOGS_PATH) -> list[dict] | None:
    """Load training_logs.json; returns list of dicts."""
    if not path.exists():
        print(f"⚠  training_logs.json not found at {path}")
        return None
    with open(path) as f:
        return json.load(f)


def plot_loss_curve(logs: list[dict], smooth: bool = True) -> None:
    """Plot training loss curve from logs."""
    apply_cobalt_theme()

    iters  = list(range(len(logs)))
    losses = [e["loss"] for e in logs]

    fig, ax = plt.subplots(figsize=(12, 5))

    # Raw loss (faint)
    ax.plot(iters, losses, color=COBALT_BLUE, alpha=0.35, linewidth=1.2, label="Raw Loss")

    # Smoothed
    if smooth and len(losses) > 5:
        window = max(3, len(losses) // 10)
        kernel = np.ones(window) / window
        smoothed = np.convolve(losses, kernel, mode="valid")
        x_smooth = iters[window - 1:]
        ax.plot(x_smooth, smoothed, color=COBALT_CYAN, linewidth=2.5, label=f"Smoothed (w={window})")

    # Min loss annotation
    min_idx = int(np.argmin(losses))
    ax.annotate(
        f"  min: {losses[min_idx]:.4f}",
        xy=(min_idx, losses[min_idx]),
        color=COBALT_PURPLE,
        fontsize=9,
        fontweight="bold",
    )
    ax.scatter([min_idx], [losses[min_idx]], color=COBALT_PURPLE, zorder=5, s=60)

    ax.set_title("⚡ Cobalt Transformer — Training Loss Curve", fontsize=16, pad=14)
    ax.set_xlabel("Log Step")
    ax.set_ylabel("Cross-Entropy Loss")
    ax.legend()
    ax.yaxis.set_major_formatter(mticker.FormatStrFormatter("%.3f"))
    plt.tight_layout()
    plt.show()


# ── Rust Inference ──────────────────────────────────────────────────────────
def run_rust_inference(
    prompt: str,
    temperature: float = 0.8,
    max_tokens: int = 100,
    binary: Path = BINARY_PATH,
) -> str | None:
    """
    Call the Rust cobalt_ai binary for text generation.
    Falls back to a friendly error if the binary is not built.
    """
    if not binary.exists():
        print(f"⚠  Binary not found at {binary}")
        print("   Build it first:  cargo build --release --manifest-path cobalt_ai/Cargo.toml")
        return None

    cmd = [
        str(binary),
        "generate",
        prompt,
    ]
    print(f"🦀 Running: {' '.join(cmd)}")
    result = subprocess.run(cmd, capture_output=True, text=True, timeout=60)

    if result.returncode == 0:
        out = result.stdout.strip()
        print(f"🧠 Output:\n{out}")
        return out
    else:
        print(f"❌ Rust error (exit {result.returncode}):\n{result.stderr.strip()}")
        return None


# ── Parameter Counter ───────────────────────────────────────────────────────
def count_parameters(cfg: dict) -> dict:
    """
    Estimate parameter count from config without loading the model.
    Returns a breakdown dict.
    """
    arch = cfg.get("architecture", {})
    d    = arch.get("d_model", 0)
    n_h  = arch.get("n_heads", 0)
    n_l  = arch.get("n_layers", 0)
    d_ff = arch.get("d_ff", 0)
    V    = arch.get("vocab_size") or 65        # Shakespeare default
    S    = arch.get("max_seq_len", 64)

    tok_emb  = V * d
    pos_emb  = S * d
    # Per transformer block: MHA (4 projections) + 2×LN + FFN (2 linears)
    mha      = 4 * d * d          # Q, K, V, O projections
    ln       = 2 * (2 * d)        # 2× LayerNorm (weight + bias each)
    ffn      = d * d_ff + d_ff * d
    per_block = mha + ln + ffn
    blocks   = n_l * per_block
    out_head = d * V

    total = tok_emb + pos_emb + blocks + out_head

    return {
        "token_embedding":   tok_emb,
        "position_embedding": pos_emb,
        f"transformer_blocks (×{n_l})": blocks,
        "output_head":       out_head,
        "TOTAL":             total,
    }


def print_param_breakdown(cfg: dict) -> None:
    """Pretty-print parameter count breakdown."""
    breakdown = count_parameters(cfg)
    total = breakdown.pop("TOTAL")
    print("╔══════════════════════════════════════════════╗")
    print(f"║  {'Parameter Breakdown':^44} ║")
    print("╠══════════════════════════════════════════════╣")
    for name, count in breakdown.items():
        pct = count / total * 100
        print(f"║  {name:<28}  {count:>8,}  ({pct:4.1f}%) ║")
    print("╠══════════════════════════════════════════════╣")
    print(f"║  {'TOTAL':^28}  {total:>8,}          ║")
    print("╚══════════════════════════════════════════════╝")


# ── Sanity check ────────────────────────────────────────────────────────────
if __name__ == "__main__":
    print(f"Python  : {sys.version.split()[0]}")
    print(f"Root    : {ROOT_DIR}")
    print(f"Model   : {MODEL_PATH}  ({'✅ exists' if MODEL_PATH.exists() else '⚠ not found'})")
    print(f"Binary  : {BINARY_PATH} ({'✅ exists' if BINARY_PATH.exists() else '⚠ not built'})")
    cfg = load_config()
    if cfg:
        print_config(cfg)
        print_param_breakdown(cfg)
