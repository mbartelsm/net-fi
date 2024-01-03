/// Trait that verifies whether a given value is null or not.
pub trait FFIArg {
    /// Returns `true` if the value corresponds to a null value.
    /// Otherwise it returns `false`.
    fn is_null(&self) -> bool;

    /// Returns an option containing the original value if not-null, or `None` if null
    fn non_null(self) -> Option<Self>
    where
        Self: Sized,
    {
        if self.is_null() {
            None
        } else {
            Some(self)
        }
    }
}

impl<T: ?Sized> FFIArg for *const T {
    fn is_null(&self) -> bool {
        <*const T>::is_null(*self)
    }
}

impl<T: ?Sized> FFIArg for *mut T {
    fn is_null(&self) -> bool {
        <*mut T>::is_null(*self)
    }
}

macro_rules! never_null {
    ($($t:ty),*) => {
        $(
            impl FFIArg for $t {
                fn is_null(&self) -> bool { false }
            }
        )*
    }
}

never_null!(
    // Unsigned integers
    usize, u8, u16, u32, u64, u128,

    // Signed integers
    isize, i8, i16, i32, i64, i128,
    
    // Floating point numbers
    f32, f64,
    
    // Others
    bool
);
