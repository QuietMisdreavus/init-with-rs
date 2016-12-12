//! A helper trait to initialize a data structure with custom code.
//!
//! This crate is meant to aid in initializing fixed arrays using something other than a `Default`
//! implementation. For example, if you wanted to create an array of Vecs, you could create one
//! with `Default` that made all the Vecs empty:
//!
//! ```rust
//! let my_array = <[Vec<u32>; 3]>::default();
//! assert_eq!(my_array, [[], [], []]);
//! ```
//!
//! But if you wanted to start the arrays with some state, you either need to start with the empty
//! arrays and fill from there, or drop into unsafe code to write in a partially-initialized array.
//!
//! ```rust
//! let mut my_array = <[Vec<usize>; 3]>::default();
//!
//! for (idx, arr) in my_array.iter_mut().enumerate() {
//!     for i in 0..(idx+1) {
//!         arr.push(i);
//!     }
//! }
//!
//! assert_eq!(my_array, [vec![0], vec![0, 1], vec![0, 1, 2]]);
//! ```
//!
//! With `InitWith`, the same array could be initialized like this:
//!
//! ```rust
//! use init_with::InitWith;
//!
//! let my_array = {
//!     let mut seed = Vec::new();
//!     let mut next_val = 0;
//!
//!     <[Vec<u32>; 3]>::init_with(|| {
//!         seed.push(next_val);
//!         next_val += 1;
//!         seed.clone()
//!     })
//! };
//!
//! assert_eq!(my_array, [vec![0], vec![0, 1], vec![0, 1, 2]]);
//! ```
//!
//! **Warning**: If the function given to `init_with` panics, any elements that have already been
//! created will not run their destructor. This means that any elements with heap allocations -
//! `Vec`, `Box`, etc - will leak their contents.

extern crate nodrop;

use std::{mem, ptr};

use nodrop::NoDrop;

/// A trait that allows you to create an instance of a type with a given function.
pub trait InitWith<T> {
    /// Create a new instance of this type using the given function to fill elements.
    fn init_with<F>(init: F) -> Self
        where F: FnMut() -> T,
              Self: Sized;
}

macro_rules! array_init {
    ($($N:expr),+) => {
        $(
            impl<T> InitWith<T> for [T; $N] {
                fn init_with<F>(mut init: F) -> Self
                    where F: FnMut() -> T
                {
                    let mut ret: NoDrop<[T; $N]> = unsafe { NoDrop::new(mem::uninitialized()) };

                    for i in 0..$N {
                        unsafe {
                            ptr::write(&mut ret[i], init());
                        }
                    }

                    ret.into_inner()
                }
            }
        )+
    };
}

array_init!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
            11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
            21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
            31, 32);

#[cfg(test)]
mod tests {
    use super::InitWith;

    #[test]
    fn expected() {
        let val = <[i32; 3]>::init_with(|| 4);
        assert_eq!(val, [4, 4, 4]);
    }
}
