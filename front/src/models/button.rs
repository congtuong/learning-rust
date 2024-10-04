use dioxus::prelude::*;
use std::fmt;
pub enum ButtonType {
    Primary,
    Secondary,
}

impl fmt::Display for ButtonType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ButtonType::Primary => write!(f, "text-slate-200 bg-blue-500 inline-flex items-center border-0 py-1 px-3 focus:outline-none rounded md:mt-0 button hover:bg-slate-300 active:bg-slate-500"),
            ButtonType::Secondary => write!(f, "text-slate-200 bg-red-500 inline-flex items-center border-0 py-1 px-3 focus:outline-none rounded md:mt-0 button hover:bg-slate-300 active:bg-slate-500"),
        }
    }
}
