//! Anchor-compatible SDK for the Raydium farm program.

#![deny(rustdoc::all)]
#![allow(rustdoc::missing_doc_code_examples)]
#![allow(clippy::nonstandard_macro_braces)]

mod accounts;
mod instructions;

pub use accounts::*;
pub use instructions::*;

use anchor_lang::prelude::*;

declare_id!("J3npseAcnFi9jVwbCRd1dfng7HEzcnnCNDM4wc4oSu9R");

/// Farm Program
#[derive(Clone)]
pub struct Farm;

impl anchor_lang::Id for Farm {
    fn id() -> Pubkey {
        ID
    }
}
