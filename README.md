> We're dsplce.co, check out our work on [github.com/dsplce-co](https://github.com/dsplce-co) 🖤

# statsig-wasm

📈 Wasm bindings for the Statsig JavaScript (Web) SDK.

---

## 📦 Installation

In your project, run:

```sh
cargo add statsig-wasm
```

## 🔧 Setup

Add the following script to your HTML `<head>` tag to make Statsig available to your Wasm module:

```html
<script
  src="https://cdn.jsdelivr.net/npm/@statsig/js-client@3/build/statsig-js-client+session-replay+web-analytics.min.js"
  crossorigin="anonymous"
>
</script>
```

Bear in mind, if you're happy for Statsig to autoinitialise the client, just append `?apikey=<YOUR_CLIENT_API_KEY>` to the script's `src` above and uninstall this crate 😉

If you're looking to initialise the client yourself, read on.

---

## 🧪 Usage

Set a custom user ID and initialise the client:

```rust
use statsig_wasm::{StatsigClient, StatsigUser};

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

---

## 🔒 License

MIT or Apache-2.0, at your option.

---
