use std::str;

// const fn const_digits_u128<const N: u128>() -> &'static [u8; 39] {
//     &[
//         (N / (10u128.pow(38)) % 10) as u8 + b'0',
//         (N / (10u128.pow(37)) % 10) as u8 + b'0',
//         (N / (10u128.pow(36)) % 10) as u8 + b'0',
//         (N / (10u128.pow(35)) % 10) as u8 + b'0',
//         (N / (10u128.pow(34)) % 10) as u8 + b'0',
//         (N / (10u128.pow(33)) % 10) as u8 + b'0',
//         (N / (10u128.pow(32)) % 10) as u8 + b'0',
//         (N / (10u128.pow(31)) % 10) as u8 + b'0',
//         (N / (10u128.pow(30)) % 10) as u8 + b'0',
//         (N / (10u128.pow(29)) % 10) as u8 + b'0',
//         (N / (10u128.pow(28)) % 10) as u8 + b'0',
//         (N / (10u128.pow(27)) % 10) as u8 + b'0',
//         (N / (10u128.pow(26)) % 10) as u8 + b'0',
//         (N / (10u128.pow(25)) % 10) as u8 + b'0',
//         (N / (10u128.pow(24)) % 10) as u8 + b'0',
//         (N / (10u128.pow(23)) % 10) as u8 + b'0',
//         (N / (10u128.pow(22)) % 10) as u8 + b'0',
//         (N / (10u128.pow(21)) % 10) as u8 + b'0',
//         (N / (10u128.pow(20)) % 10) as u8 + b'0',
//         (N / (10u128.pow(19)) % 10) as u8 + b'0',
//         (N / (10u128.pow(18)) % 10) as u8 + b'0',
//         (N / (10u128.pow(17)) % 10) as u8 + b'0',
//         (N / (10u128.pow(16)) % 10) as u8 + b'0',
//         (N / (10u128.pow(15)) % 10) as u8 + b'0',
//         (N / (10u128.pow(14)) % 10) as u8 + b'0',
//         (N / (10u128.pow(13)) % 10) as u8 + b'0',
//         (N / (10u128.pow(12)) % 10) as u8 + b'0',
//         (N / (10u128.pow(11)) % 10) as u8 + b'0',
//         (N / (10u128.pow(10)) % 10) as u8 + b'0',
//         (N / (10u128.pow( 9)) % 10) as u8 + b'0',
//         (N / (10u128.pow( 8)) % 10) as u8 + b'0',
//         (N / (10u128.pow( 7)) % 10) as u8 + b'0',
//         (N / (10u128.pow( 6)) % 10) as u8 + b'0',
//         (N / (10u128.pow( 5)) % 10) as u8 + b'0',
//         (N / (10u128.pow( 4)) % 10) as u8 + b'0',
//         (N / (10u128.pow( 3)) % 10) as u8 + b'0',
//         (N / (10u128.pow( 2)) % 10) as u8 + b'0',
//         (N / (10u128.pow( 1)) % 10) as u8 + b'0',
//         (N / (10u128.pow( 0)) % 10) as u8 + b'0',
//     ]
// }

// const fn const_digits_u64<const N: u64>() -> &'static [u8; 20] {
//     &[
//         (N / (10u64.pow(18)) % 10) as u8 + b'0',
//         (N / (10u64.pow(19)) % 10) as u8 + b'0',
//         (N / (10u64.pow(17)) % 10) as u8 + b'0',
//         (N / (10u64.pow(16)) % 10) as u8 + b'0',
//         (N / (10u64.pow(15)) % 10) as u8 + b'0',
//         (N / (10u64.pow(14)) % 10) as u8 + b'0',
//         (N / (10u64.pow(13)) % 10) as u8 + b'0',
//         (N / (10u64.pow(12)) % 10) as u8 + b'0',
//         (N / (10u64.pow(11)) % 10) as u8 + b'0',
//         (N / (10u64.pow(10)) % 10) as u8 + b'0',
//         (N / (10u64.pow( 9)) % 10) as u8 + b'0',
//         (N / (10u64.pow( 8)) % 10) as u8 + b'0',
//         (N / (10u64.pow( 7)) % 10) as u8 + b'0',
//         (N / (10u64.pow( 6)) % 10) as u8 + b'0',
//         (N / (10u64.pow( 5)) % 10) as u8 + b'0',
//         (N / (10u64.pow( 4)) % 10) as u8 + b'0',
//         (N / (10u64.pow( 3)) % 10) as u8 + b'0',
//         (N / (10u64.pow( 2)) % 10) as u8 + b'0',
//         (N / (10u64.pow( 1)) % 10) as u8 + b'0',
//         (N / (10u64.pow( 0)) % 10) as u8 + b'0',
//     ]
// }

