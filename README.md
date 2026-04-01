# ⚡ CobaltDev

A modular full-stack system focused on backend infrastructure, cloud storage, and a modern web interface.

---

## 🚀 Overview

CobaltDev is a multi-component project built to explore **scalable backend systems**, **local object storage**, and **clean API design**, paired with a frontend portfolio and service platform.

This project is not just a demo — it's being shaped into a **production-grade architecture**.

---

## 🧩 Project Structure

```
cobaltdev/
├── cobalt_backend/   # Core backend (Rust + Axum)
├── cobalt_cloud/     # Local object storage system (HDD-based)
├── cobalt_web/       # Flutter web app (portfolio + services)
```

---

## ⚙️ Components

### 🔹 cobalt_backend

Backend service built with:

* Rust (performance + safety)
* Axum (web framework)
* SQLx (database layer)
* SQLite (current DB)
* Argon2 (password hashing)
* Lettre (email services)

**Responsibilities:**

* API layer
* Authentication & authorization
* Email workflows
* File handling integration with storage

---

### 🔹 cobalt_cloud

A custom local storage layer powered by:

* HDD-based file system storage
* Object-style storage design (inspired by S3)

**Goal:**

* Replace dependency on external cloud providers
* Build a **self-hosted storage system**

---

### 🔹 cobalt_web

Frontend built using Flutter.

🌐 Live: https://cobaltdev.vercel.app

**Features:**

* Personal portfolio
* Service showcase
* Entry point for users

---

## 🧠 Architecture Vision

* Backend-first design
* Storage abstraction (local → cloud-ready)
* Clean separation of concerns
* Async + scalable systems

---

## 🔮 Future Plans

* PostgreSQL migration (replace SQLite)
* Dockerized deployment
* Reverse proxy setup (Nginx / Traefik)
* Auth improvements (JWT + refresh tokens)
* File upload pipeline optimization
* CDN integration
* Background job processing (queues)
* Observability (tracing + metrics)

---

## 🛠️ Tech Stack

**Backend**

* Rust
* Axum
* SQLx
* Tokio

**Frontend**

* Flutter (Web)
* Dioxus (Rust)

**Storage**

* Local filesystem (HDD)
* Object storage abstraction

**Infra (planned)**

* Docker
* Nginx / Traefik
* PostgreSQL

---

## 📌 Philosophy

CobaltDev is built with a focus on:

* Performance over convenience
* Explicit control over abstractions
* Learning by building real systems
* Avoiding unnecessary dependencies

---

## ⚠️ Status

This project is actively evolving.

Expect breaking changes, refactors, and architectural improvements.

---

## 👤 Author

Ibrahim
Building systems, not just apps.

---
