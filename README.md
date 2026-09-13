# gh-dev-loop-test

Polyglot playground to test the Muse -> GitHub dev loop.
Each language implements `greet(name)` plus a test.
Pushed to GitHub entirely by Merlin (Muse agent) via `gh`.

## The machine (where Merlin runs)

- OS: Linux x86_64, kernel 7.0.0-26-generic (Ubuntu 24.04)
- CPU: 2 vCPU — AMD EPYC 9D25 126-Core Processor
- RAM: 7.7 GB
- Disk: 7.5 GB
- Tools: git 2.43.0, gh 2.100.0, gcc/g++ 13.3.0, make 4.3, cmake 3.28.3, sqlite 3.45.1,
  rustc/cargo 1.98.1 (via rustup), go 1.27.1

## Language runtimes — what's installed and what runs

| Language      | Version            | Smoke test (`greet("merlin")`) | Notes                           |
|---------------|--------------------|--------------------------------|---------------------------------|
| Python        | 3.12.3             | ✅ `hello, merlin!`            | `python/greet.py` + test        |
| Node.js       | 24.20.0 (npm 10)   | ✅ `hello, merlin!`            | `node/greet.js` + `node --test` |
| Perl          | 5.38.2             | ✅ `hello, merlin!`            | `perl/greet.pl`                 |
| C (gcc)       | 13.3.0             | ✅ `hello, merlin!`            | `c/greet.c`                     |
| C++ (g++)     | 13.3.0             | ✅ `hello, merlin!`            | smoke-tested, no repo file      |
| SQLite        | 3.45.1             | ✅ `hello, merlin!`            | `:memory:` query                |
| Rust          | 1.98.1 (via rustup)    | ✅ `hello, merlin!`            | `rust/` — `cargo test` passes |
| Go            | 1.27.1                 | ✅ `hello, merlin!`            | `go/` — `go test ./...` passes |
| Ruby/PHP/Java | — not installed        | ➖                              |                                 |

All smoke tests run 2026-09-13 on the machine above. Rust (rustup) and Go
(tarball from go.dev) were installed on demand into `~/.cargo` and `~/sdk`
— no sudo needed, took a couple of minutes. The CI workflow is kept local
until the `workflow` OAuth scope is granted (see below).

## Known friction (Muse -> GitHub loop)

- `gh` ships unauthenticated; login needs the device flow (copy a code on
  mobile — painful).
- The device-flow token lacks the `workflow` scope, so pushing
  `.github/workflows/*.yml` is rejected; `gh auth refresh -s workflow`
  needs a second device-code round-trip.
