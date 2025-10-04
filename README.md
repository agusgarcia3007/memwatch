# memwatch

<p align="center">
  <img src="memwatch.icns" alt="memwatch icon" width="128" height="128">
</p>

A ultra-lightweight macOS process monitor built in Rust. View top processes, force-quit them, and monitor real-time system resources — all with minimal overhead.

**⚡️ Fast** • **🪶 Lightweight** • **🎯 Native** • **🔒 Secure**

## Features

- **Process List**: See top processes sorted by memory or CPU usage
- **Force Quit**: Graceful termination (SIGTERM) with fallback to SIGKILL
- **Real-Time Charts**: CPU and memory usage over last 60-300 seconds
- **CLI Toggle**: Use `memwatch toggle` to show/hide from terminal
- **Ultra-Light**: <50-80 MB RAM, <2% CPU when idle, sub-300ms startup
- **Native**: Pure Rust with egui, no Electron or heavy frameworks

## Why memwatch?

Unlike Activity Monitor or heavyweight alternatives:
- **Smaller**: 3 MB binary vs 50+ MB for Electron apps
- **Faster**: <300ms startup vs multi-second launches
- **Lighter**: 40-70 MB RAM vs 200+ MB for browser-based tools
- **Simpler**: Focused interface showing exactly what you need
- **CLI Integration**: Toggle window from terminal or scripts

## Requirements

- macOS 13+ (Ventura or later)
- Compatible with Apple Silicon and Intel Macs
- Rust toolchain (for building from source)

## Installation

### Quick Install

```bash
# Build and package
make package

# Install to /Applications and create CLI link
make install
```

### Manual Install

1. **Create the app icon** (optional - already included):
   ```bash
   ./create_icon.sh
   ```

2. **Build the app**:
   ```bash
   ./build.sh
   ```

3. **Create the package**:
   ```bash
   ./package.sh
   ```

4. **Install**:
   - Drag `dist/memwatch.app` to `/Applications`
   - Create CLI symlink:
     ```bash
     sudo ln -sf /Applications/memwatch.app/Contents/MacOS/memwatch /usr/local/bin/memwatch
     ```

### Build Options

The build script automatically creates a universal binary (Apple Silicon + Intel) if both targets are installed:

```bash
rustup target add aarch64-apple-darwin
rustup target add x86_64-apple-darwin
```

## Usage

### Launch GUI

```bash
# From Applications folder or Spotlight
open -a memwatch

# Or from CLI
memwatch
```

### CLI Commands

```bash
# Show/hide window
memwatch toggle

# Help
memwatch --help
```

### Global Hotkey

Global hotkey is currently disabled in this build due to macOS API thread safety requirements. Use `memwatch toggle` from terminal instead.

## Interface

### Toolbar
- **Sort by**: Toggle between Memory and CPU sorting
- **Filter**: Search processes by name or PID
- **Settings**: Configure refresh rate, chart window, and hotkey

### Process List
- Displays top 100 processes
- Columns: Process Name, PID, CPU %, Memory (MB)
- **Force Quit** button for each process:
  1. First attempt: Sends SIGTERM (graceful shutdown)
  2. If process doesn't exit: Prompts for SIGKILL confirmation

### Resource Chart
- Bottom third of window
- Shows CPU % and Memory (GB) over time
- Configurable window: 60-300 seconds (default: 120s)
- Auto-scales and updates every 1s

## Settings

Access via **⚙ Settings** button:

- **Chart window**: 60-300 seconds
- **Refresh interval**: 0.5-2 seconds (default: 1s)

Settings are automatically saved to:
```
~/Library/Application Support/memwatch/settings.json
```

## App Icon

