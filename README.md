# 🐝 Hive CLI

<h3 align="center">
Your local development environment, orchestrated.
</h3>

<p align="center">
A powerful CLI tool for managing, running and exposing your applications.
</p>

<p align="center">

[![Build](https://github.com/HiveSofts/hive-cli/actions/workflows/build.yml/badge.svg)](https://github.com/HiveSofts/hive-cli/actions)
[![Release](https://img.shields.io/github/v/release/HiveSofts/hive-cli)](https://github.com/HiveSofts/hive-cli/releases)

</p>


---

## 🚀 About Hive

Hive CLI is a developer-focused command line tool designed to simplify local development workflows.

It helps you initialize projects, manage environments, run services, handle secrets, view logs and expose local applications with a simple command.

Built with Rust for speed, reliability and cross-platform support.

---

## ✨ Features

- 🐝 Project initialization
- ⚡ Fast command execution
- 🔥 Environment management
- 🔐 Secrets management
- 📜 Logs management
- 🚀 Local application runner
- 🌍 Public tunnel exposure
- 📦 Project registry
- 🩺 Environment diagnostics
- 🖥 Cross-platform support


---

## 📦 Installation


### Linux / macOS

```bash
curl -fsSL https://raw.githubusercontent.com/HiveSofts/hive-cli/main/install/install.sh | bash
````

### Windows

PowerShell:

```powershell
irm https://raw.githubusercontent.com/HiveSofts/hive-cli/main/install/install.ps1 | iex
```

Verify installation:

```bash
hive --version
```

---

## 🛠 Usage

Initialize Hive in your project:

```bash
hive init
```

Run application:

```bash
hive run
```

Run specific script:

```bash
hive run dev
```

Check project status:

```bash
hive status
```

Manage environment:

```bash
hive env
```

Manage secrets:

```bash
hive secrets
```

Expose local service:

```bash
hive expose
```

View logs:

```bash
hive logs
```

---

## 📚 Commands

| Command      | Description                        |
| ------------ | ---------------------------------- |
| `init`       | Initialize Hive in current project |
| `run`        | Run project scripts                |
| `logs`       | View application logs              |
| `env`        | Manage environment variables       |
| `secrets`    | Manage secrets                     |
| `expose`     | Create public tunnel               |
| `status`     | Show project information           |
| `projects`   | Manage registered projects         |
| `scripts`    | Manage project scripts             |
| `doctor`     | Check system requirements          |
| `completion` | Generate shell completion          |

---

## ⚙️ Project Structure

After initialization:

```
your-project/

├── .hive/
│
├── config.yml
├── .env
├── .secrets
└── logs/
```

Hive stores project configuration inside `.hive` and keeps your development environment organized.

---

## 🏗 Development

Clone repository:

```bash
git clone git@github.com:HiveSofts/hive-cli.git

cd hive-cli
```

Build:

```bash
cargo build
```

Run:

```bash
cargo run -- --help
```

Release build:

```bash
cargo build --release
```

---

## 🧪 Supported Platforms

| Platform | Status |
| -------- | ------ |
| Linux    | ✅      |
| Windows  | ✅      |
| macOS    | 🚧     |

---

## 🛣 Roadmap

* [x] Hive Init
* [x] Hive Run
* [x] Hive Env
* [x] Hive Secrets
* [x] Hive Logs
* [x] Hive Expose foundation
* [x] Automatic releases
* [ ] Cloud tunnel service
* [ ] Team collaboration
* [ ] Plugin system
* [ ] Package manager

---

## 🤝 Contributing

Contributions are welcome.

Before submitting a pull request:

```bash
cargo fmt
cargo test
cargo clippy
```

---

## 📄 License

MIT License

---

## 🐝 HiveSofts

Built with ❤️ and Rust by HiveSofts.

```
github.com/HiveSofts/hive-cli
```

```
