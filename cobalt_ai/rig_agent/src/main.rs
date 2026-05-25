fn main() {
    println!("Hello, world!");
}
// rig_agent_lab/
// ├── Cargo.toml
// ├── .env                          # API keys (OPENAI_API_KEY, etc.)
// ├── .gitignore
// ├── README.md
// ├── configs/
// │   ├── default.toml
// │   ├── agents.toml
// │   └── providers.toml
// ├── src/
// │   ├── main.rs                   # CLI entry point
// │   ├── lib.rs                    # Shared library exports
// │   ├── agents/
// │   │   ├── mod.rs
// │   │   ├── basic_chat.rs
// │   │   ├── rag_agent.rs
// │   │   └── tool_agent.rs
// │   ├── tools/
// │   │   ├── mod.rs
// │   │   ├── calculator.rs
// │   │   ├── web_search.rs
// │   │   └── file_reader.rs
// │   ├── rag/
// │   │   ├── mod.rs
// │   │   ├── embeddings.rs
// │   │   ├── vector_store.rs
// │   │   └── document_loader.rs
// │   ├── providers/
// │   │   ├── mod.rs
// │   │   └── client.rs
// │   ├── prompts/
// │   │   ├── mod.rs
// │   │   └── templates.rs
// │   └── utils/
// │       ├── mod.rs
// │       └── logger.rs
// ├── data/
// │   ├── documents/                # PDFs, markdown files for RAG
// │   ├── vector_store/             # Persistent vector DB
// │   └── prompts/
// ├── examples/
// │   ├── simple_chat.rs
// │   ├── rag_example.rs
// │   ├── tool_use_example.rs
// │   └── streaming_example.rs
// ├── tests/
// │   ├── integration_test.rs
// │   └── agent_tests.rs
// └── scripts/
//     ├── setup.sh
//     └── run_examples.sh