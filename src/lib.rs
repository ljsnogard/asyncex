#![no_std]
#![feature(type_alias_impl_trait)]
#![feature(try_trait_v2)]

// We always pull in `std` during tests, because it's just easier
// to write tests when you can assume you're on a capable platform
#[cfg(test)]
extern crate std;

pub mod cancellation;
pub mod channel;
pub mod rwlock;

pub mod x_deps {
    pub use mm_ptr;

    pub use mm_ptr::x_deps::atomic_sync;
    pub use atomic_sync::x_deps::{abs_sync, atomex};
    pub use abs_sync::x_deps::pin_utils;
}
