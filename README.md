# 📒 Rust Contact Book CLI

A simple, fast, and lightweight **command-line contact manager** written in Rust.
It lets you **add, view, search, and delete** contacts using a clean interactive CLI loop.

---

## 🚀 Features

* ➕ **Add new contacts** (name + phone number)
* 📄 **List all saved contacts**
* 🔍 **Search for a contact** by name
* 🗑️ **Delete a contact**
* 🔁 **Runs in an interactive loop** until user enters `exit`
* ⚡ Extremely fast — powered by Rust’s `HashMap`

---

## 🛠️ How It Works

Contacts are stored inside a `HashMap<String, String>` where:

* **Key** = Contact name
* **Value** = Phone number

Commands supported:

```
add     → Add a new contact  
list    → Show all contacts  
search  → Look up a contact  
remove  → Delete a contact  
exit    → Quit the application  
```

---

## 📦 Installation

Clone the repo:

```sh
git clone https://github.com/iamweird2/cli_app.git
cd cli_app
```

Build the binary:

```sh
cargo build --release
```

Run it:

```sh
cargo run
```

---

## 📤 Publishing (optional)

If you want to publish the crate:

```sh
cargo login
cargo publish
```

Before doing this, ensure your `Cargo.toml` includes:

```toml
[package]
name = "contact_book_cli"
version = "0.1.0"
edition = "2021"
description = "A simple Rust CLI contact manager"
license = "MIT"
repository = "https://github.com/iamweird2/cli_app"
```

---

## 🧩 Example Usage

```
Enter command: add
Enter name: Smart
Enter phone: 07026267991
Contact added!

Enter command: list
Smart : 07026267991

Enter command: exit
Goodbye!
```

---

## 📄 License

This project is licensed under the **MIT License**.
