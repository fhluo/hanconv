use pyo3::prelude::*;

#[pymodule]
mod hanconv {
    use pyo3::prelude::*;

    #[pyfunction]
    fn s2t(s: &str) -> String {
        hanconv::s2t(s)
    }

    #[pyfunction]
    fn t2s(s: &str) -> String {
        hanconv::t2s(s)
    }

    #[pyfunction]
    fn s2tw(s: &str) -> String {
        hanconv::s2tw(s)
    }

    #[pyfunction]
    fn tw2s(s: &str) -> String {
        hanconv::tw2s(s)
    }

    #[pyfunction]
    fn s2twp(s: &str) -> String {
        hanconv::s2twp(s)
    }

    #[pyfunction]
    fn tw2sp(s: &str) -> String {
        hanconv::tw2sp(s)
    }

    #[pyfunction]
    fn t2tw(s: &str) -> String {
        hanconv::t2tw(s)
    }

    #[pyfunction]
    fn tw2t(s: &str) -> String {
        hanconv::tw2t(s)
    }

    #[pyfunction]
    fn s2hk(s: &str) -> String {
        hanconv::s2hk(s)
    }

    #[pyfunction]
    fn hk2s(s: &str) -> String {
        hanconv::hk2s(s)
    }

    #[pyfunction]
    fn t2hk(s: &str) -> String {
        hanconv::t2hk(s)
    }

    #[pyfunction]
    fn hk2t(s: &str) -> String {
        hanconv::hk2t(s)
    }

    #[pyfunction]
    fn t2jp(s: &str) -> String {
        hanconv::t2jp(s)
    }

    #[pyfunction]
    fn jp2t(s: &str) -> String {
        hanconv::jp2t(s)
    }
}
