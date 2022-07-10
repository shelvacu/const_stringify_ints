#![no_std]

//! This library solves a very specific problem: You have a (possibly generic) constant integer, and you want it as a `&'static str`.
//!
//! All methods are named `const_str_` and then the type.
//!
//! ```
//! use const_stringify_ints::*;
//!
//! const SIZE_OF_MY_ARR:usize = 294;
//! const SIZE_STR:&'static str = const_str_usize::<SIZE_OF_MY_ARR>();
//!
//! assert_eq!(SIZE_OF_MY_ARR.to_string().as_str(), SIZE_STR);
//! ```
//!
//! # Support
//!
//! This library supports `no_std` environments.
//!
//! This library should in theory support targets with a 16-bit, however I don't have a way of testing this. Additionaly, these targets are likely very memory-constrained, and this crate always 'allocates' 5 bytes of static memory for a `u16` even when only 1 byte is needed.

use core::str;
#[cfg(test)]
extern crate alloc;
#[cfg(test)]
use alloc::string::ToString;

macro_rules! impl_everything {
    (
        $ty_unsigned: ident,
        $ty_signed: ident,
        $width: expr,
        $bits: literal,
        $method_unsigned: ident,
        $method_signed: ident,
        $str_method_unsigned: ident,
        $str_method_signed: ident,
        {
            $($n_unsized: literal,)*
        },
        {
            $($n: literal,)*
        },
    ) => {
        const fn $method_unsigned<const N: $ty_unsigned>() -> &'static [u8; { ($( (($n_unsized - $n_unsized) + 1) + )* $( (($n as $ty_unsigned) - $n + 1) + )* 0) as usize }] {
            &[
                $(
                    (N / $n_unsized % 10) as u8 + b'0',
                )*
                $(
                    (N / $n % 10) as u8 + b'0',
                )*
            ]
        }

        #[cfg(target_pointer_width = $width)]
        const fn const_digits_usize<const N: usize>() -> &'static [u8; { $( (($n_unsized - $n_unsized) + 1) + )* $( ($n - $n + 1) + )* 0 }] {
            &[
                $(
                    (N / $n_unsized % 10) as u8 + b'0',
                )*
                $(
                    (N / $n % 10) as u8 + b'0',
                )*
            ]
        }

        const fn $method_signed<const N: $ty_signed>() -> &'static [u8; { ($( ((($n as $ty_unsigned) - $n) + 1) + )* 1) as usize }] {
            &[
                numeric_if!(
                    if [(N >> $bits) as u8] {
                        b'-'
                    } else {
                        b'0'
                    }
                ),
                $(
                    numeric_if!(
                        if [(N >> $bits) as u8] {
                            boolific_if!(
                                if [N > -$n]:u8 {
                                    b'-'
                                } else {
                                    (absolute!((N), $ty_unsigned) / $n % 10) as u8 + b'0'
                                }
                            )
                        } else {
                            boolific_if!(
                                if [N < $n]:u8 {
                                    b'0'
                                } else {
                                    (absolute!((N), $ty_unsigned) / $n % 10) as u8 + b'0'
                                }
                            )
                        }
                    ),
                )*
            ]
        }

        #[cfg(target_pointer_width = $width)]
        const fn const_digits_isize<const N: isize>() -> &'static [u8; { $( (($n - $n) + 1) + )* 1 }] {
            &[
                numeric_if!(
                    if [(N >> $bits) as u8] {
                        b'-'
                    } else {
                        b'0'
                    }
                ),
                $(
                    numeric_if!(
                        if [(N >> $bits) as u8] {
                            boolific_if!(
                                if [N > -$n]:u8 {
                                    b'-'
                                } else {
                                    (absolute!((N), $ty_unsigned) / $n % 10) as u8 + b'0'
                                }
                            )
                        } else {
                            boolific_if!(
                                if [N < $n]:u8 {
                                    b'0'
                                } else {
                                    (absolute!((N), $ty_unsigned) / $n % 10) as u8 + b'0'
                                }
                            )
                        }
                    ),
                )*
            ]
        }

        /// Returns the const parameter `N` as a `&'static str`.
        ///
        /// ```
        /// # use const_stringify_ints::*;
        #[doc=concat!("const MY_AWESOME_NUMBER:", stringify!($ty_unsigned), " = 123;")]
        ///
        #[doc=concat!("assert_eq!(\"123\",", stringify!($str_method_unsigned) ,"::<MY_AWESOME_NUMBER>());")]
        /// ```
        pub const fn $str_method_unsigned<const N: $ty_unsigned>() -> &'static str {
            if N == 0 {
                return "0";
            }
            let digits = $method_unsigned::<N>();
            let mut digits = digits as &[_];
            while digits.len() > 0 && digits[0] == b'0' {
                let (_, x) = match digits.split_first() {
                    Some(x) => x,
                    None => panic!(),
                };
                digits = x;
            }
            unsafe { str::from_utf8_unchecked(digits) }
        }

        /// Returns the const parameter `N` as a `&'static str`.
        ///
        /// ```
        /// # use const_stringify_ints::*;
        #[doc=concat!("const MY_AWESOME_NUMBER:", stringify!($ty_signed), " = -123;")]
        ///
        #[doc=concat!("assert_eq!(\"-123\",", stringify!($str_method_signed) ,"::<MY_AWESOME_NUMBER>());")]
        /// ```
        pub const fn $str_method_signed<const N: $ty_signed>() -> &'static str {
            if N == 0 {
                return "0";
            }
            let digits = $method_signed::<N>();
            let mut digits = digits as &[_];
            if N >= 0 {
                while digits.len() > 0 && digits[0] == b'0' {
                    let (_, x) = match digits.split_first() {
                        Some(x) => x,
                        None => panic!(),
                    };
                    digits = x;
                }
            } else {
                while digits.len() > 1 && digits[1] == b'-' {
                    let (_, x) = match digits.split_first() {
                        Some(x) => x,
                        None => panic!(),
                    };
                    digits = x;
                }
            }
            unsafe { str::from_utf8_unchecked(digits) }
        }
    }
}

