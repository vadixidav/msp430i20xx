#![no_std]
#![feature(const_mut_refs, abi_msp430_interrupt)]

// #![allow(dead_code, clippy::return_self_not_must_use)]
use modular_bitfield::prelude::*;

#[bitfield]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Lpm45Ctl {
    pub lock_lpm_45: bool,
    pub lpm_45_ifg: bool,
    #[skip]
    reserved0: B2,
    pub reg_off: bool,
    #[skip]
    reserved1: B3,
}
