use::std::iter::zip;
use::std::collections::HashMap;
use::pyo3::prelude::*;

#[pyfunction]
fn two_three_letter_map() -> PyResult<HashMap<String, String>> {
    let two_letter_codes = ["AU", "BR", "TW", "US", "CA"];
    let three_letter_codes = ["AUS", "BRA", "TWA", "USA", "CAN"];
    let mut two_three_map = HashMap::new();

    for (code_2, code_3) in zip(two_letter_codes.into_iter(), three_letter_codes.into_iter()){
        two_three_map.insert(code_2.to_string(), code_3.to_string());
    }
    Ok(two_three_map)
}

#[pymodule]
fn iso_tool(_py: Python, m: &PyModule) -> PyResult<()>{
    m.add_function(wrap_pyfunction!(two_three_letter_map, m)?)?;
    Ok(())
}