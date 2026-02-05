# Release Notes

This file contains suggested release notes templates and an example for a release.

---

## v1.0.0 — 2026-02-05
**Summary:** First stable build and the initial Windows x86_64 release.

### Highlights
- ✅ Initial release of **Mindbell** 
- 🔔 Notification support for desktop environments (Linux & Windows)
- 🛠️ Cross-compiled Windows x86_64 binary included
- 📝 Basic CLI usage and session flow implemented

### Assets
- `mindbell-0.1.0-1_x86_64.exe` — Windows x86_64 build
- `mindbell-0.1.0-1-amd64.deb` — Debian/Ubuntu package for amd64
- `mindbell-0.1.0-1-linux-x86_64.tar.gz` — Linux build (if provided)

### Checksums
We publish a `checksums.txt` file containing SHA256 hashes for all release assets and an optional `checksums.txt.asc` (GPG signature).

- Create checksums (example):
  ```sh
  sha256sum mindbell-0.1.0-1_x86_64.exe target/debian/mindbell_0.1.0_amd64.deb > checksums.txt
  gpg --armor --detach-sign --output checksums.txt.asc checksums.txt  # optional
  ```

- Example `checksums.txt` contents:
  ```text
  <sha256-hash>  mindbell-0.1.0-1_x86_64.exe
  <sha256-hash>  mindbell-0.1.0-1-amd64.deb
  ```

- Upload `checksums.txt` and `checksums.txt.asc` (if present) alongside assets when creating the Release.

### Release notes (short)
> Mindbell v1.0.0 is the project’s first stable release, providing a simple, distraction-free session timer with start/end bells and optional desktop notifications. Download platform binaries from GitHub Releases.

---

## Example changelog entries (for future releases)
- v1.1.0 — Added persistent notification option
- v1.2.0 — Added configuration file support and improved tests

---

### How to publish
1. Tag the release: `git tag -a v0.1.0-1 -m "Release v0.1.0-1"`
2. Push tag: `git push origin v0.1.0-1`
3. Build artifacts and create Release in GitHub (or use `gh`):
   - Build Windows: `cargo build --release --target x86_64-pc-windows-gnu`
   - Build .deb (Debian/Ubuntu package): `cargo build --release && cargo deb --no-build` (output: `target/debian/mindbell_0.1.0_amd64.deb`)
   - Create the Release and upload assets, e.g.:
     `gh release create v0.1.0-1 --title "v0.1.0-1" --notes-file RELEASE_NOTES.md --assets "target/x86_64-pc-windows-gnu/release/mindbell-0.1.0-1_x86_64.exe" "target/debian/mindbell_0.1.0_amd64.deb"`
4. Upload the checksum file and signature alongside assets.

---

*Template prepared for maintainers. Replace placeholders with real dates, checksums and asset names before publishing.*
