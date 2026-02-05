# Mindbell

**Mindbell** — a simple mindful work session timer inspired by Pomodoro and Vipassana meditation. Each session starts and ends with a bell and provides a quiet period for focused work and short reflection.


## 📥 Download
Binaries are available from the **GitHub Releases** page: https://github.com/<your-org-or-user>/mindbell/releases

Please download the platformed build for your OS (e.g., `mindbell-0.1.0-1_x86_64.exe`).


## ✅ Verification
We publish checksums and signatures with every release. Each release includes a `checksums.txt` file containing SHA256 hashes for the release artifacts and an optional GPG signature (e.g. `checksums.txt.asc`).

- Create checksums (example):
  ```sh
  sha256sum mindbell-0.1.0-1_x86_64.exe mindbell-0.1.0-1-amd64.deb > checksums.txt
  ```

- Verify downloads using the included `checksums.txt` file (run in the directory with the downloaded assets):
  ```sh
  sha256sum -c checksums.txt
  ```

- Example `checksums.txt` contents:
  ```text
  <sha256-hash>  mindbell-0.1.0-1_x86_64.exe
  <sha256-hash>  mindbell-0.1.0-1-amd64.deb
  ```

- GPG verify (optional):
  ```sh
  gpg --verify checksums.txt.asc checksums.txt
  ```


## 🧩 Installation
- Windows: download the `.exe` or `.zip`, extract and run `mindbell-0.1.0-1_x86_64.exe`.
- Debian/Ubuntu (.deb): download the Debian package from the Releases page (e.g. `mindbell-0.1.0-1-amd64.deb`) and install with either:

  ```sh
  # newer apt supports installing local debs directly
  sudo apt install ./mindbell-0.1.0-1-amd64.deb

  # or with dpkg + fix deps
  sudo dpkg -i mindbell-0.1.0-1-amd64.deb
  sudo apt-get install -f
  ```

  Verify the downloaded file with SHA256 if desired:

  ```sh
  sha256sum mindbell-0.1.0-1-amd64.deb
  ```

- Linux/macOS (build from source): see **Build from source** below.


## ▶️ Usage
Run the program from a terminal:

```sh
./mindbell
```

Typical interaction:
- Enter the session intent (press Enter to exit)
- Enter duration in minutes (press Enter for 25)


## 🔧 Build from source
Prerequisites: Rust toolchain (rustup + cargo).

Cross-compile to Windows (x86_64) using MinGW (example on Debian/Ubuntu):

```sh
# add the Windows target
rustup target add x86_64-pc-windows-gnu

# install mingw-w64
sudo apt update && sudo apt install mingw-w64

# one-off env 
export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER=x86_64-w64-mingw32-gcc

[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"


# build release
cargo build --target x86_64-pc-windows-gnu --release
```

Output: `target/x86_64-pc-windows-gnu/release/mindbell-0.1.0-1_x86_64.exe` (zip it before attaching to Releases).


## ⚙️ CI / Releases
We recommend automating builds and releases with GitHub Actions. See `.github/workflows/release.yml` (example in the docs repository) that builds and uploads artifacts on tag push.


## 🐛 Troubleshooting
- If notifications fail on headless/SSH sessions you'll see errors like:
  `org.freedesktop.DBus.Error.ServiceUnknown`
  This indicates no desktop notification daemon / session D-Bus is available. Run locally, or use alternative notification strategies.


## 🤝 Contributing
Contributions welcome — please open issues or pull requests. Follow the contributor guidelines in `CONTRIBUTING.md` (if present).


## 📜 License
Mindbell is dual-licensed under **MIT OR Apache-2.0**. See `LICENSE-MIT` and `LICENSE-APACHE` for the full license texts.

If you prefer a single license, the project also accepts contributions under either MIT or Apache-2.0 terms.


---
*Mindbell — simple, quiet, mindful focus.*
