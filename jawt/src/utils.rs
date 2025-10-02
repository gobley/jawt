// Copyright (c) 2025 Gobley Contributors.

macro_rules! unwrap_fn {
    ($expr:expr, $ty:ident.$fn:ident $(,)?) => {
        ($expr.$fn).expect(concat!(
            stringify!($ty),
            ".",
            stringify!($fn),
            "is not available",
        ))
    };
}

pub(crate) use unwrap_fn;
