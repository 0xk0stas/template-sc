#![no_std]

use multiversx_sc::{derive_imports::*, imports::*};

#[multiversx_sc::contract]
pub trait Template {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}
}
