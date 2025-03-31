# mail-sender# Brevo Email Sender in Rust

## 📌 Overview
A simple Rust-based email sender using the **Brevo API**. It follows a **layered architecture** (Controller → Service → Repository-Model) for maintainability.

## 🏗️ Project Structure
```
├── src/
│   ├── main.rs
│   ├── config.rs  # Loads env variables
│   ├── model/models.rs  # Email request models
│   ├── repository/email_repository.rs  # API requests
│   ├── service/email_service.rs  # Business logic
│   ├── controller/email_controller.rs  # Handles requests
├── .env  # API keys
├── Cargo.toml  # Dependencies
├── README.md  # Documentation
```

## 🚀 Setup
1️⃣ **Clone & Navigate**
```sh
git clone git@github.com:KevinEjiofor/mail-sender-.git
```

2️⃣ **Install Rust & Dependencies**
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo build
```

3️⃣ **Configure .env**
```
BREVO_API_KEY=your_api_key_here
SENDER_EMAIL=your_sender_email@example.com
```

4️⃣ **Run the App**
```sh
cargo run
```

## 📩 Sending an Email
Modify `email_controller.rs` to customize recipient and content.

## 🛠️ Debugging
- Check `.env` for correct API Key.
- Run with `RUST_LOG=debug cargo run` for logs.

## 📜 License
Open-source under the **MIT License**.

🚀 **Happy Coding!**

