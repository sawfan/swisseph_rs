# unknown_web (swisseph example)

This example builds a small `wasm32-unknown-unknown` `.wasm` module from the `swisseph` crate (as a Cargo `--example unknown_web`) and includes a minimal web runner (Trunk) that loads it in the browser.

Unlike the WASI demo, this target has **no WASI filesystem APIs**, so the wasm module registers a Swiss Ephemeris **VFS backend** and the JS host provides ephemeris assets by uploading their bytes into wasm memory.

## Build the wasm

You don’t need to manually copy the wasm into `web/`.

`web/Trunk.toml` includes build hooks that automatically:

- run a `pre_build` hook to compile the wasm into `target/wasm32-unknown-unknown/...`
- run a `post_build` hook to copy the resulting artifact into Trunk’s staging/dist directory as `swisseph_unknown.wasm`

This avoids a rebuild loop: if we were to copy the wasm into `web/` (the watched source dir), Trunk would detect the change and rebuild repeatedly.

If you want to build it manually anyway:

```bash
cargo build --manifest-path swisseph_rs/Cargo.toml --target wasm32-unknown-unknown --example unknown_web
cp target/wasm32-unknown-unknown/debug/examples/unknown_web.wasm swisseph_rs/examples/unknown_web/web/swisseph_unknown.wasm
```

## Run the web page (Trunk)

Install Trunk if you don't have it:

```bash
cargo install trunk
```

Then serve the site:

```bash
trunk serve --config swisseph_rs/examples/unknown_web/web/Trunk.toml --open
```

Trunk will build a static site under `web/dist` and serve it locally.

## Ephemeris assets

The web runner ships (or fetches) ephemeris files as web assets.

- Put the files under `web/assets/`.
- List them in `web/assets/manifest.json`.
- Trunk copies `web/assets/*` to `web/dist/assets/*`.
- `web/main.js` fetches the listed files and calls the wasm exports to mount them into the in-wasm VFS.

Default recommended ephemeris set:
- `seas_18.se1`
- `semo_18.se1`
- `sepl_18.se1`

The default `manifest.json` marks these as `optional: true`, so the page still runs even if you haven’t added the binary files.

Example `web/assets/manifest.json`:

```json
{
  "mount": "ephe",
  "files": [
    { "path": "seas_18.se1" },
    { "path": "sepl_18.se1" }
  ]
}
```

## What this demonstrates

- `libswisseph-sys` patched C sources compiling for `wasm32-unknown-unknown`
- JS host mounting `.se1` files into an in-wasm VFS
- `swisseph` high-level call path (`swe2::calc_ut2_ecliptic`) used inside the wasm export

