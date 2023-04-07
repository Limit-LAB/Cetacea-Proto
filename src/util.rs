macro_rules! cfg_wasm {
    ($($item:item)*) => {
            $(
                #[cfg(feature = "wasm")]
                #[cfg_attr(docsrs, doc(cfg(feature = "wasm")))]
                $item
            )*
    };
}

pub(crate) use cfg_wasm;
