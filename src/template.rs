#![no_std]

use multiversx_sc::imports::*;

mod proxy;

#[multiversx_sc::contract]
pub trait Template {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}
}
