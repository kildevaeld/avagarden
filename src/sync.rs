#[cfg(not(feature = "std"))]
pub use spin::{Lazy, Mutex, MutexGuard, Once, RwLock, RwLockReadGuard, RwLockWriteGuard};

#[cfg(feature = "std")]
pub use ::{
    once_cell::sync::Lazy,
    parking_lot::{Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard},
    std::sync::Once,
};
