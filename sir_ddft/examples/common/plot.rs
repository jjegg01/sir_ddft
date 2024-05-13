//! Plotting via matplotlib 

use anyhow::{Result, anyhow};
use pyo3::{Python, types::{PyModule, PyBool, PyDict}};

use super::GenericState;

static PYCODE: &str = include_str!("../plot.py");
static PYPLOTFUNC: &str = "plot";

#[allow(non_snake_case)]
pub(crate) fn plot<S: GenericState>(data: &[(f64, S)], xlim: Option<(f64,f64)>, title: &str, outfile_prefix: &str) -> Result<()> {
    // Run in cooperation with Python
    Python::with_gil(|py| {
        // Unpack data into a Python dict
        let (t,fields) = S::into_pyarrays(py, data);
        let data_py = PyDict::new(py);
        data_py.set_item("t", t)?;
        for (field_name, field_data) in fields {
            data_py.set_item(field_name, field_data)?;
        }
        if let Some(xlim) = xlim {
            data_py.set_item("xlim", xlim)?;
        }
        data_py.set_item("title", title)?;
        // Instantiate module for plotting
        let module = PyModule::from_code(py, PYCODE, "", "")
            .map_err(|e| anyhow!("Failed to run Python code: {}", e))?;
        // Check if matplotlib is available
        let has_matplotlib = module.getattr("MATPLOTLIB_AVAILABLE")
            .map_err(|e| anyhow!("Failed to get MATPLOTLIB_AVAILABLE attribute: {}", e))?
            .downcast::<PyBool>()
            .map_err(|e| anyhow!("Cannot cast MATPLOTLIB_AVAILABLE to bool: {}", e))?;
        // Hand over to Python plotting code if matplotlib is available
        if has_matplotlib.is_true() {
            let plotfunc_py = module.getattr(PYPLOTFUNC)
                .map_err(|e| anyhow!("Failed to get {} function: {}", PYPLOTFUNC, e))?;
            plotfunc_py.call1((data_py, outfile_prefix))
                .map_err(|e| anyhow!("Failed to execute {} function: {}", PYPLOTFUNC, e))?;
        }
        // Dump data as .npz otherwise
        else {
            println!("Warning: Failed to import matplotlib for graphical output. Output will be stored in file {} instead.", outfile_prefix);
            println!("Hint: You can run the plotting code manually via 'python3 examples/plot.py {}' (needs matplotlib)", outfile_prefix);
            let np = PyModule::import(py, "numpy")
                .map_err(|e| anyhow!("Failed to import numpy: {}", e))?;
            let savez = np.getattr("savez")
                .map_err(|e| anyhow!("Failed to get numpy.savez: {}", e))?;
            savez.call((format!("{}.npz", outfile_prefix),), Some(data_py))
                .map_err(|e| anyhow!("Failed to call numpy.savez: {}", e))?;
        }
        Ok(())
    })
}