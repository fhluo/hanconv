use pyo3::prelude::*;

#[pyfunction]
fn s2t(s: &str) -> String {
    ::hanconv::s2t(s)
}

#[pyfunction]
fn t2s(s: &str) -> String {
    ::hanconv::t2s(s)
}

#[pyfunction]
fn s2tw(s: &str) -> String {
    ::hanconv::s2tw(s)
}

#[pyfunction]
fn tw2s(s: &str) -> String {
    ::hanconv::tw2s(s)
}

#[pyfunction]
fn s2twp(s: &str) -> String {
    ::hanconv::s2twp(s)
}

#[pyfunction]
fn tw2sp(s: &str) -> String {
    ::hanconv::tw2sp(s)
}

#[pyfunction]
fn t2tw(s: &str) -> String {
    ::hanconv::t2tw(s)
}

#[pyfunction]
fn tw2t(s: &str) -> String {
    ::hanconv::tw2t(s)
}

#[pyfunction]
fn s2hk(s: &str) -> String {
    ::hanconv::s2hk(s)
}

#[pyfunction]
fn hk2s(s: &str) -> String {
    ::hanconv::hk2s(s)
}

#[pyfunction]
fn t2hk(s: &str) -> String {
    ::hanconv::t2hk(s)
}

#[pyfunction]
fn hk2t(s: &str) -> String {
    ::hanconv::hk2t(s)
}

#[pyfunction]
fn t2jp(s: &str) -> String {
    ::hanconv::t2jp(s)
}

#[pyfunction]
fn jp2t(s: &str) -> String {
    ::hanconv::jp2t(s)
}

#[pymodule]
fn hanconv(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(s2t, m)?)?;
    m.add_function(wrap_pyfunction!(t2s, m)?)?;
    m.add_function(wrap_pyfunction!(s2tw, m)?)?;
    m.add_function(wrap_pyfunction!(tw2s, m)?)?;
    m.add_function(wrap_pyfunction!(s2twp, m)?)?;
    m.add_function(wrap_pyfunction!(tw2sp, m)?)?;
    m.add_function(wrap_pyfunction!(t2tw, m)?)?;
    m.add_function(wrap_pyfunction!(tw2t, m)?)?;
    m.add_function(wrap_pyfunction!(s2hk, m)?)?;
    m.add_function(wrap_pyfunction!(hk2s, m)?)?;
    m.add_function(wrap_pyfunction!(t2hk, m)?)?;
    m.add_function(wrap_pyfunction!(hk2t, m)?)?;
    m.add_function(wrap_pyfunction!(t2jp, m)?)?;
    m.add_function(wrap_pyfunction!(jp2t, m)?)?;
    Ok(())
}