// const fn const_digits_u32<const N: u32>() -> &'static [u8; 10] {
//     &[
//         (N / (10u32.pow( 9)) % 10) as u8 + b'0',
//         (N / (10u32.pow( 8)) % 10) as u8 + b'0',
//         (N / (10u32.pow( 7)) % 10) as u8 + b'0',
//         (N / (10u32.pow( 6)) % 10) as u8 + b'0',
//         (N / (10u32.pow( 5)) % 10) as u8 + b'0',
//         (N / (10u32.pow( 4)) % 10) as u8 + b'0',
//         (N / (10u32.pow( 3)) % 10) as u8 + b'0',
//         (N / (10u32.pow( 2)) % 10) as u8 + b'0',
//         (N / (10u32.pow( 1)) % 10) as u8 + b'0',
//         (N / (10u32.pow( 0)) % 10) as u8 + b'0',
//     ]
// }

// const fn const_digits_u16<const N: u16>() -> &'static [u8; 5] {
//     &[
//         (N / (10u16.pow( 4)) % 10) as u8 + b'0',
//         (N / (10u16.pow( 3)) % 10) as u8 + b'0',
//         (N / (10u16.pow( 2)) % 10) as u8 + b'0',
//         (N / (10u16.pow( 1)) % 10) as u8 + b'0',
//         (N / (10u16.pow( 0)) % 10) as u8 + b'0',
//     ]
// }

// #[cfg(target_pointer_width = "128")]
// const fn const_digits<const N: usize>() -> &'static [u8; 39] {
//     const_digits_u128::<{N as u128}>()
// }

// #[cfg(target_pointer_width = "64")]
// const fn const_digits<const N: usize>() -> &'static [u8; 20] {
//     const_digits_u64::<{N as u64}>()
// }

// #[cfg(target_pointer_width = "32")]
// const fn const_digits<const N: usize>() -> &'static [u8; 10] {
//     const_digits_u32::<{N as u32}>()
// }

