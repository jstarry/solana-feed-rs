#![no_std]

use arrayref::array_mut_ref;
use core::mem::size_of;
use solana_sdk_bpf_types::*;

pub struct UserAccountData<'a> {
    pub banned: &'a mut bool,
    pub creator: &'a mut SolPubkey,
}
impl<'a> UserAccountData<'a> {
    pub fn new(data: &'a mut [u8]) -> Self {
        let (banned, creator) = data.split_at_mut(1);
        Self {
            banned: unsafe { &mut *(&mut banned[0] as *mut u8 as *mut bool) },
            creator: array_mut_ref!(creator, 0, size_of::<SolPubkey>()),
        }
    }
}

pub struct MessageAccountData<'a> {
    pub next_message: &'a mut SolPubkey,
    pub from: &'a mut SolPubkey,
    pub creator: &'a mut SolPubkey,
    pub text: &'a mut [u8],
}
impl<'a> MessageAccountData<'a> {
    pub fn new(data: &'a mut [u8]) -> Self {
        let (next_message, rest) = data.split_at_mut(size_of::<SolPubkey>());
        let (from, rest) = rest.split_at_mut(size_of::<SolPubkey>());
        let (creator, text) = rest.split_at_mut(size_of::<SolPubkey>());
        Self {
            next_message: array_mut_ref!(next_message, 0, size_of::<SolPubkey>()),
            from: array_mut_ref!(from, 0, size_of::<SolPubkey>()),
            creator: array_mut_ref!(creator, 0, size_of::<SolPubkey>()),
            text,
        }
    }
}
