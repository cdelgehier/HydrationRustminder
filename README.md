# 💧 HydrationRustminder

A lightweight macOS tray application written in Rust that reminds you to stay hydrated throughout your workday.

[![codecov](https://codecov.io/gh/cdelgehier/HydrationRustminder/branch/main/graph/badge.svg)](https://codecov.io/gh/cdelgehier/HydrationRustminder)
![Rust](https://img.shields.io/badge/rust-1.70+-orange?logo=rust)
![macOS](https://img.shields.io/badge/macOS-13.0+-blue?logo=apple)

## Features

- 🎯 **System tray icon** with menu for easy access
- ⚙️ **Configurable settings** via menu (start/end hours, intervals)
- 💾 **Persistent configuration** saved to `~/Library/Application Support`
- 🔔 **Native macOS notifications**
- 🎨 **Dynamic menu** with real-time updates
- 🪶 **Lightweight** - minimal resource usage

## Installation

### Prerequisites

- macOS 13.0 (Ventura) or later
- Rust 1.70+ (see below for installation)

### Install Rust (if not already installed)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Build and Run

```bash
# Clone the repository
git clone https://github.com/cdelgehier/HydrationRustminder.git
cd HydrationRustminder

# Build and run in debug mode
RUST_LOG=info cargo run

# Or build release version
cargo build --release
./target/release/hydration-rustminder
```

## Development

### Run Tests

```bash
# Run all tests
cargo test

# Run tests with coverage report
cargo install cargo-llvm-cov
rustup component add llvm-tools-preview
cargo llvm-cov --html
cargo llvm-cov --open  # Opens coverage report in browser
```

### Code Quality

```bash
# Format code
cargo fmt

# Run linter
cargo clippy -- -D warnings

# Run all checks (format + clippy + tests)
cargo fmt && cargo clippy -- -D warnings && cargo test
```

### Pre-commit Hooks

```bash
# Install pre-commit
pip install pre-commit

# Install hooks
pre-commit install

# Run manually
pre-commit run --all-files
```

## Configuration

Settings are stored in `~/Library/Application Support/hydration-rustminder/config.yaml`

Default configuration:
```yaml
start_hour: 9        # Start reminders at 9 AM
end_hour: 18         # Stop reminders at 6 PM
interval_minutes: 30 # Reminder every 30 minutes
reminder_minutes: 5  # Follow-up reminder after 5 minutes
```

All settings can be changed via the tray icon menu and are saved automatically.

## Project Structure

```
src/
├── main.rs         # Entry point and event loop
├── config.rs       # Configuration management
├── notifier.rs     # Notification system
└── ui/
    ├── mod.rs      # UI module exports
    ├── menu.rs     # Menu construction and updates
    └── tray.rs     # Tray icon management
```

## CI/CD

- **Commitizen** - Conventional commits validation
- **Auto-versioning** - Semantic versioning on main branch
- **GitHub Releases** - Automatic release creation
- **Code coverage** - Codecov integration

## License

MIT License - see [LICENSE](LICENSE)

## Contributing

Contributions welcome! Please follow:
- Conventional Commits for commit messages
- Run `cargo fmt` and `cargo clippy` before committing
- Add tests for new features
- Target 70%+ code coverage

---

**💧 Stay hydrated, stay productive!**
