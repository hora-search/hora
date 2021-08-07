#[macro_export]
macro_rules! vec_iter {
    (  $v:expr, $d: ident) => {
        #[cfg(not(feature = "no_thread"))]
        let $d = $v.par_iter();
        #[cfg(feature = "no_thread")]
        let $d = $v.iter();
    };
}

#[macro_export]
macro_rules! vec_iter_mut {
    (  $v:expr, $d: ident) => {
        #[cfg(not(feature = "no_thread"))]
        let $d = $v.par_iter_mut();
        #[cfg(feature = "no_thread")]
        let $d = $v.iter_mut();
    };
}

#[macro_export]
macro_rules! into_iter {
    (  $v:expr, $d: ident) => {
        #[cfg(not(feature = "no_thread"))]
        let $d = $v.into_par_iter();
        #[cfg(feature = "no_thread")]
        let $d = $v.into_iter();
    };
}