macro_rules! boolific_if {
    (
        if [$($cond:tt)+]:$t:tt {
            $($if:tt)+
        } else {
            $($else:tt)+
        }
    ) => {
        numeric_if!(
            if [nonzero!((($($cond)+) as $t), $t)] { $($if)+ } else { $($else)+ }
        )
    }
}

macro_rules! numeric_if {
    (
        if [$($cond:tt)+] { $($if:tt)+ } else { $($else:tt)+ }
    ) => {
        (
            ($($cond)+) & ($($if)+)
        ) | (
            !($($cond)+) & ($($else)+)
        )
    }
}

macro_rules! absolute {
    ($a:tt, u128) => {
        numeric_if!(if [($a >> 127) as u128] {
            wrapping_inc!((!($a as u128)))
        } else {
            $a as u128
        })
    };
    ($a:tt, u64) => {
        numeric_if!(if [($a >> 63) as u64] {
            wrapping_inc!((!($a as u64)))
        } else {
            $a as u64
        })
    };
    ($a:tt, u32) => {
        numeric_if!(if [($a >> 31) as u32] {
            wrapping_inc!((!($a as u32)))
        } else {
            $a as u32
        })
    };
    ($a:tt, u16) => {
        numeric_if!(if [($a >> 15) as u16] {
            wrapping_inc!((!($a as u16)))
        } else {
            $a as u16
        })
    };
    ($a:tt, u8) => {
        numeric_if!(if [($a >> 7) as u8] {
            wrapping_inc!((!($a as u8)))
        } else {
            $a as u8
        })
    };
}

macro_rules! wrapping_inc {
    ($a:tt) => {
        (((($a >> 1) + ($a & 1)) << 1) + (!$a & 1))
    };
}

