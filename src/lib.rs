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
//! Alternatively, `init_with_indices` can be used to more easily create array entries based on their index:
//!
//! ```rust
//! use init_with::InitWith;
//!
//! let squares = <[usize; 5]>::init_with_indices(|i| i*i);
//!
//! assert_eq!(squares, [0,1,4,9,16]);
//! ```
//!
//! This crate is built with `#![no_std]` and only uses libcore for its code, so it can be used
//! from other `no_std` crates.

#![no_std]

/// A trait that allows you to create an instance of a type by using a given function to generate
/// each element.
pub trait InitWith<T> {
    /// Create a new instance of this type using the given function to fill elements.
    ///
    /// # Examples
    ///
    /// Prefilling an array with a Vec, with no unsafe code:
    ///
    /// ```rust
    /// use init_with::InitWith;
    ///
    /// let src = vec![1, 2, 3];
    /// let dest: [i32; 3] = {
    ///     let mut idx = 0;
    ///
    ///     <[i32; 3]>::init_with(|| {
    ///         let val = src[idx];
    ///         idx += 1;
    ///         val
    ///     })
    /// };
    ///
    /// assert_eq!(src, dest);
    /// ```
    fn init_with<F>(init: F) -> Self
    where
        F: FnMut() -> T,
        Self: Sized;

    /// Create a new instance of this type to fill elements
    /// by mapping the given function over the new array's indices.
    ///
    /// # Examples
    ///
    /// Prefilling an array of even numbers, with no unsafe code:
    ///
    /// ```rust
    /// use init_with::InitWith;
    ///
    /// let src = vec![0, 2, 4];
    /// let dest = <[i32; 3]>::init_with_indices(|x| 2*x as i32);
    ///
    ///
    /// assert_eq!(src, dest);
    /// ```
    fn init_with_indices<F>(init: F) -> Self
    where
        F: FnMut(usize) -> T,
        Self: Sized;
}

macro_rules! array_init {
    {$n:expr, $init:ident, $($stack:ident,)+} => {
        impl<T> InitWith<T> for [T; $n] {
            fn init_with<F>(mut $init: F) -> Self
                where F: FnMut() -> T,
                Self: Sized
            {
                [$init(), $($stack()),+]
            }
            fn init_with_indices<F>(mut $init: F) -> Self
                where F: FnMut(usize) -> T
            {
                build_incrementing_list!([], 0, $init, $($stack),+)
            }
        }
        array_init!{($n - 1), $($stack,)+}
    };
    {$n:expr, $init:ident,} => {
        impl<T> InitWith<T> for [T; $n] {
            fn init_with<F>(mut $init: F) -> Self
                where F: FnMut() -> T,
                Self: Sized
            {
                [$init()]
            }
            fn init_with_indices<F>(mut $init: F) -> Self
                where F: FnMut(usize) -> T,
                Self: Sized
            {
                [$init(0)]
            }
        }
        array_init!{($n - 1)}
    };
    {$n:expr} => {
        impl<T> InitWith<T> for [T; $n] {
            fn init_with<F>(_init: F) -> Self
                where F: FnMut() -> T,
                Self: Sized
            {
                []
            }
            fn init_with_indices<F>(_: F) -> Self
                where F: FnMut(usize) -> T,
                Self: Sized
            {
                []
            }
        }
    };
}

macro_rules! build_incrementing_list {
	{[$($result:tt)*], $n:expr, $head:ident} => { 
		[$($result)* $head($n),]
	};
	{[$($result:tt)*], $n:expr, $head:ident, $($stack:ident),+} => { 
		build_incrementing_list!([$($result)* $head($n),], $n+1, $($stack),+)
	};
}

array_init!{32, init, init, init, init, init, init, init, init, init, init, init, init, init, init, init, init, init, init, init, init, init, init, init, init, init, init, init, init, init, init, init, init,}

#[cfg(test)]
mod tests {
    use super::InitWith;

    #[test]
    fn expected() {
        let val = <[i32; 3]>::init_with(|| 4);
        assert_eq!(val, [4, 4, 4]);
    }

    #[test]
    fn expected_build_incrementing_list() {
        let val = <[usize; 5]>::init_with_indices(|x| x);
        assert_eq!(val, [0, 1, 2, 3, 4]);
    }
}
