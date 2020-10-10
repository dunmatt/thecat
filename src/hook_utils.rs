//! This module is a collection of utilities that aid in working with Hooks.
//!

#[macro_export]
/// This macro helps implement `Hook`s that contain collections of `Hook`s (for example bars).
/// If a given `Hook` doesn't need to tie into a given method you can use this macro to create
/// a passthrough method that dispatches the method call to each contained `Hook`.
macro_rules! pass_through_method_to {
    (fn $f:ident ( $(&$mt:ident $slf:ident,)? $($arg:ident : $tipe:ty),+ ) |=> $iter:expr) => {
        fn $f ( $(&$mt $slf,)? $($arg:$tipe),+ ) {
            for hook in $iter {
                hook.$f ($($arg),+);
            }
        }
    };
    (fn $f:ident ( $(&$slf:ident,)? $($arg:ident : $tipe:ty),+ ) |=> $iter:expr) => {
        fn $f ( $(&$slf,)? $($arg:$tipe),+ ) {
            for hook in $iter {
                hook.$f ($($arg),+);
            }
        }
    };
}
