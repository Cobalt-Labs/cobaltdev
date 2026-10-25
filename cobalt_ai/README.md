# Cobalt AI 

Welcome to the **Cobalt AI** repository! This project demonstrates how to build robust, scalable AI applications using Rust and the [Rig](https://github.com/0xPlaygrounds/rig) framework.

## Project Structure
This directory is structured to separate different concerns of a complex AI system, allowing for apps (like CLIs and APIs) and crates (for specialized AI components like RAG, embeddings, tools, etc.):

```text
cobalt_ai/
├── apps/               # Executable applications (Axum API, Dioxus Desktop, CLI)
├── crates/             # Modular Rust crates (inference, RAG, agents, tools)
├── datasets/           # Raw, processed, and tokenized datasets
├── models/             # Local LLM models, LoRA checkpoints, and configs
├── vector_store/       # Vector database data (e.g., Qdrant)
├── rig_agent/          # Minimal baseline Rig Agent implementations
└── scripts/            # Build, deployment, and testing scripts
```

## Barebones `rig_agent`
We've stripped down the `rig_agent` inside this repo to its absolute bare minimum. It does exactly what it's meant to do: act as a boilerplate for building AI logic in Rust without unnecessary bloat.
You can run it immediately if you have an `OPENAI_API_KEY` in your `.env`:

```bash
cd rig_agent
cargo run
```

---

## Your Development Roadmap (8-Commit Plan)
Ready to build this out yourself? Follow this roadmap step-by-step. Each step represents a distinct feature you can implement and commit.

- [ ] **Commit 1: Setup Workspace & Core Architecture**
  - *Goal*: Convert the root `Cargo.toml` into a Cargo Workspace that manages the newly created `apps/` and `crates/` directories.
  - *Action*: Add `[workspace]` and define workspace members.

- [ ] **Commit 2: Enhance the Basic Agent (System Prompts & Memory)**
  - *Goal*: Expand the barebones `rig_agent` to support dynamic system prompts and conversation history.
  - *Action*: Update the agent builder in `main.rs` to retain chat context across loops.

- [ ] **Commit 3: Implement Custom Tools (Function Calling)**
  - *Goal*: Give your AI agency. 
  - *Action*: Create a simple Rust tool (e.g., a file reader or external API caller), implement the Rig `Tool` trait, and attach it to your agent.

- [ ] **Commit 4: Create a RAG Pipeline (`crates/cobalt-rag`)**
  - *Goal*: Allow the AI to read your local documents.
  - *Action*: Write a small crate that parses text files, chunks them, and generates embeddings.

- [ ] **Commit 5: Connect Local Vector Database**
  - *Goal*: Make the RAG pipeline scalable.
  - *Action*: Integrate a local Qdrant vector store to save and query your document embeddings.

- [ ] **Commit 6: Spin Up the Axum API Server (`apps/api_server`)**
  - *Goal*: Move away from the CLI and serve your agent via HTTP.
  - *Action*: Initialize a new Axum project and create a `/chat` POST endpoint that streams agent responses.

- [ ] **Commit 7: Build the Desktop UI (`apps/chat_desktop`)**
  - *Goal*: Give your AI a face.
  - *Action*: Use Dioxus to create a cross-platform desktop UI that connects to your Axum backend.

- [ ] **Commit 8: Containerization & Deployment Setup**
  - *Goal*: Prepare the AI backend for production.
  - *Action*: Write a `Dockerfile` and a `docker-compose.yml` that spins up both your API server and the Qdrant database.

Happy coding!
