#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::single_match_else)]
#![allow(clippy::manual_let_else)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::missing_errors_doc)]

pub mod allanime;

pub use cynic::QueryBuilder;

pub mod prelude {
    pub use super::allanime;
    pub use cynic::QueryBuilder;
}
