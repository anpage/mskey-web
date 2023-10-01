# mskey-web: Web front-end for umskt-rs

This is a web frontend for the [unofficial Rust port](https://github.com/anpage/umskt-rs) of the [UMSKT project](https://github.com/UMSKT/UMSKT) using [Leptos](https://leptos.dev/) with client-side rendering through WASM. It does not require a server beyond a simple static file host as all the computation is done inside users' browsers.

Like the CLI tool `mskey` included with umskt-rs, the `keys.json` provided by UMSKT is still required.

## TODO
The HTML is messy at the moment and does weird things on mobile. I'm working on refactoring the app to use Tailwind instead of Bulma.

## Development Requirements
* [The Rust toolchain](https://rustup.rs/)
* Trunk: `cargo install trunk`
* [Tailwind CLI](https://github.com/tailwindlabs/tailwindcss/releases) (available as `tailwindcss` on your PATH)
* (Optional) [just](https://github.com/casey/just)

If you don't want to install just, you can see the equivalent commands inside the `justfile`.

## Build Steps
1. Place `keys.json` in the project root
2. Run `just build`

## Serving Locally
If you want to just use the app locally on your own machine, you can start a simple webserver and launch the app in a browser by running:
```
just serve
```

## Building Tauri app
A Tauri wrapper is included, which allows the app to be run in its own window as a "desktop" program. You can run it in development mode with the following command:

(Node and npm are required)
```
just tauri-serve
```

You can build an installer package with:
```
just tauri-build
```
