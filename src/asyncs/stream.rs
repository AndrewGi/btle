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
    fn map<U, Funct>(self, f: Funct) -> Map<Self, Funct>
    where
        Self: Sized,
        Funct: FnMut(Self::Item) -> U,
    {
        Map {
            stream: self,
            function: f,
        }
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

pub struct Map<S, F> {
    stream: S,
    function: F,
}
impl<S: Unpin, F> Unpin for Map<S, F> {}
impl<T, S: Stream, F: FnMut(S::Item) -> T> Stream for Map<S, F> {
    type Item = T;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        unsafe { self.as_mut().map_unchecked_mut(|s| &mut s.stream) }
            .poll_next(cx)
            .map(|o| o.map(|t| unsafe { &mut self.get_unchecked_mut().function }(t)))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.stream.size_hint()
    }
}