// #[cfg(target_pointer_width = "16")]
// const fn const_digits<const N: usize>() -> &'static [u8; 10] {
//     const_digits_u16::<{N as u16}>()
// }

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
        const fn $method_unsigned<const N: $ty_unsigned>() -> &'static [u8; { $( (($n_unsized - $n_unsized) + 1) + )* $( ($n - $n + 1) + )* 0 }] {
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

        const fn $method_signed<const N: $ty_signed>() -> &'static [u8; { $( (($n - $n) + 1) + )* 1 }] {
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
                // if N < 0 { b'-' } else { b'0' },
                // $(
                //     if $n > 0 && (N / $n % 10) == 0 {
                //         b'-'
                //     } else {
                //         (N / $n % 10) as u8 + b'0'
                //     },
                // )*
                numeric_if!(
                    if [(N >> $bits) as u8] {
                        b'-'
                    } else {
                        b'0'
                    }
                ),
                //(((N >> $bits) as u8) & b'-') | ((!(N >> $bits) as u8) & b'0'),
                $(
                    numeric_if!(
                        if [(N >> $bits) as u8] {
                            boolific_if!(
                                // if [absolute!((N), $ty_unsigned) < $n]:u8 {
                                if [N > -$n]:u8 {
                                    b'-'
                                } else {
                                    (absolute!((N), $ty_unsigned) / $n % 10) as u8 + b'0'
                                }
                            )
                        } else {
                            boolific_if!(
                                // if [absolute!((N), $ty_unsigned) < $n]:u8 {
                                if [N < $n]:u8 {
                                    b'0'
                                } else {
                                    (absolute!((N), $ty_unsigned) / $n % 10) as u8 + b'0'
                                }
                            )
                        }
                    ),
                    //(((N >> $bits) as u8) & b'-') | ((!(N >> $bits) as u8) & ((N / $n % 10) as u8 + b'0')),
                )*
            ]
        }

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
        numeric_if!(
            if [($a >> 127) as u128] {
                wrapping_inc!((!($a as u128)))
            } else {
                $a as u128
            }
        )
    };
    ($a:tt, u64) => {
        numeric_if!(
            if [($a >> 63) as u64] {
                wrapping_inc!((!($a as u64)))
            } else {
                $a as u64
            }
        )
    };
    ($a:tt, u32) => {
        numeric_if!(
            if [($a >> 31) as u32] {
                wrapping_inc!((!($a as u32)))
            } else {
                $a as u32
            }
        )
    };
    ($a:tt, u16) => {
        numeric_if!(
            if [($a >> 15) as u16] {
                wrapping_inc!((!($a as u16)))
            } else {
                $a as u16
            }
        )
    };
    ($a:tt, u8) => {
        numeric_if!(
            if [($a >> 7) as u8] {
                wrapping_inc!((!($a as u8)))
            } else {
                $a as u8
            }
        )
    };
}

macro_rules! wrapping_inc {
    ($a:tt) => {
        (((($a >> 1) + ($a & 1)) << 1) + (!$a & 1))
    }
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

macro_rules! get_bit {
    ( $a:tt, $index:literal, $size:tt ) => {
        nonzero!(($a & 1 << $index), $size)
    };
}

// macro_rules! cmp {
//     ( $a:tt < $b:tt ) => {
//         $a ^ $b //where they differ
//         $a & !$b //1 where a > b
//         !$a & $b //1 where a < b

//     }
// }

#[cfg(test)]
mod macro_tests {
    use super::*;

    #[test]
    fn test_wrapping_inc() {
        assert_eq!(
            wrapping_inc!(0u8),
            1u8,
        );
        assert_eq!(
            wrapping_inc!(1u8),
            2u8,
        );
        assert_eq!(
            wrapping_inc!(254u8),
            255u8,
        );
        assert_eq!(
            wrapping_inc!(255u8),
            0u8,
        );
    }

    #[test]
    fn test_abs() {
        assert_eq!(
            absolute!(0isize, u64),
            0
        );
        assert_eq!(
            absolute!((-1isize), u64),
            1
        );
        assert_eq!(
            absolute!((-128isize), u64),
            128
        );
        assert_eq!(
            absolute!((i64::MIN as isize), u64),
            i64::MIN.unsigned_abs()
        );
    }

    #[test]
    fn test_numeric_if() {
        assert_eq!(
            numeric_if!{
                if [255u8] { 1 } else { 2 }
            },
            1
        );
        assert_eq!(
            numeric_if!{
                if [0u8] { 1 } else { 2 }
            },
            2
        );
    }

    #[test]
    fn test_nonzero() {
        assert_eq!(
            nonzero!(0u8, u8),
            0
        );
        assert_eq!(
            nonzero!(1u8, u8),
            255
        );
        assert_eq!(
            nonzero!(0i8, u8),
            0
        );
        assert_eq!(
            nonzero!(5i8, u8),
            -1
        );
        assert_eq!(
            nonzero!(5i16, u16),
            -1
        );
        assert_eq!(
            nonzero!(0i16, u16),
            0
        );
        assert_eq!(
            nonzero!(5i64, u64),
            -1
        );
        assert_eq!(
            nonzero!(0i64, u64),
            0
        );
        assert_eq!(
            nonzero!(0u64, u64),
            0
        );
        assert_eq!(
            nonzero!(1u64, u64),
            u64::MAX
        );
    }
}

const fn const_digits_ishsize<const N: isize>() -> &'static [u8; 2] {
    &[
        //(((N >> 63) as u8) & b'-') | ((!(N >> 63) as u8) & b'0'),
        numeric_if!(
            if [(N >> 63) as u8] { b'-' } else { b'0' | (N < 5) as u8 }
        ),
        // boolific_if!(
        //     if [N > 1 && 4 < 3]:u8 { b'*' } else { b'.' }
        // ),
        boolific_if!(
            if [N > 1]:u8 {
                boolific_if!(
                    if [3 < 4]:u8 {
                        b'*'
                    } else {
                        b'q'
                    }
                )
            } else {
                wrapping_inc!((255u8))
            }
        ),
    ]
}

