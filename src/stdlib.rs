//! Load the Nickel standard library in strings at compile-time.

use crate::identifier::Ident;
use crate::term::make as mk_term;
use crate::term::RichTerm;

pub const BUILTIN: (StdlibModule, &str, &str) = (
    StdlibModule::Builtin,
    "<stdlib/builtin.ncl>",
    include_str!("../stdlib/builtin.ncl"),
);
pub const CONTRACT: (StdlibModule, &str, &str) = (
    StdlibModule::Contract,
    "<stdlib/contract.ncl>",
    include_str!("../stdlib/contract.ncl"),
);
pub const ARRAY: (StdlibModule, &str, &str) = (
    StdlibModule::Array,
    "<stdlib/array>",
    include_str!("../stdlib/array.ncl"),
);
pub const RECORD: (StdlibModule, &str, &str) = (
    StdlibModule::Record,
    "<stdlib/record>",
    include_str!("../stdlib/record.ncl"),
);
pub const STRING: (StdlibModule, &str, &str) = (
    StdlibModule::String,
    "<stdlib/string>",
    include_str!("../stdlib/string.ncl"),
);
pub const NUM: (StdlibModule, &str, &str) = (
    StdlibModule::Num,
    "<stdlib/num>",
    include_str!("../stdlib/num.ncl"),
);
pub const FUNCTION: (StdlibModule, &str, &str) = (
    StdlibModule::Function,
    "<stdlib/function>",
    include_str!("../stdlib/function.ncl"),
);
pub const INTERNALS: (StdlibModule, &str, &str) = (
    StdlibModule::Internals,
    "<stdlib/internals>",
    include_str!("../stdlib/internals.ncl"),
);

/// Return the list `(name, source_code)` of all the stdlib modules.
pub fn modules() -> Vec<(StdlibModule, &'static str, &'static str)> {
    vec![
        BUILTIN, CONTRACT, ARRAY, RECORD, STRING, NUM, FUNCTION, INTERNALS,
    ]
}

/// Represents a particular Nickel standard library module.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum StdlibModule {
    Builtin,
    Contract,
    Array,
    Record,
    String,
    Num,
    Function,
    Internals,
}

impl TryFrom<Ident> for StdlibModule {
    type Error = &'static str;

    fn try_from(value: Ident) -> Result<Self, Self::Error> {
        let module = match value.label() {
            "builtin" => StdlibModule::Builtin,
            "contract" => StdlibModule::Contract,
            "array" => StdlibModule::Array,
            "record" => StdlibModule::Record,
            "string" => StdlibModule::String,
            "num" => StdlibModule::Num,
            "function" => StdlibModule::Function,
            "internals" => StdlibModule::Internals,
            _ => return Err("Unknown stdlib module name"),
        };
        Ok(module)
    }
}

macro_rules! generate_accessor {
    ($value:ident) => {
        pub fn $value() -> RichTerm {
            mk_term::var(format!("${}", stringify!($value)))
        }
    };
}

/// Accessors to the builtin contracts.
pub mod contract {
    use super::*;

    // `dyn` is a reserved keyword in rust
    pub fn dynamic() -> RichTerm {
        mk_term::var("$dyn")
    }

    generate_accessor!(num);
    generate_accessor!(bool);
    generate_accessor!(string);
    generate_accessor!(array);
    generate_accessor!(func);
    generate_accessor!(forall_var);
    generate_accessor!(fail);
    generate_accessor!(enums);
    generate_accessor!(enum_fail);
    generate_accessor!(record);
    generate_accessor!(dyn_record);
    generate_accessor!(record_extend);
    generate_accessor!(forall_tail);
    generate_accessor!(dyn_tail);
    generate_accessor!(empty_tail);
}

pub mod internals {
    use super::*;

    generate_accessor!(push_default);
    generate_accessor!(push_force);
}
