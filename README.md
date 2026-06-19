> We're dsplce.co, check out our work on our website: [dsplce.co](https://dsplce.co) 🖤

# statsig-wasm

[![WebAssembly](https://img.shields.io/badge/WebAssembly-654FF0?style=for-the-badge&logo=webassembly&logoColor=white)](https://webassembly.org/)
[![crates.io](https://img.shields.io/crates/v/statsig-wasm?style=for-the-badge&color=%230F80C1)](https://crates.io/crates/statsig-wasm)
[![crates.io Downloads](https://img.shields.io/crates/d/statsig-wasm?style=for-the-badge&color=%23FF0346)](https://crates.io/crates/statsig-wasm)
[![crates.io Size](https://img.shields.io/crates/size/statsig-wasm?style=for-the-badge)](https://crates.io/crates/statsig-wasm)
[![License](https://img.shields.io/crates/l/statsig-wasm.svg?style=for-the-badge)](https://crates.io/crates/statsig-wasm)

📈 Wasm bindings for the Statsig JavaScript (Web) SDK.

`statsig-wasm` lets you drive Statsig — initialise the client, run session replay, run autocapture — straight from your Rust/Wasm frontend, instead of dropping back into a patch of hand-written JS every time. You define your user in Rust, it gets serialised across the Wasm boundary for you, and the rest is just method calls on a typed `StatsigClient`.

_Disclaimer: this project has no affiliation with the official Statsig project or trademark._

---

## Table of Contents

- [📦 Installation](#-installation)
- [🔧 Setup](#-setup)
- [🧪 Usage](#-usage)
  - [Initialise the client](#initialise-the-client)
  - [Initialise synchronously](#initialise-synchronously)
  - [Session replay & autocapture](#session-replay--autocapture)
- [🛠️ Requirements](#%EF%B8%8F-requirements)
- [📁 Repo & Contributions](#-repo--contributions)
- [📄 License](#-license)

⸻

## 📦 Installation

It's a library crate, so add it to your project with:

```sh
cargo add statsig-wasm
```

⸻

## 🔧 Setup

These bindings call into the Statsig JS client that lives on the global `Statsig` namespace — so that script has to be on the page before your Wasm module runs. Add the following to your HTML `<head>`:

```html
<script
  src="https://cdn.jsdelivr.net/npm/@statsig/js-client@3/build/statsig-js-client+session-replay+web-analytics.min.js"
  crossorigin="anonymous"
>
</script>
```

That particular bundle ships session replay and web analytics (autocapture) baked in, which is what the `run_statsig_session_replay` and `run_statsig_auto_capture` bindings below reach for — so keep those in the URL if you intend to use them.

Bear in mind, if you're happy for Statsig to autoinitialise the client, just append `?apikey=<YOUR_CLIENT_API_KEY>` to the script's `src` above and uninstall this crate 😉

If you're looking to initialise the client yourself, read on.

⸻

## 🧪 Usage

### Initialise the client

Build a `StatsigClient` from your client API key and a user, then kick off initialisation:

```rust
use statsig_wasm::{run_statsig_auto_capture, StatsigClient, StatsigUser};

let statsig = StatsigClient::new(
    env!("STATSIG_API_KEY"),
    StatsigUser {
        user_id: user_id.get(),
    },
)
.unwrap();

run_statsig_auto_capture(&statsig);

// `spawn_local` is native to Leptos, use your
// framework's equivalent to run the async method.
spawn_local(async move {
    statsig.initialize().await;
});
```

`StatsigUser` is serialised to the shape Statsig expects (your `user_id` lands as `userID`), so you stay in Rust and don't hand-roll the JS object.

### Initialise synchronously

If you're not in an async context — or just don't want to await — there's a synchronous initialiser that returns straight away:

```rust
statsig.initialize_sync();
```

### Session replay & autocapture

Both are one-liners that take the client you've already built. Call them before (or right alongside) initialisation:

```rust
use statsig_wasm::{run_statsig_auto_capture, run_statsig_session_replay};

run_statsig_session_replay(&statsig);
run_statsig_auto_capture(&statsig);
```

Just remember they need the matching features in the JS bundle from [Setup](#-setup) — the `session-replay+web-analytics` build covers both.

⸻

## 🛠️ Requirements

- **A Wasm frontend**: this is a `wasm-bindgen` crate meant to compile to `wasm32` and run in the browser — it's not a native-target library
- **The Statsig JS client on the page**: the CDN script from [Setup](#-setup) has to be loaded, since the bindings call into the global `Statsig` namespace
- **Rust 1.85+**: the crate is on the 2024 edition, so you'll want a recent toolchain

⸻

## 📁 Repo & Contributions

🛠️ **Repo**: [https://github.com/dsplce-co/statsig-wasm](https://github.com/dsplce-co/statsig-wasm)<br>
📦 **Crate**: [https://crates.io/crates/statsig-wasm](https://crates.io/crates/statsig-wasm)

PRs welcome, feel free to contribute

⸻

## 📄 License

MIT or Apache-2.0, at your option.
