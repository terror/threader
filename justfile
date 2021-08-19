set dotenv-load

default:
	just --list

ci: build test clippy fmt-check

build:
	cargo build

test:
	cargo test

clippy:
  cargo clippy --all-targets --all-features

fmt-check:
  cargo +nightly fmt --all -- --check
  @echo formatting check done

run *args:
	cargo run -- --{{args}}

fmt:
	cargo +nightly fmt

check:
 cargo check

watch +COMMAND='test':
	cargo watch --clear --exec "{{COMMAND}}"

forbid:
	./bin/forbid

usage:
	cargo run -- --help | pbcopy

sloc:
  @cat src/*.rs | sed '/^\s*$/d' | wc -l
