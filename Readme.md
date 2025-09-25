- `crates/macro` contains procedural macro to generate some boilerplate.
- `crates/user` contains user code for procedural macros. additionally, this explores how the same behaviour could be achieved using declarative macros.
- `show_streams` macro is a simpler macro to showcase the simple but effective way of debug-printing macros to get more understanding of how things work.

More resources:

- Learning Macros:
  - The book for a quick understanding: https://doc.rust-lang.org/book/ch20-05-macros.html
  - The reference for deeper application: https://doc.rust-lang.org/reference/macros-by-example.html#metavariables
  - https://github.com/dtolnay/cargo-expand : useful to verify what a macro does. Some IDE provide this functionality directly, try it!
- Useful blogs
  - https://medium.com/@alfred.weirich/the-rust-macro-system-part-1-an-introduction-to-attribute-macros-73c963fd63ea
  - https://towardsdatascience.com/nine-rules-for-creating-procedural-macros-in-rust-595aa476a7ff/
