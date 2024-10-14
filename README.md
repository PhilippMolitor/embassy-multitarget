# Embassy Multitarget

## ToDo

- [ ] Clean up `xtask` script
  - [ ] General code cleanup
  - [ ] Implement objcopy script via `cargo-bintutils` crate instead of command
  - [ ] Implement DFU flashing via `cargo-dfu` crate instead of command
  - [ ] Add bin -> UF2 conversion via `uf2conv-rs` crate
  - [ ] Implement UF2 flashing (somehow)

## Crates

```
- xtask
- hal
- app
- targets/
  - [target-name]
```

## Tasks

### Information

```sh
cargo xtask list-targets [--long -l]
```

```sh
cargo xtask list-features
```

### Firmware

```sh
cargo xtask fw build
  --target -t | --all-targets
  --features -f | --all-features
  --log-level -l
  --release -r
```

```sh
cargo xtask fw flash
  <<--target / -t> | [--all-targets]>
  [[--features -f] | [--all-features]]
  [--log-level / -l]
  [--release / -r]
  <--method / -m>
  [--device / -d]
```