impl_everything!{ u8, i8, "8", 7, const_digits_u8, const_digits_i8, const_str_u8, const_str_i8, {}, {
    100,
    10,
    1,
},}
impl_everything!{ u16, i16, "16", 15, const_digits_u16, const_digits_i16, const_str_u16, const_str_i16, {}, {
    10000,
    1000,
    100,
    10,
    1,
},}
impl_everything!{ u32, i32, "32", 31, const_digits_u32, const_digits_i32, const_str_u32, const_str_i32, {}, {
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
impl_everything!{ u64, i64, "64", 63, const_digits_u64, const_digits_i64, const_str_u64, const_str_i64, {
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
// impl_everything!{ u128, i128, "128", 127, const_digits_u128, const_digits_i128, const_str_u128, const_str_i128, {}, {
//     100000000000000000000000000000000000000,
//     10000000000000000000000000000000000000,
//     1000000000000000000000000000000000000,
//     100000000000000000000000000000000000,
//     10000000000000000000000000000000000,
//     1000000000000000000000000000000000,
//     100000000000000000000000000000000,
//     10000000000000000000000000000000,
//     1000000000000000000000000000000,
//     100000000000000000000000000000,
//     10000000000000000000000000000,
//     1000000000000000000000000000,
//     100000000000000000000000000,
//     10000000000000000000000000,
//     1000000000000000000000000,
//     100000000000000000000000,
//     10000000000000000000000,
//     1000000000000000000000,
//     100000000000000000000,
//     10000000000000000000,
//     1000000000000000000,
//     100000000000000000,
//     10000000000000000,
//     1000000000000000,
//     100000000000000,
//     10000000000000,
//     1000000000000,
//     100000000000,
//     10000000000,
//     1000000000,
//     100000000,
//     10000000,
//     1000000,
//     100000,
//     10000,
//     1000,
//     100,
//     10,
//     1,
// },}

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
        #[cfg(target_pointer_width = "64")]
        assert_eq!("18446744073709551615", const_str_usize::<18446744073709551615>());

        assert_eq!("-1", const_str_isize::< -1 >());
        assert_eq!("-128", const_str_isize::< -128 >());
        assert_eq!("-256", const_str_isize::< -256 >());
        #[cfg(target_pointer_width = "64")]
        assert_eq!("-9223372036854775808", const_str_isize::< -9223372036854775808 >());
        assert_eq!("1", const_str_isize::< 1 >());
        assert_eq!("2", const_str_isize::< 2 >());
        assert_eq!("10", const_str_isize::< 10 >());
        #[cfg(target_pointer_width = "64")]
        assert_eq!("9223372036854775807", const_str_isize::< 9223372036854775807 >());

        assert_eq!("0", const_str_u16::<0>());
        assert_eq!("1", const_str_u16::<1>());
        assert_eq!("1000", const_str_u16::<1000>());
        assert_eq!("65535", const_str_u16::<65535>());

        assert_eq!("0", const_str_i16::<0>());
        assert_eq!("1", const_str_i16::<1>());
        assert_eq!("-1", const_str_i16::<-1>());
        assert_eq!("1000", const_str_i16::<1000>());
        assert_eq!("-32768", const_str_i16::<-32768>());
        assert_eq!("32767", const_str_i16::<32767>());
    }
}
