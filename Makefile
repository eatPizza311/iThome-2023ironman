.PHONY: version run test format lint release all

# Define ANSI escape codes for colors and formatting
RESET_COLOR = \033[0m
BOLD = \033[1m
RED = \033[91m
GREEN = \033[92m
YELLOW = \033[93m
BLUE = \033[94m

version:
	@echo "$(BOLD)Rust CLI Versions:$(RESET_COLOR)"
	@echo "------------------"
	@echo "$(BLUE)rustc (compiler):$(RESET_COLOR)            $$(rustc --version | sed 's/rustc //' | tr -d '\n')"
	@echo "$(BLUE)cargo (package manager):$(RESET_COLOR)     $$(cargo --version | sed 's/cargo //' | tr -d '\n')"
	@echo "$(BLUE)rustfmt (code formatter):$(RESET_COLOR)    $$(rustfmt --version | sed 's/rustfmt //' | tr -d '\n')"
	@echo "$(BLUE)clippy (linter):$(RESET_COLOR)             $$(clippy-driver --version | sed 's/clippy //' | tr -d '\n')"
	@echo "$(BLUE)rustup (toolchain manager):$(RESET_COLOR)  $$(rustup --version 2>/dev/null | sed 's/rustup //' | tr -d '\n')"

format:
	@echo "$(BLUE)$(BOLD)Formatting code...$(RESET_COLOR)"
	@cargo fmt --quiet

lint:
	@echo "$(YELLOW)$(BOLD)  Running linter...$(RESET_COLOR)"
	@cargo clippy --quiet

test:
	@echo "$(GREEN)$(BOLD)    Running Rust tests...$(RESET_COLOR)"
	@cargo test --quiet 2>&1 | \
		grep -E '^test result' | \
		sed 's/^/    /' | \
		sed -E 's/(ok)/\x1b[32m\1\x1b[0m/g; s/(FAILED)/\x1b[31m\1\x1b[0m/g'

run:
	@echo "$(RED)$(BOLD)Running the Rust program...$(RESET_COLOR)"
	@cargo run --quiet

release:
	@echo "$(BOLD)Building in release mode...$(RESET_COLOR)"
	@cargo build --release

all: format lint test run
	@echo "$(RESET_COLOR)$(BOLD)All tasks completed.$(RESET_COLOR)"
