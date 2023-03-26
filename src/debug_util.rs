#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug {
    ($x:stmt) => { $x }
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug {
    ($x:stmt) => {  }
}