The memwatch icon features:
- **Gradient Design**: Blue gradient background (#4A90E2 → #357ABD)
- **Letter "M"**: Large white "M" for memwatch
- **Native Style**: Rounded corners following macOS design guidelines
- **Retina Support**: All sizes from 16x16 to 1024x1024 (@1x and @2x)

### Regenerating the Icon

If you want to customize the icon:

```bash
# Requires ImageMagick
brew install imagemagick

# Generate new icon
./create_icon.sh

# Rebuild package with new icon
./package.sh
```

To use a custom icon, place a 1024x1024 PNG at `/tmp/memwatch_base.png` before running `./create_icon.sh`.

## Performance

Measured on Apple Silicon MacBook:

- **Binary Size**: 3.0 MB (release build with LTO and stripping)
- **Icon Size**: 29 KB (.icns with all sizes)
- **Memory Usage**: 40-70 MB idle
- **CPU Usage**: <1-2% idle, ~3-5% during 1s refresh
- **Startup Time**: <200-300ms cold start

## Permissions

memwatch only requires standard macOS permissions:

- **Process listing**: Uses standard sysinfo APIs (no special access needed)
- **Force quit**: Requires permission to send signals to user-owned processes
- **Global hotkey**: Registers system-wide keyboard event monitor

No Full Disk Access or Accessibility permissions required.

> **Note**: System processes owned by root may show "Operation not permitted" when force-quitting. This is expected macOS security behavior.

## Troubleshooting

### CLI Toggle Fails

```bash
# Check if app is running
pgrep -x memwatch

# If not running, launch it first
memwatch
```

### Permission Errors When Force-Quitting

- You can only terminate processes owned by your user
- System processes (owned by root) require admin privileges
- Protected processes (like kernel tasks) cannot be killed

### Build Errors

Ensure you have Rust installed:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

For universal binary support:
```bash
rustup target add aarch64-apple-darwin
rustup target add x86_64-apple-darwin
```

## Uninstall

```bash
# Remove app
sudo rm -rf /Applications/memwatch.app

# Remove CLI link
sudo rm /usr/local/bin/memwatch

# Remove settings (optional)
rm -rf ~/Library/Application\ Support/memwatch
```

## Development

### Project Structure

```
memory-monitor/
├── src/
│   ├── main.rs       # Entry point and CLI routing
│   ├── ui.rs         # Main UI and event loop
│   ├── metrics.rs    # Process and system metrics collection
│   ├── killer.rs     # Process termination (SIGTERM/SIGKILL)
│   ├── hotkey.rs     # Global hotkey (placeholder)
│   ├── ipc.rs        # Unix socket for CLI toggle
│   └── settings.rs   # Settings persistence
├── build.sh          # Build script (with universal binary support)
├── create_icon.sh    # Generate app icon with gradient and "M"
├── package.sh        # Create .app bundle and DMG
├── install.sh        # Install to /Applications
├── Makefile          # Build targets
├── memwatch.icns     # App icon (29 KB, all sizes)
└── Cargo.toml        # Rust dependencies and build config
```

### Build Commands

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Create icon (requires ImageMagick)
./create_icon.sh

# Universal binary
make build

# Create DMG
make package

# Full build + package
make all
```

### Testing

```bash
# Run tests
cargo test

# Check for issues
cargo clippy

# Format code
cargo fmt

# Verify icon is in bundle
ls -lh dist/memwatch.app/Contents/Resources/memwatch.icns
```

## Architecture Decisions

- **egui**: Immediate-mode GUI for minimal overhead and fast rendering
- **sysinfo**: Cross-platform system metrics with excellent macOS support
- **Unix sockets**: Lightweight IPC for CLI toggle command
- **CLI toggle**: Unix socket IPC for instant window show/hide
- **No background daemon**: Single process model, IPC only when GUI running

## Known Limitations

- **Process list**: Limited to top 100 processes (performance)
- **Chart history**: Capped at 300 seconds (memory)
- **Global hotkey**: Disabled (requires migration to objc2 crate for thread safety)
- **Process icons**: No app icons in process list (would increase memory/CPU overhead)
- **Menu bar**: No menu bar icon (minimal footprint design choice)

## Technical Details

### Dependencies

Core dependencies (see `Cargo.toml` for full list):
- **eframe/egui**: Immediate-mode GUI framework (minimal, fast)
- **egui_plot**: Chart rendering
- **egui_extras**: Table widget for process list
- **sysinfo**: Cross-platform system information
- **libc**: POSIX signals (SIGTERM/SIGKILL)
- **serde/serde_json**: Settings serialization
- **directories**: Standard app directories

### Build Optimizations

The `Cargo.toml` includes aggressive size optimizations:
```toml
[profile.release]
opt-level = "z"        # Optimize for size
lto = true             # Link-time optimization
codegen-units = 1      # Better optimization
strip = true           # Remove debug symbols
panic = "abort"        # Smaller panic handling
```

Result: 3.0 MB binary (down from ~10+ MB unoptimized)

## Roadmap

Potential future enhancements (contributions welcome):

- [ ] Global hotkey support (migrate to objc2 crate)
- [ ] Menu bar icon with dropdown
- [ ] Process tree view
- [ ] Network I/O monitoring
- [ ] Disk I/O stats per process
- [ ] Export metrics to CSV/JSON
- [ ] Custom alert thresholds
- [ ] Process favorites/pinning
- [ ] App icon display in process list

## Contributing

This is a personal project, but contributions are welcome!

**How to contribute:**
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run `cargo fmt` and `cargo clippy`
5. Submit a pull request

**Bug reports and feature requests** are welcome via GitHub Issues.

## License

This project is provided as-is for personal use.

MIT License - see LICENSE file for details.

## Acknowledgments

Built with:
- **[Rust](https://www.rust-lang.org/)** - Systems programming language
- **[egui](https://github.com/emilk/egui)** - Immediate-mode GUI framework
- **[sysinfo](https://github.com/GuillaumeGomez/sysinfo)** - System information library

---

**Built with** ❤️ **using Rust and egui**

*memwatch - Because your Mac deserves a lightweight process monitor*
