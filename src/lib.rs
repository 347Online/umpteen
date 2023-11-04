mod umpteen {
    pub mod ast {
        mod ast;
        mod expr;
        mod ops;
        mod stmt;

        pub use ast::*;
        pub use expr::*;
        pub use ops::*;
        pub use stmt::*;
    }

    pub mod bytecode {
        mod bytecode;
        mod chunk;
        mod compiler;
        mod instruction;
        mod serialize;

        pub use chunk::*;
        pub use compiler::*;
        pub use instruction::*;
        pub use serialize::*;
    }

    pub mod error {
        mod error;
        pub use error::*;
    }

    pub use error::*;

    pub mod token {
        mod lexer;
        mod token;
        mod token_type;

        pub use lexer::*;
        pub use token::*;
        pub use token_type::*;
    }

    pub mod value {
        mod object;
        mod value;

        pub use object::*;
        pub use value::*;
    }

    mod exec;
    mod runtime;

    pub use exec::*;
    pub use runtime::*;
}

pub use umpteen::*;

pub type Result<T> = std::result::Result<T, Error>;
