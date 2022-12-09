#[macro_export]
macro_rules! bin_struct {
    {
        $(#[$meta:meta])*
        $pub:vis struct $struct_name:ident {
            $($field_pub:vis $field:ident: $ty:ty,)*
        }
    } => {
        $(#[$meta])*
        $pub struct $struct_name {
            $($field_pub $field: $ty,)*
        }

        impl $struct_name {
            $pub const SIZE: usize = { $(std::mem::size_of::<$ty>() + )* 0 };

            $pub fn from_bytes(bytes: [u8; Self::SIZE]) -> Self {
                let mut offset: usize = 0;
                $(let $field = {
                    let size: usize = std::mem::size_of::<$ty>();
                    let slice = bytes[offset..(offset + size)].try_into().unwrap();
                    let val = <$ty>::from_be_bytes(slice);
                    offset += size;
                    val
                };)*
                assert_eq!(offset, Self::SIZE);
                Self { $($field,)* }
            }

            $pub fn to_bytes(&self) -> [u8; Self::SIZE] {
                let mut bytes = [0u8; Self::SIZE];
                let mut offset: usize = 0;
                $({
                    let size: usize = std::mem::size_of::<$ty>();
                    let slice = self.$field.to_be_bytes();
                    (&mut bytes[offset..(offset + size)]).copy_from_slice(&slice);
                    offset += size;
                })*
                assert_eq!(offset, Self::SIZE);
                bytes
            }
        }
    };
}

#[macro_export]
macro_rules! __bin_struct_complex_size {
    (Number($ty:ty)) => {
        std::mem::size_of::<$ty>()
    };
    (Bytes($size:literal)) => {
        $size
    };
    (Struct($ty:ty)) => {
        <$ty>::SIZE
    };
}

#[macro_export]
macro_rules! __bin_struct_complex_type {
    (Number($ty:ty)) => {
        $ty
    };
    (Bytes($size:literal)) => {
        [u8; $size]
    };
    (Struct($ty:ty)) => {
        $ty
    };
}

#[macro_export]
macro_rules! __bin_struct_complex_from {
    (Number($ty:ty), $slice:expr) => {
        <$ty>::from_be_bytes($slice)
    };
    (Bytes($size:literal), $slice:expr) => {
        $slice
    };
    (Struct($ty:ty), $slice:expr) => {
        <$ty>::from_bytes($slice)
    };
}

#[macro_export]
macro_rules! __bin_struct_complex_to {
    (Number($ty:ty), $field:expr) => {
        $field.to_be_bytes()
    };
    (Bytes($size:literal), $field:expr) => {
        $field
    };
    (Struct($ty:ty), $field:expr) => {
        $field.to_bytes()
    };
}

#[macro_export]
macro_rules! bin_struct_complex {
    {
        $(#[$meta:meta])*
        $pub:vis $struct_name:ident { $($field_pub:vis $field:ident -> $_ftyk:ident($_ftyv:tt))* }
        $impl_pub:vis impl
    } => {
        $(#[$meta])*
        $pub struct $struct_name {
            $($field_pub $field: macros::__bin_struct_complex_type!($_ftyk($_ftyv)),)*
        }

        macros::bin_struct_complex_impl! {
            $struct_name { $($field -> $_ftyk($_ftyv))* }
            $impl_pub impl
        }
    };
}

#[macro_export]
macro_rules! bin_struct_complex_impl {
    {
        $struct_name:ident { $($field:ident -> $_ftyk:ident($_ftyv:tt))* }
        $impl_pub:vis impl
    } => {

        impl $struct_name {
            $impl_pub const SIZE: usize = { $(macros::__bin_struct_complex_size!($_ftyk($_ftyv)) + )* 0 };

            $impl_pub fn from_bytes(bytes: [u8; Self::SIZE]) -> Self {
                let mut offset: usize = 0;
                $(let $field = {
                    let size = macros::__bin_struct_complex_size!($_ftyk($_ftyv));
                    let slice = bytes[offset..(offset + size)].try_into().unwrap();
                    let val = macros::__bin_struct_complex_from!($_ftyk($_ftyv), slice);
                    offset += size;
                    val
                };)*
                assert_eq!(offset, Self::SIZE);
                Self { $($field,)* }
            }

            $impl_pub fn to_bytes(&self) -> [u8; Self::SIZE] {
                let mut bytes = [0u8; Self::SIZE];
                let mut offset: usize = 0;
                $({
                    let size = macros::__bin_struct_complex_size!($_ftyk($_ftyv));
                    let slice = macros::__bin_struct_complex_to!($_ftyk($_ftyv), self.$field);
                    (&mut bytes[offset..(offset + size)]).copy_from_slice(&slice);
                    offset += size;
                })*
                assert_eq!(offset, Self::SIZE);
                bytes
            }
        }
    };
}
