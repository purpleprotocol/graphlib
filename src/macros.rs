/// Find the minimum of a set of numbers
#[macro_export]
macro_rules! min {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => (::core::cmp::min($x, min!($($z),*)));
}

/// Returns a HashSet containing the passed values.
#[macro_export]
macro_rules! set {
    () => ({
        HashSet::new()
    });

    ($fst:expr $(, $v:expr)*) => ({
        let mut set = HashSet::with_capacity(count!($fst $(, $v)*));

        set.insert($fst);
        $(set.insert($v);)*

        set
    });
}

/// Counts the number of values passed to it.
#[macro_export]
macro_rules! count {
    () => (0);
    ($fst:expr) => (1);
    ($fst:expr, $snd:expr) => (2);
    ($fst:expr, $snd:expr $(, $v:expr)*) => (1 + count!($snd $(, $v)*));
}
