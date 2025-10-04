.PHONY: build package install uninstall clean help

help:
	@echo "memwatch - Build and Install Targets"
	@echo ""
	@echo "Usage:"
	@echo "  make build      Build the binary (universal if possible)"
	@echo "  make package    Create .app bundle and DMG"
	@echo "  make install    Install to /Applications and create CLI link"
	@echo "  make uninstall  Remove app and CLI link"
	@echo "  make clean      Remove build artifacts"
	@echo "  make all        Build, package, and show install instructions"

build:
	@./build.sh

package: build
	@./package.sh

install: package
	@./install.sh

uninstall:
	@echo "Uninstalling memwatch..."
	@sudo rm -rf /Applications/memwatch.app
	@sudo rm -f /usr/local/bin/memwatch
	@echo "✓ Uninstalled successfully"

clean:
	cargo clean
	rm -rf dist

all: package
	@echo ""
	@echo "Build complete! To install, run: make install"
