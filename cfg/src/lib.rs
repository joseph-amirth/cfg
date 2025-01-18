pub use cfg_core::*;
pub use cfg_macros::*;

pub mod prelude {
    pub use cfg_macros::*;

    pub use cfg_core::{
        interpret::Interpreter,
        parse::{CykParser, EarleyParser, FormatOptions, FormatStyle, Parser},
        Cfg, CfgBuilder, Rule, Symbol, Var,
    };
}
