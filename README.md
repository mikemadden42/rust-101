# rust-101

Examples from TCM Security's Rust 101 [course](https://academy.tcm-sec.com/p/rust-101)

```bash
# Check for formatting issues.
cargo fmt --check
cargo fmt --all -- --check
```

```bash
# Check package to catch common mistakes and improve your Rust code.
cargo clippy -- -Wclippy::pedantic -Wrustdoc::private_intra_doc_links -Wrustdoc::broken-intra-doc-links -Aclippy::single-match-else -Aclippy::default-trait-access -Aclippy::missing-panics-doc -Wclippy::missing-errors-doc
```
