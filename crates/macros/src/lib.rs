#[macro_export]
macro_rules! ensure {
    ($predicate:expr, $err:expr) => {
        if !$predicate {
            return $err.map_err(core::convert::Into::into);
        }
    };
}
