use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

pub struct Ready<T>(Option<T>);
impl<T> Unpin for Ready<T> {}
impl<T> Future for Ready<T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(self.0.take().expect("Ready polled more than once"))
    }
}
pub fn ready<T>(t: T) -> Ready<T> {
    Ready(Some(t))
}

pub trait FutureExt: Future {
    fn map<U, Funct>(self, f: Funct) -> Map<Self, Funct>
    where
        Self: Sized,
        Funct: FnOnce(Self::Output) -> U,
    {
        Map {
            future: self,
            function: Some(f),
        }
    }
}

pub struct Map<Fut, Funct> {
    future: Fut,
    function: Option<Funct>,
}
impl<T, Fut: Future, Funct: FnOnce(Fut::Output) -> T> Future for Map<Fut, Funct> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        unsafe { self.as_mut().map_unchecked_mut(|s| &mut s.future) }
            .poll(cx)
            .map(|output| {
                unsafe { self.get_unchecked_mut() }
                    .function
                    .take()
                    .expect("poll called more than twice")(output)
            })
    }
}
