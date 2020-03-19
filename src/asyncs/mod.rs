//! Async primitives wrappers/reexports for (`Mutex`, `mpsc`, `RwLock`, `task::spawn`). Just
//! wrappers around which ever async library is available (`tokio`, `async-std`, embedded, etc).
#[cfg(feature = "asyncs")]
pub mod poll_function;
#[cfg(feature = "asyncs")]
pub mod stream;
#[cfg(feature = "asyncs")]
pub mod sync;
#[cfg(feature = "asyncs")]
pub mod task;
#[cfg(feature = "asyncs")]
pub mod time;
