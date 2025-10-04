# memwatch

A ultra-lightweight macOS process monitor built in Rust. View top processes, force-quit them, and monitor real-time system resources — all with minimal overhead.

## Features

- **Process List**: See top processes sorted by memory or CPU usage
- **Force Quit**: Graceful termination (SIGTERM) with fallback to SIGKILL
- **Real-Time Charts**: CPU and memory usage over last 60-300 seconds
- **CLI Toggle**: Use `memwatch toggle` to show/hide from terminal
- **Ultra-Light**: <50-80 MB RAM, <2% CPU when idle, sub-300ms startup
- **Native**: Pure Rust with egui, no Electron or heavy frameworks

## Requirements

- macOS 13+ (Ventura or later)
- Compatible with Apple Silicon and Intel Macs

## Installation

### Quick Install

```bash
# Build and package
make package

# Install to /Applications and create CLI link
make install
```

### Manual Install

1. **Build the app**:
   ```bash
   ./build.sh
   ```

2. **Create the package**:
   ```bash
   ./package.sh
   ```

3. **Install**:
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

## Performance

Measured on Apple Silicon MacBook:

- **Binary Size**: ~3-5 MB (release build with LTO and stripping)
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
│   ├── hotkey.rs     # Global hotkey registration
│   ├── ipc.rs        # Unix socket for CLI toggle
│   └── settings.rs   # Settings persistence
├── build.sh          # Build script (with universal binary support)
├── package.sh        # Create .app bundle and DMG
├── install.sh        # Install to /Applications
└── Makefile          # Build targets
```

### Build Commands

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

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
```

## Architecture Decisions

- **egui**: Immediate-mode GUI for minimal overhead and fast rendering
- **sysinfo**: Cross-platform system metrics with excellent macOS support
- **Unix sockets**: Lightweight IPC for CLI toggle command
- **CLI toggle**: Unix socket IPC for instant window show/hide
- **No background daemon**: Single process model, IPC only when GUI running

## Known Limitations

- Process list limited to top 100 processes (performance)
- Chart history capped at 300 seconds (memory)
- Global hotkey disabled (requires migration to objc2 crate for thread safety)
- No app icons in process list (would increase memory/CPU overhead)
- No menu bar icon (minimal footprint design choice)

## License

This project is provided as-is for personal use.

## Contributing

This is a personal project, but feedback and bug reports are welcome via GitHub Issues.

---

**Built with** ❤️ **using Rust and egui**
