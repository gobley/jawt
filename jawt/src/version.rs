// Copyright (c) 2025 Gobley Contributors.

//! Implements the [AwtVersion] struct.

use jni::sys::jint;

use crate::sys::*;

/// Structure that represents a version of [Awt]. Specifying an earlier version will limit the
/// available functions of [Awt].
///
/// [Awt]: crate::awt::Awt
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AwtVersion(pub(crate) jint);

impl AwtVersion {
    /// Constructs an [AwtVersion] from an integer value.
    ///
    /// # Safety
    ///
    /// The caller should verify the value before calling this function.
    pub const unsafe fn from_raw(value: jint) -> Self {
        Self(value)
    }

    /// Returns the underlying integer value of the given [AwtVersion].
    pub const fn inner(self) -> jint {
        self.0
    }

    pub const VERSION_1_3: Self = Self(JAWT_VERSION_1_3);
    #[cfg(feature = "java-1-4")]
    pub const VERSION_1_4: Self = Self(JAWT_VERSION_1_4);
    #[cfg(feature = "java-1-4")]
    pub const VERSION_1_7: Self = Self(JAWT_VERSION_1_7);
    #[cfg(feature = "java-9")]
    pub const VERSION_9: Self = Self(JAWT_VERSION_9);
}

impl From<AwtVersion> for jint {
    fn from(value: AwtVersion) -> Self {
        value.0
    }
}
