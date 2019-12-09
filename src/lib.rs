//! Macros to work with `std::collections` that are mostly straight-forward syntactic sugars.
pub use std::collections::HashMap;
pub use std::collections::HashSet;

/// Counts the number of `args` passed to this macro invocation.
///
/// Returns the count as `usize`.
///
/// # Examples
///
/// ```
/// #[macro_use] extern crate colmac;
///
/// assert_eq!(0, count_args!());
/// assert_eq!(1, count_args!("one"));
/// assert_eq!(3, count_args!("one", "two", "three"));
/// ```
#[macro_export]
macro_rules! count_args {
    // base cases
    () => {
        0usize
    };
    ( $arg:expr ) => {
        1usize
    };
    // recurse
    ( $arg:expr, $( $rest:expr ),* ) => {
        1usize + count_args!( $( $rest ),* )
    };
}

/// Sugar for `String::new` and `String::from`.
///
/// # Examples
///
/// ```
/// #[macro_use] extern crate colmac;
///
/// // create an empty string
/// assert_eq!(String::new(), string!());
///
/// // syntactically identical
/// assert_eq!(String::from("abc"), string!("abc"));
/// ```
#[macro_export]
macro_rules! string {
    () => {
        String::new()
    };
    ( $arg:expr ) => {
        String::from($arg)
    };
}

/// Just like `vec!`, but for `std::collections::HashMap`.
///
/// This macro uses `count_args!` to preallocate the exact amount of memory
/// needed, so it's more efficient than simply iteratively inserting.
///
/// ```
/// #[macro_use] extern crate colmac;
///
/// use std::collections::HashMap;
///
/// // create an empty one
/// let empty: HashMap<u64, u64> = hashmap![];
/// assert_eq!(0, empty.len());
///
/// // literal initialization
/// let mut map_a = HashMap::new();
/// map_a.insert("a", 123);
/// map_a.insert("b", 456);
///
/// let map_b = hashmap!["a" => 123, "b" => 456];
/// assert_eq!(map_a, map_b);
/// ```
#[macro_export]
macro_rules! hashmap {
    () => {
        HashMap::new()
    };
    ( $( $key:expr => $value:expr ),* ) => {{
        let size = count_args!( $( $key ),* );
        let mut map = HashMap::with_capacity(size);
        $(
            map.insert($key, $value);
        )*
        map
    }};
}

/// Just like `vec!`, but for `std::collections::HashSet`.
///
/// This macro uses `count_args!` to preallocate the exact amount of memory
/// needed, so it's more efficient than simply iteratively inserting.
///
/// ```
/// #[macro_use] extern crate colmac;
///
/// use std::collections::HashSet;
///
/// // create an empty one
/// let empty: HashSet<u64> = hashset![];
/// assert_eq!(0, empty.len());
///
/// // literal initialization
/// let mut set_a = HashSet::new();
/// set_a.insert(123);
/// set_a.insert(456);
///
/// let set_b = hashset!(123, 456);
/// assert_eq!(set_a, set_b);
/// ```
#[macro_export]
macro_rules! hashset {
    () => {
        HashSet::new()
    };
    ( $( $elem:expr ),* ) => {{
        let size = count_args!( $($elem),* );
        let mut set = HashSet::with_capacity(size);
        $(
            set.insert($elem);
        )*
        set
    }};
}

/// Sorts the input collection that impl's the trait `std::ops::IndexMut`.
///
/// There are two ways to invoke this macro:
/// 1. with one argument, a mutable collection
///     1. uses [`slice::sort_unstable`](https://doc.rust-lang.org/std/primitive.slice.html#method.sort_unstable) to sort
/// 1. with two arguments, a mutable collection followed by a closure
///     1. passes the closure to [`slice::sort_unstable_by`](https://doc.rust-lang.org/std/primitive.slice.html#method.sort_unstable_by) to sort
///
/// # Examples
///
/// ```
/// #[macro_use] extern crate colmac;
/// use std::cmp::Ordering::{Equal, Greater, Less};
///
/// // sort without a custom closure
/// let mut v1 = vec![2, 4, -1];
/// sort!(v1);
/// assert_eq!(vec![-1, 2, 4], v1);
///
/// // sort with; sort in reverse order
/// let mut v2 = vec![2, 4, -1];
/// sort!(v2, |a, b| match a.cmp(b) {
///     Less => Greater,
///     Greater => Less,
///     Equal => Equal,
/// });
/// assert_eq!(vec![4, 2, -1], v2);
/// ```
#[macro_export]
macro_rules! sort {
    ( $collection:expr ) => {
        (&mut $collection[..]).sort_unstable();
    };
    ( $collection:expr, $compare_fn:expr ) => {
        (&mut $collection[..]).sort_unstable_by($compare_fn);
    };
}

