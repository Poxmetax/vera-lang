//! VERA Phase 1 — reference front-end + tree-walking interpreter.
//!
//! MVP subset per `docs/spec/SPEC.md` §3 (functions, Int/Bool/Str/Unit/Console,
//! let/if/calls, requires/ensures runtime checks, content-addressed defs).

pub mod ast;
pub mod interp;
pub mod lexer;
pub mod parser;
pub mod store;
pub mod typecheck;

pub use interp::{Console, Interpreter, Trap};
pub use parser::{parse, ParseError};
pub use store::CodebaseStore;
pub use typecheck::{check_program, TypeError};
