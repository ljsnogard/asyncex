mod source_;
mod token_;

pub use mm_ptr::x_deps::atomic_sync::x_deps::abs_sync::cancellation::*;
pub use source_::CancellationTokenSource;
pub use token_::CancellationToken;

#[cfg(test)]mod tests_;
