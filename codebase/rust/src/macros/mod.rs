#[macro_export]
macro_rules! visual_debug {
    ($($t:tt)*) => {
        #[cfg(any(feature = "visual_debug"))]
        $($t)*
    };
}