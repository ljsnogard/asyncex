use core::{
    fmt,
    future::Future,
    mem::MaybeUninit,
    pin::Pin,
    task::{Context, Poll, Waker},
};
use atomex::{
    AtomicFlags, StrictOrderings, TrAtomicFlags,
    TrAtomicLockConfig, TrAtomicLockSignal,
};
use fastmm::{
    mem_alloc::TrMalloc,
    sync::{Shared, XtMallocShared},
    x_deps::atomic_sync::x_deps::atomex,
};

pub fn new_with_alloc<T, A>(
    capacity: usize,
    alloc: &A,
) -> Result<(Sender<T, A>, Receiver<T, A>), usize>
where
    A: TrMalloc + Clone,
{
    todo!()
}

pub enum SenderError<T> {
    Divoced(T),
    Congested(T),
}

#[derive(Debug)]
pub struct Sender<T, A: TrMalloc + Clone> {
    wake_: Option<Waker>,
    pipe_: Shared<Bounded<T, A>, A>,
}

impl<T, A: TrMalloc + Clone> Sender<T, A> {
    #[inline]
    pub fn is_divorced(&self) -> bool {
        Bounded::is_divorced(&self.pipe_)
    }

    pub fn try_send(
        &mut self,
        msg: T,
    ) -> Result<usize, SenderError<T>> {
        todo!()
    }

    pub async fn send_async(
        &mut self,
        msg: T,
    ) -> Result<usize, SenderError<T>> {
        todo!()
    }
}

impl<T, A: TrMalloc + Clone> Drop for Sender<T, A> {
    fn drop(&mut self) {
        todo!()
    }
}

pub struct Receiver<T, A: TrMalloc + Clone> {
    wake_: Option<Waker>,
    pipe_: Shared<Bounded<T, A>, A>,
}

impl<T, A: TrMalloc + Clone> Receiver<T, A> {
    #[inline]
    pub fn is_divorced(&self) -> bool {
        Bounded::is_divorced(&self.pipe_)
    }

    pub fn try_receive(&mut self) -> Result<Option<T>, SenderError<T>> {
        todo!()
    }

    pub async fn send_async(
        &mut self,
        msg: T,
    ) -> Result<usize, SenderError<T>> {
        todo!()
    }
}

impl<T, A: TrMalloc + Clone> Drop for Receiver<T, A> {
    fn drop(&mut self) {
        todo!()
    }
}

#[derive(Debug)]
struct Bounded<T, A: TrMalloc + Clone> {
    array_: Shared<[MaybeUninit<T>], A>,
    index_: AtomicFlags<usize>,
}

impl<T, A: TrMalloc + Clone> Bounded<T, A> {
    fn is_divorced(pipe: &Shared<Self, A>) -> bool {
        Shared::strong_count(pipe) > 1usize
    }
}