macro_rules! nonzero {
    ( $a:tt, u8 ) => {
        $a | $a << 1 | $a << 2 | $a << 3 | $a << 4 | $a << 5 | $a << 6 | $a << 7
    };
    ( $a:tt, u16 ) => {
        nonzero!($a, u8) | nonzero!(($a << 8), u8)
    };
    ( $a:tt, u32 ) => {
        nonzero!($a, u16) | nonzero!(($a << 16), u16)
    };
    ( $a:tt, u64 ) => {
        nonzero!($a, u32) | nonzero!(($a << 32), u32)
    };
    ( $a:tt, u128 ) => {
        nonzero!($a, u64) | nonzero!(($a << 32), u64)
    };
}

#[cfg(test)]
mod macro_tests {

    #[test]
    fn test_wrapping_inc() {
        assert_eq!(wrapping_inc!(0u8), 1u8,);
        assert_eq!(wrapping_inc!(1u8), 2u8,);
        assert_eq!(wrapping_inc!(254u8), 255u8,);
        assert_eq!(wrapping_inc!(255u8), 0u8,);
    }

    #[cfg(target_pointer_width = "64")]
    #[test]
    fn test_abs() {
        assert_eq!(absolute!(0isize, u64), 0);
        assert_eq!(absolute!((-1isize), u64), 1);
        assert_eq!(absolute!((-128isize), u64), 128);
        assert_eq!(absolute!((isize::MIN), u64), i64::MIN.unsigned_abs());
    }

    #[cfg(target_pointer_width = "32")]
    #[test]
    fn test_abs() {
        assert_eq!(absolute!(0isize, u32), 0);
        assert_eq!(absolute!((-1isize), u32), 1);
        assert_eq!(absolute!((-128isize), u32), 128);
        assert_eq!(absolute!((isize::MIN), u32), i32::MIN.unsigned_abs());
    }

    #[test]
    fn test_numeric_if() {
        assert_eq!(
            numeric_if! {
                if [255u8] { 1 } else { 2 }
            },
            1
        );
        assert_eq!(
            numeric_if! {
                if [0u8] { 1 } else { 2 }
            },
            2
        );
    }

    #[test]
    fn test_nonzero() {
        assert_eq!(nonzero!(0u8, u8), 0);
        assert_eq!(nonzero!(1u8, u8), 255);
        assert_eq!(nonzero!(0i8, u8), 0);
        assert_eq!(nonzero!(5i8, u8), -1);
        assert_eq!(nonzero!(5i16, u16), -1);
        assert_eq!(nonzero!(0i16, u16), 0);
        assert_eq!(nonzero!(5i64, u64), -1);
        assert_eq!(nonzero!(0i64, u64), 0);
        assert_eq!(nonzero!(0u64, u64), 0);
        assert_eq!(nonzero!(1u64, u64), u64::MAX);
    }
}

impl_everything! { u8, i8, "8", 7, const_digits_u8, const_digits_i8, const_str_u8, const_str_i8, {}, {
    100,
    10,
    1,
},}
impl_everything! { u16, i16, "16", 15, const_digits_u16, const_digits_i16, const_str_u16, const_str_i16, {}, {
    10000,
    1000,
    100,
    10,
    1,
},}
impl_everything! { u32, i32, "32", 31, const_digits_u32, const_digits_i32, const_str_u32, const_str_i32, {}, {
    1000000000,
    100000000,
    10000000,
    1000000,
    100000,
    10000,
    1000,
    100,
    10,
    1,
},}
impl_everything! { u64, i64, "64", 63, const_digits_u64, const_digits_i64, const_str_u64, const_str_i64, {
    10000000000000000000,
}, {
    1000000000000000000,
    100000000000000000,
    10000000000000000,
    1000000000000000,
    100000000000000,
    10000000000000,
    1000000000000,
    100000000000,
    10000000000,
    1000000000,
    100000000,
    10000000,
    1000000,
    100000,
    10000,
    1000,
    100,
    10,
    1,
},}
impl_everything! { u128, i128, "128", 127, const_digits_u128, const_digits_i128, const_str_u128, const_str_i128, {}, {
    100000000000000000000000000000000000000,
    10000000000000000000000000000000000000,
    1000000000000000000000000000000000000,
    100000000000000000000000000000000000,
    10000000000000000000000000000000000,
    1000000000000000000000000000000000,
    100000000000000000000000000000000,
    10000000000000000000000000000000,
    1000000000000000000000000000000,
    100000000000000000000000000000,
    10000000000000000000000000000,
    1000000000000000000000000000,
    100000000000000000000000000,
    10000000000000000000000000,
    1000000000000000000000000,
    100000000000000000000000,
    10000000000000000000000,
    1000000000000000000000,
    100000000000000000000,
    10000000000000000000,
    1000000000000000000,
    100000000000000000,
    10000000000000000,
    1000000000000000,
    100000000000000,
    10000000000000,
    1000000000000,
    100000000000,
    10000000000,
    1000000000,
    100000000,
    10000000,
    1000000,
    100000,
    10000,
    1000,
    100,
    10,
    1,
},}

