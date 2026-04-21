    use pyo3::prelude::*;

#[pyclass]
pub struct LinearModel {
    pub weight: f64,
    pub bias: f64,
}

#[pymethods]
impl LinearModel {
    #[new]
    pub fn new() -> Self {
        Self {
            weight: 0.0,
            bias: 0.0,
        }
    }

    pub fn predict(&self, x: f64) -> f64 {
        self.weight * x + self.bias
    }
    
    // Change this line:
    pub fn train(&mut self, x: Vec<f64>, y: Vec<f64>, lr: f64, epochs: usize) {
        for _ in 0..epochs {
            let mut dw = 0.0;
            let mut db = 0.0;
    
            let n = x.len() as f64;
    
            for i in 0..x.len() {
                let pred = self.predict(x[i]);
                let error = pred - y[i];
    
                dw += error * x[i];
                db += error;
            }
    
            self.weight -= lr * dw / n;
            self.bias -= lr * db / n;
        }
    }
}

#[pymodule]
fn scratch_model(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<LinearModel>()?;
    Ok(())
}