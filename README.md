`llvm-safe` provides Rust bindings to LLVM in a way that should statically prevent breaking LLVM's invariants in a memory unsafe way. Using this library, a compiler should not be memory unsafe. It can, however, still generate inlvalid IR or memory unsafe programs. `llvm-safe` should have no runtime checks and therefore no runtime overhead over calling into the LLVM-C API directly.

In particular, all of the following should be statically prevented:

- Using any LLVM objects after they have been destroyed.
- Using an instruction builder before it has been positioned.
- Using an instruction builder that has been positioned to a destroyed block.
- Touching a `Module` after the `Context` it was created in is destroyed.
- Touching a `Function` after the `Module` it was created in is destroyed.
- Using `Type`s/`Module`s/`Value`s created in one `Context` in a different `Context`.
- Calling functions defined in one `Module` from a different `Module`.
- Using `Values` defined in one `Function` from a different `Function`.
- Adding incoming values to non-phi instructions.
- Referencing blocks from different functions in phi instructions.

This crate is a work in progress: there are probably things that should be prevented that aren't, and there are probably things that are prevented that shouldn't be.
