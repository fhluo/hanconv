use pyo3::prelude::*;

/// Convert between Chinese character variants.
#[pymodule]
mod hanconv {
    use pyo3::prelude::*;

    /// Convert Simplified Chinese to Traditional Chinese.
    #[pyfunction]
    fn s2t(s: &str) -> String {
        hanconv::s2t(s)
    }

    /// Convert Traditional Chinese to Simplified Chinese.
    #[pyfunction]
    fn t2s(s: &str) -> String {
        hanconv::t2s(s)
    }

    /// Convert Simplified Chinese to Traditional Chinese (Taiwan).
    #[pyfunction]
    fn s2tw(s: &str) -> String {
        hanconv::s2tw(s)
    }

    /// Convert Traditional Chinese (Taiwan) to Simplified Chinese.
    #[pyfunction]
    fn tw2s(s: &str) -> String {
        hanconv::tw2s(s)
    }

    /// Convert Simplified Chinese to Traditional Chinese (Taiwan) with Taiwanese idiom.
    #[pyfunction]
    fn s2twp(s: &str) -> String {
        hanconv::s2twp(s)
    }

    /// Convert Traditional Chinese (Taiwan) to Simplified Chinese with Mainland Chinese idiom.
    #[pyfunction]
    fn tw2sp(s: &str) -> String {
        hanconv::tw2sp(s)
    }

    /// Convert Traditional Chinese to Traditional Chinese (Taiwan).
    #[pyfunction]
    fn t2tw(s: &str) -> String {
        hanconv::t2tw(s)
    }

    /// Convert Traditional Chinese (Taiwan) to Traditional Chinese.
    #[pyfunction]
    fn tw2t(s: &str) -> String {
        hanconv::tw2t(s)
    }

    /// Convert Simplified Chinese to Traditional Chinese (Hong Kong).
    #[pyfunction]
    fn s2hk(s: &str) -> String {
        hanconv::s2hk(s)
    }

    /// Convert Traditional Chinese (Hong Kong) to Simplified Chinese.
    #[pyfunction]
    fn hk2s(s: &str) -> String {
        hanconv::hk2s(s)
    }

    /// Convert Traditional Chinese to Traditional Chinese (Hong Kong).
    #[pyfunction]
    fn t2hk(s: &str) -> String {
        hanconv::t2hk(s)
    }

    /// Convert Traditional Chinese (Hong Kong) to Traditional Chinese.
    #[pyfunction]
    fn hk2t(s: &str) -> String {
        hanconv::hk2t(s)
    }

    /// Convert Traditional Chinese characters (Kyūjitai) to New Japanese Kanji (Shinjitai).
    #[pyfunction]
    fn t2jp(s: &str) -> String {
        hanconv::t2jp(s)
    }

    /// Convert New Japanese Kanji (Shinjitai) to Traditional Chinese characters (Kyūjitai).
    #[pyfunction]
    fn jp2t(s: &str) -> String {
        hanconv::jp2t(s)
    }
}