/// Creates a sorted `Vec` with cloned elements of the input collection, leaving the original
/// collection untouched.
///
/// The input collection should support `.iter()` method that returns an `Iterator` over its
/// elemnts.
///
/// There are two ways to invoke this macro:
/// 1. with one argument, a mutable collection
///     1. uses [`slice::sort_unstable`](https://doc.rust-lang.org/std/primitive.slice.html#method.sort_unstable) to sort
/// 1. with two arguments, a mutable collection followed by a closure
///     1. passes the closure to [`slice::sort_unstable_by`](https://doc.rust-lang.org/std/primitive.slice.html#method.sort_unstable_by) to sort
///
/// # Examples
///
/// ```
/// #[macro_use] extern crate colmac;
/// use std::cmp::Ordering::{Equal, Greater, Less};
///
/// // sort without a custom closure
/// let v1 = vec![2, 4, -1];
/// let v1_sorted = sorted!(v1);
/// assert_eq!(vec![2, 4, -1], v1);  // v1 is not modified
/// assert_eq!(vec![-1, 2, 4], v1_sorted);
///
/// // sort with; sort in reverse order
/// let v2 = vec![2, 4, -1];
/// let v2_sorted = sorted!(v2, |a, b| match a.cmp(b) {
///     Less => Greater,
///     Greater => Less,
///     Equal => Equal,
/// });
/// assert_eq!(vec![2, 4, -1], v2);  // v2 is not modified
/// assert_eq!(vec![4, 2, -1], v2_sorted);
/// ```
#[macro_export]
macro_rules! sorted {
    ( $collection:expr ) => {{
        let mut clones: Vec<_> = $collection.iter().cloned().collect();
        sort!(clones);
        clones
    }};
    ( $collection:expr, $compare_fn:expr ) => {{
        let mut clones: Vec<_> = $collection.iter().cloned().collect();
        sort!(clones, $compare_fn);
        clones
    }};
}

