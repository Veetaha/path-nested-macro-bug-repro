# Bug report for `#[path = concat!(env!("OUT_DIR"), "/generated.rs")]`

This feature doesn't work.

If you run `cargo check`, you'll see the following output:

```
$ cargo check
   Compiling path-nested-macro v0.1.0 (/home/veetaha/junk/path-nested-macro)
error[E0583]: file not found for module `generated`
 --> src/lib.rs:9:1
  |
9 | mod generated;
  | ^^^^^^^^^^^^^^
  |
  = help: to create the module `generated`, create file "src/generated.rs" or "src/generated/mod.rs"

error[E0425]: cannot find function `bar` in module `generated`
  --> src/lib.rs:13:16
   |
13 |     generated::bar();
   |                ^^^ not found in `generated`

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0425, E0583.
For more information about an error, try `rustc --explain E0425`.
error: could not compile `path-nested-macro`

To learn more, run the command again with --verbose.
```

# Meta

```
$ cargo --version --verbose
cargo 1.54.0 (5ae8d74b3 2021-06-22)
release: 1.54.0
commit-hash: 5ae8d74b3b2d58f32c8d357e5cfa04d430a70e0b
commit-date: 2021-06-22

$ rustc --version --verbose
rustc 1.54.0 (a178d0322 2021-07-26)
binary: rustc
commit-hash: a178d0322ce20e33eac124758e837cbd80a6f633
commit-date: 2021-07-26
host: x86_64-unknown-linux-gnu
release: 1.54.0
LLVM version: 12.0.1
```
