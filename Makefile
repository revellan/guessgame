# PRODUCTION MAKEFILE

guessgame:
	@cargo build --release

install: guessgame
	@if [ "$(shell whoami)" != "root" ]; then \
		echo "You must be root to install this package!"; \
		exit 1; \
	fi
	@install -Dm755 target/release/guessgame /usr/local/bin/guessgame
	@echo "Installed to /usr/local/bin/guessgame"

clean:
	@cargo clean