/// Creates a sorted `Vec<f64>` from the input.
///
/// This is a syntactic sugar for calling `sorted!` with the closure
/// `|a, b| a.partial_cmp(b).unwrap()`.
///
/// # Examples
///
/// ```
/// #[macro_use] extern crate colmac;
/// # use std::cmp::Ordering::{Equal, Greater, Less};
///
/// // sort some collection that impl's `IntoIterator`
/// let vec = vec![2.0, 4.0, -1.0];
///
/// let sorted_vec = sorted_f64!(vec);
/// let expected = vec![-1.0, 2.0, 4.0];
///
/// assert_eq!(expected, sorted_vec);
/// ```
#[macro_export]
macro_rules! sorted_f64 {
    ( $collection:expr ) => {
        sorted!($collection, |a, b| a.partial_cmp(b).unwrap())
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering::{Equal, Greater, Less};

    mod count_args {
        use super::*;

        #[test]
        fn zero() {
            let expected = 0;
            let result = count_args!();
            assert_eq!(expected, result);
        }
        #[test]
        fn one() {
            let expected = 1;
            let result = count_args!(10);
            assert_eq!(expected, result);
        }
        #[test]
        fn many() {
            let expected = 4;
            let result = count_args!(10, 20, 30, 40);
            assert_eq!(expected, result);
        }
    }

    mod hashmap {
        use super::*;

        #[test]
        fn zero() {
            let expected: HashMap<usize, usize> = HashMap::new();
            let result: HashMap<usize, usize> = hashmap!();
            assert_eq!(expected, result);
        }
        #[test]
        fn one() {
            let key = "abcde";
            let expected: HashMap<String, usize> =
                [(string!(key), 3usize)].iter().cloned().collect();
            let result: HashMap<String, usize> = hashmap!(string!(key) => 3usize);
            assert_eq!(expected, result);
        }
        #[test]
        fn many() {
            let expected: HashMap<String, usize> = vec![
                (string!("a"), 10usize),
                (string!("ab"), 20usize),
                (string!("abc"), 30usize),
            ]
            .into_iter()
            .collect();
            let result = hashmap!(
                string!("a") => 10usize,
                string!("ab") => 20usize,
                string!("abc") => 30usize
            );
            assert_eq!(expected, result);
        }
    }

    mod hashset {
        use super::*;

        #[test]
        fn zero() {
            let expected: HashSet<usize> = HashSet::new();
            let result: HashSet<usize> = hashset!();
            assert_eq!(expected, result);
        }
        #[test]
        fn one() {
            let name = string!("Jack");
            let expected: HashSet<String> = vec![&name].into_iter().cloned().collect();
            let result: HashSet<String> = hashset!(name);
            assert_eq!(expected, result);
        }
        #[test]
        fn many() {
            let expected: HashSet<&str> = vec!["a", "b", "c"].into_iter().collect();
            let result: HashSet<&str> = hashset!("a", "b", "c");
            assert_eq!(expected, result);
        }
    }

    mod sort {
        use super::*;

        #[test]
        fn vec() {
            let expected = vec![-14, -1, 0, 2, 3, 4, 8];

            let mut v = vec![4, 2, 3, -1, -14, 0, 8];
            sort!(v);
            assert_eq!(expected, v);
        }
        #[test]
        fn array() {
            let expected = vec![-14, -1, 0, 2, 3, 4, 8];

            let mut v = [4, 2, 3, -1, -14, 0, 8];
            sort!(v);
            assert_eq!(expected, v);
        }
        #[test]
        fn vec_reverse_sort() {
            let expected = vec![8, 4, 3, 2, 0, -1, -14];

            let mut v = vec![4, 2, 3, -1, -14, 0, 8];
            sort!(v, |a, b| match a.cmp(b) {
                Less => Greater,
                Greater => Less,
                Equal => Equal,
            });
            assert_eq!(expected, v);
        }
    }

    mod sorted {
        use super::*;

        #[test]
        fn vec() {
            let expected = vec![-14, -1, 0, 2, 3, 4, 8];

            let v = vec![4, 2, 3, -1, -14, 0, 8];
            let result = sorted!(v);
            assert_eq!(expected, result);
            assert_eq!(vec![-14, -1, 0, 2, 3, 4, 8], expected); // unmodified
        }
        #[test]
        fn hashset() {
            let expected = vec![-14, -1, 0, 2, 3, 4, 8];

            let v = hashset![4, 2, 3, -1, -14, 0, 8];
            let result = sorted!(v);
            assert_eq!(expected, result);
        }
        #[test]
        fn array() {
            let expected = vec![-14, -1, 0, 2, 3, 4, 8];

            let v = [4, 2, 3, -1, -14, 0, 8];
            let result = sorted!(v);
            assert_eq!(expected, result);
        }
        #[test]
        fn vec_reverse_sort() {
            let expected = vec![8, 4, 3, 2, 0, -1, -14];

            let v = vec![4, 2, 3, -1, -14, 0, 8];
            let result = sorted!(v, |a, b| match a.cmp(b) {
                Less => Greater,
                Greater => Less,
                Equal => Equal,
            });
            assert_eq!(expected, result);
            assert_eq!(vec![8, 4, 3, 2, 0, -1, -14], expected); // unmodified
        }
    }

    mod sorted_f64 {
        use super::*;

        #[test]
        fn vec() {
            let expected = vec![-14.0, -1.0, 0.0, 2.0, 3.0, 4.0, 8.0];

            let v = vec![4.0, 2.0, 3.0, -1.0, -14.0, 0.0, 8.0];
            let result = sorted_f64!(v);
            assert_eq!(expected, result);
        }
        #[test]
        fn array() {
            let expected = vec![-14.0, -1.0, 0.0, 2.0, 3.0, 4.0, 8.0];

            let v = [4.0, 2.0, 3.0, -1.0, -14.0, 0.0, 8.0];
            let result = sorted_f64!(v);
            assert_eq!(expected, result);
        }
    }
}
