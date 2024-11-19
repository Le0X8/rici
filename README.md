# RiCI?

_**R**unning **i**n **C**ontinuous **I**ntegration?_

## What is this?

This is a simple tool to check if the current code is running in a CI environment. It is useful to adjust some behaviors of your code when running in CI.

## How to use?

Just install the crate and use the `rici` function to check if the code is running in a CI environment.

```rust
use rici::rici;

fn main() {
    if rici() {
        println!("Running in CI");
    } else {
        println!("Not running in CI");
    }
}
```

## Supported CI environments

- GitHub Actions

More CI environments will be supported in the future.
