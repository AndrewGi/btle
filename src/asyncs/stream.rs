use core::pin::Pin;
use core::task::{Context, Poll};

/// Reexport `futures_core::Stream`
pub use futures_core::Stream;

/// Stream Extensions. Similar to `futures_utils::StreamExt` but stripped down and doesn't require
/// bringing in the whole `futures_util` crate. Designed just for internal use.
pub trait StreamExt: Stream {
    fn next(&mut self) -> Next<Self>
    where
        Self: Unpin,
    {
        Next(self)
    }
}

impl<S: Stream> StreamExt for S {}

pub struct Next<'a, T: ?Sized>(&'a mut T);

impl<'a, T: ?Sized + Stream + Unpin> core::future::Future for Next<'a, T> {
    type Output = Option<T::Item>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut *self.0).poll_next(cx)
    }
}
