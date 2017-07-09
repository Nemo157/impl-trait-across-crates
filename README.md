A small test case showing build failure from rust-lang/rust#40839.

To reproduce:

```sh
$ cd foo
$ cargo build
   Compiling bar v0.1.0 (file:///Users/Nemo157/sources/impl-trait-across-crates/bar)
   Compiling foo v0.1.0 (file:///Users/Nemo157/sources/impl-trait-across-crates/foo)
error: linking with `cc` failed: exit code: 1
  |
  = note: "cc" "-m64" "-L" "/Users/Nemo157/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib" "/Users/Nemo157/sources/impl-trait-across-crates/foo/target/debug/deps/foo-e44b5c1f474d8218.0.o" "-o" "/Users/Nemo157/sources/impl-trait-across-crates/foo/target/debug/deps/foo-e44b5c1f474d8218" "-Wl,-dead_strip" "-nodefaultlibs" "-L" "/Users/Nemo157/sources/impl-trait-across-crates/foo/target/debug/deps" "-L" "/Users/Nemo157/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib" "/Users/Nemo157/sources/impl-trait-across-crates/foo/target/debug/deps/libbar-5d1f8c94d5285ba5.rlib" "/Users/Nemo157/sources/impl-trait-across-crates/foo/target/debug/deps/libfutures-82f759521b139a28.rlib" "/Users/Nemo157/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/libstd-a260b5db713b337f.rlib" "/Users/Nemo157/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/librand-664091cbac310259.rlib" "/Users/Nemo157/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/libcollections-4e19d5a43d7fdd2c.rlib" "/Users/Nemo157/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/libstd_unicode-49cd3c7af2b2f27f.rlib" "/Users/Nemo157/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/libpanic_unwind-18bf5d50673f1daa.rlib" "/Users/Nemo157/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/libunwind-6b24ec54aa474d14.rlib" "/Users/Nemo157/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/liballoc-99e0cdfb2e11773a.rlib" "/Users/Nemo157/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/liballoc_jemalloc-7b14c3e63843fe84.rlib" "/Users/Nemo157/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/liblibc-109501e572ed7296.rlib" "/Users/Nemo157/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/libcore-f1125930d2c15bcd.rlib" "/Users/Nemo157/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib/libcompiler_builtins-594db3b2ae45efeb.rlib" "-l" "System" "-l" "pthread" "-l" "c" "-l" "m"
  = note: Undefined symbols for architecture x86_64:
            "bar::foo::_$u7b$$u7b$closure$u7d$$u7d$::__STATIC_FMTSTR::he0f1619a48125270", referenced from:
                bar::foo::_$u7b$$u7b$closure$u7d$$u7d$::h038e55ceda77c203 in foo-e44b5c1f474d8218.0.o
          ld: symbol(s) not found for architecture x86_64
          clang: error: linker command failed with exit code 1 (use -v to see invocation)

error: aborting due to previous error

error: Could not compile `foo`.

To learn more, run the command again with --verbose.
```
