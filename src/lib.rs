pub mod umpteen {
    pub mod ast {
        pub mod ast;
        pub mod expr;
        pub mod ops;
        pub mod stmt;

        pub use ast::*;
        pub use expr::*;
        pub use ops::*;
        pub use stmt::*;
    }

    pub mod bytecode {
        pub mod chunk;
        pub mod compiler;
        pub mod instruction;
        pub mod serialize;

        pub use chunk::*;
        pub use compiler::*;
        pub use instruction::*;
        pub use serialize::*;
    }

    pub mod error {
        pub mod error;

        pub use error::*;
    }

    pub mod token {
        pub mod lexer;
        pub mod token;
        pub mod token_type;

        pub use lexer::*;
        pub use token::*;
        pub use token_type::*;
    }

    pub mod value {
        pub mod object;
        pub mod value;

        pub use object::*;
        pub use value::*;
    }

    pub mod exec;
    pub mod runtime;

    pub use exec::*;
    pub use runtime::*;
}

pub use umpteen::*;

pub mod prelude {
    use crate::{bytecode::Chunk, error::Error};

    pub type Program = Vec<Chunk>;
    pub type Result<T> = std::result::Result<T, Error>;
}

pub use prelude::*;