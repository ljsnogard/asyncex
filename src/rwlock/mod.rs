//! An asynchronous reader-writer lock with no heap memory allocation.
//! 
mod contexts_;
mod impl_;
mod reader_;
mod writer_;
mod upgrade_;

pub use impl_::{Acquire, RwLock};
pub use reader_::{ReadAsync, ReadFuture, ReaderGuard};
pub use upgrade_::{
    Upgrade, UpgradeAsync, UpgradeFuture,
    UpgradableReadAsync, UpgradableReadFuture, UpgradableReaderGuard,
};
pub use writer_::{WriteAsync, WriteFuture, WriterGuard};

#[cfg(test)]mod tests_;
