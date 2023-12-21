#[cfg(not(feature = "std"))]
pub use spin::{Lazy, Mutex, RwLock};

#[cfg(feature = "std")]
pub use ::{
    once_cell::sync::Lazy,
    parking_lot::{Mutex, RwLock},
};
