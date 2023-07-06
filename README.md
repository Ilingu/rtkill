# RTKill :: Rust Target Folder Killer

#### ➡ An interactive terminal user iterface (TUI), that search and delete 'rust /target folders'.

https://github.com/Ilingu/rtkill/assets/57411599/875e31d1-bf5e-4c21-8a95-0dbb55cb3f03

## Origin

The rust `target` folder is what allow your program to run; so it can get (depending on the size of your application) **very large** (for an intermediate app it's easily >5GB).

If you come from `JavaScript` and especially `NodeJS`, the `node_modules` folder can be an analogy for the rust `target` folder.

Therefore, this folder must be deleted whenever the project bound to it is finished, in order to save space on my disk.

But keeping track of every rust projects can be very **tedious** (if you watch the demo video above, you'll see 107 'target' folders which is quite big for my brain).

So I made this tui app that will scan recursively the folder you specified to find any 'target' rust folders, it'll then shows the result on screen where you'll be able to choose whichever you want to delete.

**Be careful**, when you press `space` on the one you've selected, it **instantaneously** delete it, it does not mark it for deletion, if you press `space`, it's gone <ins>**forever**</ins>

## Purpose

- Improve my rust skillset
- Have fun

> Thus do not take this project seriously, it's just a fun side project.

## Installation

> This TUI might not works on other OS than linux, do it at your own risk.

Build from source with `cargo`, or download (_if you're on linux_) the **linux** executable from the [`release page`](https://github.com/Ilingu/rtkill/releases)

```bash
cargo build --release # will creates a single executable for your os in ./target/release, named "rtkill" (with the associated executable extension in your os)
```

## Made with:

1. **Elegance** ✅
2. `RUST` ✨🦀
3. [tui-rs](https://github.com/fdehau/tui-rs) ♥ (awesome lib btw)
4. go see [Cargo.toml](/Cargo.toml)

## Credit

This project, is **heavily** inspired by [`npkill`](https://github.com/voidcosmos/npkill), which does the same thing but for the `node_modules` folder from the `NodeJS` ecosystem.
