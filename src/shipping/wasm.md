# WebAssembly

WebAssembly builds can be created using either the automatic approach using the editor or manual. This chapter covers 
both ways.

## Automated

Use the [project exporter](shipping.md) for automated builds.

## Manual

WebAssembly builds requires a bit of preparations. Install `wasm-pack` first:

```shell
cargo install wasm-pack
```

Then run the following commands:

```shell
cd executor-wasm
wasm-pack build --target=web --release 
```

This command will produce `pkg` folder in the `executor-wasm` directory. Now create a folder for your game build, and
you need to copy the `pkg` folder together with `index.html`, `main.js`, `styles.css` to the folder of your final build.
As the last step you need to copy `data` folder in the same folder.