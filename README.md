# Royallist
## In Rust

A glorified "ls" command.

Don't be surprised, it's just ls with icons and rewritten in Rust

![Royallist example](./.github/assets/royallist-nextjs.png)

## How to use?

Call royallist and watch you current directory files shine in the terminal. If you want to use
another directory, pass it as a first argument just like "ls" and on you.

```bash
# To print local directory
royallist

# Or add as a parameter the desired directory
royallist ~/Documents

# Or even multiple paths
royallist ~/Documents ~/Pictures
```

For ease of use, I recommend aliasing Royallist to a simpler command:

Just add the following to your shell RC file

```bash
alias l="royallist"
```

## Icons

All icons can be changed in the `./src/icons.rs` file. Keep in mind that you
must use a font that suports those icons. In this case, all icons are part of
the `NerdFont` icon suite.

## Install

Please note you need a NerdFont or similar.

In order to install, do `make install` or compile the source code with `cargo build --release` and
put the binary located in `target/release/royallist` in your $PATH.
