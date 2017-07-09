A small test case showing build failure from [rust-lang/rust#40839](https://github.com/rust-lang/rust/issues/40839).

To reproduce:

```sh
$ cd foo
$ cargo +nightly-2017-07-08 build
   Compiling foo v0.1.0 (file:///Users/Nemo157/sources/impl-trait-across-crates/foo)
error: linking with `cc` failed: exit code: 1
  |
  = note: "cc" "-m64" "-L" "/Users/Nemo157/.rustup/toolchains/nightly-2017-07-08-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib" "/Users/Nemo157/sources/impl-trait-across-crates/foo/target/debug/deps/foo-509190d9c4c5257f.0.o" "-o" "/Users/Nemo157/sources/impl-trait-across-crates/foo/target/debug/deps/foo-509190d9c4c5257f" "/Users/Nemo157/sources/impl-trait-across-crates/foo/target/debug/deps/foo-509190d9c4c5257f.crate.allocator.o" "-Wl,-dead_strip" "-nodefaultlibs" "-L" "/Users/Nemo157/sources/impl-trait-across-crates/foo/target/debug/deps" "-L" "/Users/Nemo157/.rustup/toolchains/nightly-2017-07-08-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib" "/Users/Nemo157/sources/impl-trait-across-crates/foo/target/debug/deps/libbar-e44f74429dcb6ea1.rlib" "/Users/Nemo157/sources/impl-trait-across-crates/foo/target/debug/deps/libfutures-6ae93a2023d032bb.rlib" "/Users/Nemo157/.rustup/toolchains/nightly-2017-07-08-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/libstd-44a947b098ab6362.rlib" "/Users/Nemo157/.rustup/toolchains/nightly-2017-07-08-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/liballoc_jemalloc-e0b819d1caefc46e.rlib" "/Users/Nemo157/.rustup/toolchains/nightly-2017-07-08-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/liballoc_system-c7b3dfd897c71657.rlib" "/Users/Nemo157/.rustup/toolchains/nightly-2017-07-08-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/librand-478ce98ecaec6d15.rlib" "/Users/Nemo157/.rustup/toolchains/nightly-2017-07-08-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/libpanic_unwind-512815c9bc06f295.rlib" "/Users/Nemo157/.rustup/toolchains/nightly-2017-07-08-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/libunwind-cd8cd76eb0a0bb43.rlib" "/Users/Nemo157/.rustup/toolchains/nightly-2017-07-08-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/liblibc-b81a73784bc4a25a.rlib" "/Users/Nemo157/.rustup/toolchains/nightly-2017-07-08-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/liballoc-51aa53f169f1f37b.rlib" "/Users/Nemo157/.rustup/toolchains/nightly-2017-07-08-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/libstd_unicode-b6c504ac776266c1.rlib" "/Users/Nemo157/.rustup/toolchains/nightly-2017-07-08-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/libcore-10cda230398f0728.rlib" "/Users/Nemo157/.rustup/toolchains/nightly-2017-07-08-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/libcompiler_builtins-d115e8b221fa0ba7.rlib" "-l" "System" "-l" "resolv" "-l" "pthread" "-l" "c" "-l" "m"
  = note: Undefined symbols for architecture x86_64:
            "bar::foo::_$u7b$$u7b$closure$u7d$$u7d$::__STATIC_FMTSTR::h1e388498590a2043", referenced from:
                bar::foo::_$u7b$$u7b$closure$u7d$$u7d$::h19e0a6247ba53051 in foo-509190d9c4c5257f.0.o
          ld: symbol(s) not found for architecture x86_64
          clang: error: linker command failed with exit code 1 (use -v to see invocation)

error: aborting due to previous error

error: Could not compile `foo`.

To learn more, run the command again with --verbose.
```

---

Also, a small test case showing build failure from [rust-lang/rust#43135](https://github.com/rust-lang/rust/issues/43135).

To reproduce:

```sh
$ cd foo2
$ cargo +nightly-2017-07-08 build
   Compiling foo2 v0.1.0 (file:///Users/Nemo157/sources/impl-trait-across-crates/foo2)
error: internal compiler error: src/librustc_trans/collector.rs:662: Cannot create local trans-item for DefId { krate: CrateNum(12), node: DefIndex(9) => bar/ab23142::foo2[0]::{{closure}}[0]::msg[0] }

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.20.0-nightly (9b85e1cfa 2017-07-07) running on x86_64-apple-darwin

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:489:8
note: Run with `RUST_BACKTRACE=1` for a backtrace.

error: Could not compile `foo2`.

To learn more, run the command again with --verbose.
```
