use core::{
    borrow::BorrowMut,
    ops::Deref,
    pin::Pin,
};

use pin_utils::pin_mut;

use atomex::StrictOrderings;
use core_malloc::CoreAlloc as TestAlloc;
use mm_ptr::{
    x_deps::atomic_sync::x_deps::atomex,
    Shared,
};

use crate::{cancellation::*, x_deps::pin_utils};
use super::*;

async fn recv_async<B, C>(
    rx: Receiver<B, (), StrictOrderings>,
    mut tok: impl BorrowMut<C>,
) -> Result<(), RxError<()>>
where
    B: Deref<Target = Oneshot<(), StrictOrderings>>,
    C: TrCancellationToken,
{
    pin_mut!(rx);
    let cancel = unsafe { Pin::new_unchecked(tok.borrow_mut()) };
    rx.receive_async().may_cancel_with(cancel).await
}

#[tokio::test]
async fn receive_async_cancel() {
    let cts = Shared::new(
        CancellationTokenSource::new_in(
            TestAlloc::new(),
            StrictOrderings::default),
        TestAlloc::new(),
    );
    let oneshot = Shared::new(
        Oneshot::<(), StrictOrderings>::new(),
        TestAlloc::new(),
    );
    let Result::Ok((tx, rx)) = Oneshot::try_split(oneshot) else {
        panic!("[oneshot::tests_::receive_async_cancel] try_split failed");
    };
    let recv = tokio::task::spawn(recv_async(rx, cts.child_token()));
    cts.try_cancel();
    let x = recv.await;
    assert!(matches!(x, Result::Ok(Result::Err(RxError::Cancelled))));
    drop(tx)
}
