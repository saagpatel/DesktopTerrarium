# Desktop Terrarium — Nested-Repo Layout + First-Boot Validation

Walks an operator from a fresh checkout to a running terrarium window,
and documents the unusual on-disk layout (the git repo lives one
directory below where you'd expect).

> **Audience:** anyone resuming work or demoing the terrarium for the
> first time.

---

## 0. The nested layout

The portfolio operating system has a packet flagged on this repo
specifically for the nested-repo posture. Here is what is going on:

```
/Users/d/Projects/Fun:GamePrjs/DesktopTerrarium/
└── desktop_terrarium/         ← THIS is the git repo (origin: saagpatel/DesktopTerrarium)
    ├── Cargo.toml              ← workspace root
    ├── src/
    ├── scripts/
    ├── assets/
    └── third_party/
```

- The parent directory `DesktopTerrarium/` is a portfolio organisation
  folder, **not** a git repo. There is no `.git` at that level.
- The nested `desktop_terrarium/` is the canonical checkout. All
  `cargo`, `git`, and CI commands must be run from inside that
  directory.
- Both remotes are configured on the inner repo:
  - `origin` → `saagpatel/DesktopTerrarium` (current canonical remote)
  - `legacy-origin` → `saagar210/DesktopTerrarium` (older account; kept
    for historical branches — do not push)

**If you `cd /Users/d/Projects/Fun:GamePrjs/DesktopTerrarium/` and run
`git status`, you will get `fatal: not a git repository`.** That is
the symptom of the nested layout, not a corruption. Step into
`desktop_terrarium/` first.

The parent folder shape is preserved deliberately so the portfolio
tree groups related game projects together. Do not flatten it without
also updating any portfolio-OS path references.

---

## 1. Prerequisites

- **OS:** macOS (developed on; Linux/Wayland likely works via
  `bevy[wayland]` feature, untested)
- **Rust:** 1.70+, stable channel. Get it via `rustup`.
- **Cargo:** ships with rustup.

```bash
rustc --version
cargo --version
```

Optional but recommended for the canonical verify list:

```bash
cargo install cargo-nextest cargo-deny cargo-audit
```

---

## 2. Clone (if doing this from scratch)

```bash
mkdir -p /Users/d/Projects/Fun:GamePrjs/DesktopTerrarium
cd       /Users/d/Projects/Fun:GamePrjs/DesktopTerrarium
git clone https://github.com/saagpatel/DesktopTerrarium.git desktop_terrarium
cd desktop_terrarium
```

The inner directory name (`desktop_terrarium`) is what makes the
nested layout work. Pick a different name and other operator tooling
will fail to find the repo.

---

## 3. First boot — fastest possible path

From inside `desktop_terrarium/`:

```bash
cargo run --release
```

**Expected:**
- First run compiles ~700 crates (Bevy + transitive deps). Plan on
  3–8 minutes on an M-series Mac.
- A window opens titled "Desktop Terrarium" at 800×600 with the
  terrarium scene rendered.
- The window is resizable (min 400×300).
- Process keeps the foreground; no panics in the terminal.

**If it fails:**

| Symptom | Likely cause | Fix |
|---|---|---|
| `error[E0432]: unresolved import 'bevy::...'` | Old cargo cache | `cargo clean && cargo build` |
| Window opens then immediately closes | GPU driver mismatch (macOS) | Update macOS; try `cargo run` (debug) to see panic |
| `error: linking with cc failed` | Xcode CLI tools missing | `xcode-select --install` |
| `cannot find -lframework AppKit` | Bevy/macOS feature flag drift | Re-check `bevy = { version = "0.15", features = ["wayland"] }` in `Cargo.toml`; the `wayland` feature is fine on macOS — Bevy auto-selects the native backend. |

---

## 4. Canonical verification path

The `README.md` (when present on a feature branch) describes a
canonical verify list. The reproducible boot-and-test sequence is:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --no-deps -- -D warnings
cargo nextest run --workspace --all-targets --profile release --locked
cargo test --workspace --doc --locked
cargo build --workspace --all-targets --locked
```

Heavier (run only when about to release):

```bash
cargo deny check --config deny.toml advisories bans licenses sources
cargo audit --deny warnings
./scripts/perf/native-smoke.sh
```

The `master` branch is intentionally minimal — most of the CI
hardening lives on `codex/release-audit-hardening`. Use that branch
if you want the full canonical workflow.

---

## 5. Project structure reference

```
desktop_terrarium/
├── Cargo.toml                 workspace root, MIT, bevy 0.15
├── Cargo.lock                 committed
├── src/
│   ├── main.rs                window setup + plugin registration
│   ├── lib.rs                 plugin entry
│   ├── plugins/               TerrariumPlugin and friends
│   ├── components/            Bevy ECS components
│   ├── events/                Bevy ECS events
│   ├── resources/             Bevy ECS resources
│   ├── systems/               Bevy ECS systems
│   └── errors.rs              centralized error type
├── assets/                    runtime art assets
├── scripts/
│   ├── check_local_artifacts.sh
│   └── clean.sh
└── third_party/               vendored licenses + sources
```

The default Bevy features enabled in `Cargo.toml`:

- `stable-core` — core terrarium gameplay (always on)
- `experimental-weather` — weather plugin (on by default; toggle with
  `--no-default-features --features stable-core`)

---

## 6. What "works" looks like on first boot

| Visual / behavioral marker | Expected |
|---|---|
| Window title | "Desktop Terrarium" |
| Resolution | 800×600 (resizable, min 400×300) |
| Present mode | AutoVsync — no tearing, frame-paced |
| Terrarium scene | Visible content (per `codex/release-audit-hardening`'s "art-driven terrarium atmosphere" pass) |
| Terminal output | Bevy startup logs, no `ERROR` lines, no panic |
| Exit on Cmd-W / window close | Clean exit (exit code 0) |

If any of the above is missing, **do not** assume the build is fine —
treat it as a boot regression and bisect against the last known-good
commit on `master`.

---

## 7. Reactivation / handoff checklist

When picking this up after time away:

1. `cd` into `desktop_terrarium/` (not the parent — see Section 0).
2. `git status` — confirm clean working tree on `master` or your
   feature branch.
3. `git fetch --all --prune` — see if `legacy-origin` has anything new
   (it shouldn't; that account is frozen).
4. `cargo run --release` — first boot validation.
5. If you plan to publish a release, switch to
   `codex/release-audit-hardening` for the hardened CI workflow and
   verify list.
6. Branches named `codex/*` are Codex-OS scaffolding; treat as
   in-progress unless their PRs are merged.

---

## 8. Known good reference

| Field | Value |
|---|---|
| Last commit on `master` | `b2fc2ca` chore(ci): guard local artifacts and trim placeholders |
| Bevy version | 0.15 |
| Edition | 2021 |
| MSRV (tested) | Rust 1.70+ |
| First-boot command | `cargo run --release` from `desktop_terrarium/` |
| Hardened verify branch | `codex/release-audit-hardening` |
