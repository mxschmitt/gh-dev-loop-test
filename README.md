# gh-dev-loop-test

An experiment: can a Muse agent run a real GitHub development workflow
end-to-end? This repo is the lab notebook. Everything here — scaffold,
commits, pushes, toolchain installs — was done by Merlin (Muse agent) on
2026-09-13, driven from a phone.

## The machine

- Linux x86_64, kernel 7.0.0-26-generic (Ubuntu 24.04)
- 2 vCPU (AMD EPYC 9D25), 7.7 GB RAM, 7.5 GB disk
- Preinstalled: git 2.43.0, gh 2.100.0 (unauthenticated), gcc/g++ 13.3.0,
  make, cmake, sqlite3, python3 3.12, node 24

Small but sufficient: full test suites for six languages run in seconds.

## How to develop with it

```sh
git clone https://github.com/mxschmitt/gh-dev-loop-test.git
cd gh-dev-loop-test
git checkout -b <branch>
# edit, then:
python3 -m pytest python/        # or: node --test node/
cargo test --manifest-path rust/Cargo.toml
cd go && go test ./...
git add -A && git commit -m "..." && git push -u origin <branch>
gh pr create --fill              # needs gh auth (see below)
```

## How to install languages (how I did it)

No sudo, no package manager — user-space installs, minutes each:

```sh
# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o /tmp/rustup.sh
sh /tmp/rustup.sh -y --profile minimal
export PATH="$HOME/.cargo/bin:$PATH"

# Go
V=$(curl -s https://go.dev/VERSION?m=text)
curl -sL https://go.dev/dl/${V}.linux-amd64.tar.gz -o /tmp/go.tar.gz
mkdir -p ~/sdk && tar -C ~/sdk -xzf /tmp/go.tar.gz
export PATH="$HOME/sdk/go/bin:$PATH"
```

Result: rustc/cargo 1.98.1, go 1.27.1, both fully working.

## Experiments

| # | Experiment | Result |
|---|------------|--------|
| 1 | Clone / branch / edit / commit locally | ✅ trivial |
| 2 | `gh auth login` (device flow) | ✅ worked, but code copy on mobile is painful |
| 3 | `gh repo create` + push | ✅ repo live |
| 4 | Push `.github/workflows/ci.yml` | ❌ rejected — OAuth token lacks `workflow` scope |
| 5 | `gh auth refresh -s workflow` | ❌ needed a 2nd device-code round-trip; grant didn't stick |
| 6 | Language inventory (python/node/perl/c/c++/sqlite) | ✅ all run, see table below |
| 7 | Install Rust + Go on demand | ✅ minutes, no sudo, tests pass |
| 8 | Keep CI workflow local-only | ⚠️ workaround for #4 — it's untracked, not forgotten |
| 9 | Install k3s, run hello-world pod | ❌ blocked — no cgroup delegation in this container (see learnings) |

Language smoke tests (`greet("merlin")` → `hello, merlin!`):

| Language | Version | Status |
|----------|---------|--------|
| Python | 3.12.3 | ✅ |
| Node.js | 24.20.0 | ✅ |
| Perl | 5.38.2 | ✅ |
| C / C++ | gcc 13.3.0 | ✅ |
| Rust | 1.98.1 (installed in #7) | ✅ `cargo test` passes |
| Go | 1.27.1 (installed in #7) | ✅ `go test` passes |
| SQLite | 3.45.1 | ✅ |
| Ruby / PHP / Java | — | ➖ not installed, not tried |

## Learnings for the future

1. **The machine is the easy part.** Toolchains install in minutes without
   sudo; the dev loop (edit → test → commit → push) works fine.
2. **Auth is the hard part.** `gh` ships logged out, the device flow is
   phone-hostile (copy a code, switch apps, paste), and the default token
   scopes silently exclude `workflow` — so CI files can't be pushed.
3. **Scope upgrades are a trap.** `gh auth refresh -s workflow` demands
   another full device-code round-trip, and in our run the grant didn't
   stick. Budget for this or use a PAT with the right scopes up front.
4. **Keep `.github/` out of `git add -A`** until the scope is sorted, or
   every push gets rejected. (We learned this twice.)
5. **What's missing for a nice Muse↔GitHub workflow:** a one-click GitHub
   connector (like Gmail/Notion/X have), pre-authed `gh`, and a way to
   watch PR/CI status without polling.
6. **Know your sandbox.** This machine is a systemd-nspawn container with
   zero cgroup controllers delegated (`/sys/fs/cgroup/cgroup.controllers`
   is empty). k3s installs fine but can't start — `failed to find cpu
   cgroup (v2)` — and no kubelet can run here. Anything needing cgroups
   (k8s, docker) is out; plain binaries and language toolchains are in.
