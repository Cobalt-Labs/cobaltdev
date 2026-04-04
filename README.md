```markdown
# ☁️ Cobalt Cloud

> **Your Private Cloud. Running on your own laptop HDD. Built 100% in Rust.**

After 7 years of grinding — from HTML/CSS → hating JS → Java → Python → Dart/Flutter + FFI → pure Rust + unsafe Rust + Axum microservices + sqlx journal CLI → now building your own cloud infrastructure.

**Cobalt Cloud** turns your internal or external HDD into a real private cloud. Drag & drop files from a beautiful Dioxus client → they land safely on your hard drive in `/home/ibrahim/cloud_storage/users/ibrahim3595/`.

No AWS. No Google. No bills. Just your room, your data, your rules.

This is the **storage backbone** for your full-full-stack dream: modified Linux kernel in Rust, custom ROMs, Bevy games asset loading, AI/LLM backend, and future startup.

![Cobalt Cloud Banner](cobalt_logo.png)

---

## ✨ Features

- **Real HDD Storage** — Powered by `object_store` + Tokio + blake3 checksums for integrity
- **Beautiful Dioxus UI** — Desktop (native feel) + Web (WASM) with modern dark Tailwind theme
- **Drag & Drop Upload** — Live progress bar, success feedback, works on both desktop and web
- **CLI + GUI** — Same fireproof backend powers both (`cargo run -- upload` and Dioxus client)
- **User Isolation** — Files automatically stored per user (`users/ibrahim3595/`)
- **Fast & Secure** — Pure Rust, memory safe, async everything
- **Future Ready** — Easy to scale to "tons of HDDs in the room", S3-compatible API, Flutter mobile bridge

---

## 🛠 Tech Stack

| Layer          | Technology                          | Purpose                              |
|----------------|-------------------------------------|--------------------------------------|
| **Backend**    | Rust + Axum + object_store + sqlx   | API server + real HDD I/O            |
| **Frontend**   | Dioxus 0.7 (Desktop + Web)          | Reactive, cross-platform UI          |
| **Storage**    | Local filesystem + blake3           | Fast, reliable file handling         |
| **Database**   | SQLite (via sqlx)                   | File metadata & tracking             |
| **HTTP Client**| reqwest (rustls-tls)                | Secure calls from frontend           |
| **Styling**    | Tailwind CSS                        | Clean dark modern theme              |

---

## 🚀 Quick Start

### 1. Backend (The Brain)

```bash
cd cobalt_backend

# Create storage directory (internal or external HDD)
mkdir -p ~/cloud_storage/users/ibrahim3595

# Start the server
cargo run -- server
```

### 2. Frontend (The Beautiful Face)

```bash
cd cobalt_cloud

# Desktop version (recommended - native performance)
dx serve --platform desktop

# Web version
dx serve --platform web
```

Open the app → drag any file → watch it land safely on your real HDD.

---

## 📦 Building Releases

```bash
cd cobalt_cloud

# Build native desktop binary
dx build --platform desktop --release

# Build web static files (you can host anywhere)
dx build --platform web --release
```

### Future Release Plans (as you wanted)

- **Android APK** → Connect your existing `cobalt_frontend` Flutter project using `flutter_rust_bridge`
- **Windows .exe** → Build on Windows machine with `dx build --platform desktop --release`
- **Linux .deb** → Use `cargo-deb` for Debian/Ubuntu packages
- **Arch Linux .pkg.tar.zst** → We can create a PKGBUILD when you're ready
- **AppImage** → Easy portable Linux binary

---

## 🧱 Project Structure

```bash
cobaltdev/
├── Cargo.toml                  # Workspace root
├── cobalt_backend/             # Fireproof Rust backend
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   ├── config.rs
│   │   ├── storage.rs
│   │   ├── database.rs
│   │   └── ...
│   └── migrations/
│
└── cobalt_cloud/               # Dioxus client (Desktop + Web)
    ├── Cargo.toml
    ├── Dioxus.toml
    ├── tailwind.css
    ├── src/
    │   ├── main.rs
    │   ├── components/
    │   ├── pages/
    │   ├── services/
    │   ├── hooks/
    │   └── ...
    └── public/
```

---

## 🔮 Your Bigger Vision

Cobalt Cloud is just the beginning of your **full-full-stack empire**.

Next milestones:
- Multi-HDD pooling (tons of drives in your room)
- Full S3-compatible API
- JWT authentication + multi-user support
- Flutter mobile client via FFI
- Bevy game asset loading directly from this cloud
- Integration with your modified Linux kernel in Rust
- AI/LLM data storage backend

**Inshallah** this becomes the solid foundation that gets you that MNC offer or launches your own startup.

---

## ❤️ Built With Passion

From hating JavaScript in the early days to mastering unsafe Rust and building your own cloud infrastructure — this is what consistent grinding looks like.

**Cobalt Cloud** proves that one developer can build what big companies charge millions for.

Made with pure Rust love by **Ibrahim Haji**  
Pune, Maharashtra

Keep the aim high. The journey is long, but the destination is legendary.

Star this repo if you're also building your own empire ⭐

---

## 🤝 Contributing

Contributions, ideas, and feedback are always welcome. Feel free to open issues or pull requests.

## 📧 Contact

For questions or collaboration ideas — just reach out.

---

**Made in Rust • Powered by Grind • Running on Real Hardware**
```
