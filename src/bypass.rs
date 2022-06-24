use pyo3::prelude::*;

/// Use a Python module to bypass some crawling annoyances
pub fn http_get_bypassed(url: impl AsRef<str>) -> Result<String, anyhow::Error> {
    let url = url.as_ref();

    Python::with_gil(|py| -> anyhow::Result<String> {
        let scraper: Py<PyAny> = PyModule::import(py, "cloudscraper")?
            .call_method0("create_scraper")?
            .into();

        let res = scraper.call_method1(py, "get", (url,))?;

        let text: String = res.getattr(py, "text")?.extract(py)?;

        Ok(text)
    })
}