/// Returns the const parameter `N` as a `&'static str`.
///
/// ```
/// # use const_stringify_ints::*;
/// const MY_ARRAY_SIZE:usize = 321;
///
/// assert_eq!("321", const_str_usize::<MY_ARRAY_SIZE>());
/// ```
pub const fn const_str_usize<const N: usize>() -> &'static str {
    if N == 0 {
        return "0";
    }
    let digits = const_digits_usize::<N>();
    let mut digits = digits as &[_];
    while digits.len() > 0 && digits[0] == b'0' {
        let (_, x) = match digits.split_first() {
            Some(x) => x,
            None => panic!(),
        };
        digits = x;
    }
    unsafe { str::from_utf8_unchecked(digits) }
}

/// Returns the const parameter `N` as a `&'static str`.
///
/// ```
/// # use const_stringify_ints::*;
/// const MY_NEAT_INT:isize = -420;
///
/// assert_eq!("-420", const_str_isize::<MY_NEAT_INT>());
/// ```
pub const fn const_str_isize<const N: isize>() -> &'static str {
    if N == 0 {
        return "0";
    }
    let digits = const_digits_isize::<N>();
    let mut digits = digits as &[_];
    if N >= 0 {
        while digits.len() > 0 && digits[0] == b'0' {
            let (_, x) = match digits.split_first() {
                Some(x) => x,
                None => panic!(),
            };
            digits = x;
        }
    } else {
        while digits.len() > 1 && digits[1] == b'-' {
            let (_, x) = match digits.split_first() {
                Some(x) => x,
                None => panic!(),
            };
            digits = x;
        }
    }
    unsafe { str::from_utf8_unchecked(digits) }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!("0", const_str_usize::<0>());
        assert_eq!("1", const_str_usize::<1>());
        assert_eq!("128", const_str_usize::<128>());
        assert_eq!("99", const_str_usize::<99>());
        assert_eq!(
            usize::MAX.to_string().as_str(),
            const_str_usize::<{ usize::MAX }>()
        );

        assert_eq!("-1", const_str_isize::<-1>());
        assert_eq!("-128", const_str_isize::<-128>());
        assert_eq!("-256", const_str_isize::<-256>());
        assert_eq!(
            isize::MIN.to_string().as_str(),
            const_str_isize::<{ isize::MIN }>()
        );
        assert_eq!("1", const_str_isize::<1>());
        assert_eq!("2", const_str_isize::<2>());
        assert_eq!("10", const_str_isize::<10>());
        assert_eq!(
            isize::MAX.to_string().as_str(),
            const_str_isize::<{ isize::MAX }>()
        );

        assert_eq!("0", const_str_u16::<0>());
        assert_eq!("1", const_str_u16::<1>());
        assert_eq!("1000", const_str_u16::<1000>());
        assert_eq!("9999", const_str_u16::<9999>());
        assert_eq!("65535", const_str_u16::<65535>());

        assert_eq!("0", const_str_i16::<0>());
        assert_eq!("1", const_str_i16::<1>());
        assert_eq!("-1", const_str_i16::<-1>());
        assert_eq!("1000", const_str_i16::<1000>());
        assert_eq!("-32768", const_str_i16::<-32768>());
        assert_eq!("32767", const_str_i16::<32767>());
    }
}
