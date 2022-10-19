#[macro_export]
macro_rules! bin_struct {
    {
        #[derive($($derives:tt)*)]
        pub struct $name:ident {
            $(pub $field:ident: $numtype:ty,)*
        }
    } => {
        #[derive($($derives)*)]
        pub struct $name {
            $(pub $field: $numtype,)*
        }
        
        impl $name {
            pub const SIZE: usize = std::mem::size_of::<Self>();

            pub fn from_bytes(raw: [u8; Self::SIZE]) -> Self {
                let mut offset: usize = 0;
                $(
                    let _size: usize = std::mem::size_of::<$numtype>();
                    let $field = <$numtype>::from_be_bytes(raw[offset..(offset + _size)].try_into().unwrap());
                    offset += _size;
                )*
                assert_eq!(offset, Self::SIZE);
                Self { $($field,)* }
            }

            pub fn to_bytes(&self) -> [u8; Self::SIZE] {
                let mut buf = [0u8; Self::SIZE];
                let mut offset: usize = 0;
                $(
                    let _size: usize = std::mem::size_of::<$numtype>();
                    (&mut buf[offset..(offset + _size)]).copy_from_slice(self.$field.to_be_bytes().as_slice());
                    offset += _size;
                )*
                assert_eq!(offset, Self::SIZE);
                buf
            }
        }
    };
}

#[macro_export]
macro_rules! error_enum {
    {
        #[derive($($derives:tt)*)]
        $name:ident {
            $($extra:tt)*
        }
        convert {
            $($variant:ident => $error:ty),*,
        }
    } => {
        #[derive($($derives)*)]
        pub enum $name {
            $($variant($error),)*
            $($extra)*
        }

        $(
            impl From<$error> for $name {
                fn from(err: $error) -> $name {
                    <$name>::$variant(err)
                }
            }
        )*
    };
}

// Copied from https://github.com/FaultyRAM/concat-string@1.0.1
// Copyright (c) 2017-2018 FaultyRAM. Licensed under the Apache License, Version 2.0 or the MIT license.
#[macro_export]
macro_rules! concat_string {
    () => { String::with_capacity(0) };
    ($($s:expr),+) => {{
        use std::ops::AddAssign;
        let mut len = 0;
        $(len.add_assign(AsRef::<str>::as_ref(&$s).len());)+
        let mut buf = String::with_capacity(len);
        $(buf.push_str($s.as_ref());)+
        buf
    }};
}
