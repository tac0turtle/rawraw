# Architecture

Although this compiler is still a prototype, it was architected to be a suitable foundation for a production compiler rather than a throwaway proof of concept. The design decisions made so far are documented below,
and future maintainers are advised to read this document first before making major changes.

The two major components that are sketched out so far are:
- the compiler frontend - lexing, parsing and name resolution
- the LSP language server 

The above components were designed with an "IDE first" philosophy in mind. 
Nowadays, a programming language doesn't really feel like a "real" thing until there is some immediate feedback from an IDE, and the language server protocol makes this sort of functionality easy to replicate across actual IDEs.
Architecting parsing and diagnostics for a language server is in some ways more complex than a typical compiler frontend, 
so if we write the language server first, we can use the same tooling for both that and the compiler 
and also get immediate feedback on how our language feels to use.

## Frontend

The best advice available on the internet suggested following what [Rust Analyzer](https://github.com/rust-lang/rust-analyzer)
if you want IDE friendly language tooling in Rust.
So this compiler frontend uses the [rowan](https://docs.rs/rowan), [ungrammar](https://docs.rs/ungrammar) and [salsa](https://docs.rs/salsa)
crates used by Rust analyzer.

Here's a quick overview of the frontend structure:
* `ixc.ungram` - [ungrammar](https://docs.rs/ungrammar) file that describes the abstract syntax tree, NOT the actual "grammar"
* `build.rs` - this uses `ixc.ungram` to generate:
    * `src/frontend/lexer/lex_tokens.rs` - the [logos](https://docs.rs/logos) lexer `Token` enum
    * `src/frontend/syntax/syntax_kind.rs` - the `SyntaxKind` enum for [rowan](https://docs.rs/rowan)
    * `src/frontend/ast/nodes.rs`- type safe wrappers around the untyped [rowan](https://docs.rs/rowan) CST
* `db.rs` - [salsa](https://docs.rs/salsa) database for fast incremental computation
* `src/frontend` - the main module for the frontend
  * `lexer` - the [logos](https://docs.rs/logos) lexer
  * `parser` - handwritten parser based off of https://matklad.github.io/2023/05/21/resilient-ll-parsing-tutorial.html
  * `syntax` - [rowan](https://docs.rs/rowan) concrete syntax tree (CST) types
  * `ast` - typed abstract syntax tree (AST) wrapper around the untyped CST
  * `resolver` - TODO
  * `checker` - TODO


### Lexing, Parsing and Abstract/Concrete Syntax Trees

Initially, I thought that the way to save time would be to use an off-the-shelf parser crate even though most compilers use
handwritten parser. After getting something partially working with a popular parser combinator crate, I decided to switch
to the handwritten approach because the parser combinator approach didn't look any easier or more maintainable (YMMV).

The current handwritten parser implementation follows almost verbatim this article by Alex Kladov of the Rust Analyzer team: https://matklad.github.io/2023/05/21/resilient-ll-parsing-tutorial.html. The [logos](https://docs.rs/logos) crate is used to derive the lexer and the parser
takes lexed `Token`s and builds a concrete syntax tree (CST) using [rowan](https://docs.rs/rowan). An abstract syntax tree (AST)
is then code generated on top of the CST from an [ungrammar](https://docs.rs/ungrammar) file. This is all pretty similar to how things work in Rust analyzer.

## Language Server

The language server implementation lives in `src/lsp_server` and uses the [tower-lsp](https://docs.rs/tower-lsp) crate.
The root `LSPServer` type lives in `src/lsp_server/server.rs` and it holds a reference to the [salsa](https://docs.rs/salsa) database which holds all the incremental computations including the results of lexing, parsing, name resolution, etc.

Each file in `src/lsp_server` is intended to implement a single LSP operation or provide some utility functions. 
For example, the implementation of `textDocument.hover` is in `hover.rs` and `server.rs` just provides a thin wrapper
around the code there to implement the `LanguageServer` trait.

## Backend

The backend hasn't been implemented yet, but the intention was to first target Rust itself as the initial
code generation target and to leverage the existing [ixc](https://docs.rs/ixc) framework and its macros.
