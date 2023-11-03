pub mod core {
    pub mod prelude;
}

pub mod umpteen {
    pub mod ast {
        pub mod expr;
        pub mod ops;
        pub mod stmt;
    }

    pub mod bytecode {
        pub mod chunk;
        pub mod compiler;
        pub mod instruction;
    }

    pub mod error {
        pub mod error;
    }

    pub mod token {
        pub mod lexer;
        pub mod token;
        pub mod token_type;
    }

    pub mod value {
        pub mod object;
        pub mod value;
    }
    pub mod runtime;
}
