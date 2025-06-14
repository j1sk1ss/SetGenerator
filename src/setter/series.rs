#[derive(Clone)]
pub struct Series {
    pub gradation: f64,
    pub series: Vec<f64>
}

impl Series {
    pub fn new(series_count: usize, gradation: f64) -> Self {
        return Self {
            gradation,
            series: vec![0.0; series_count]
        }
    }

    pub fn from_vec(gradation: f64, series: Vec<f64>) -> Self {
        return Self {
            gradation,
            series: series
        }
    }

    pub fn from_series(src: &Series) -> Self {
        return Self {
            gradation: src.gradation,
            series: src.series.clone()
        }
    }

    pub fn equal(&self, dst: &Series) -> bool {
        self.series == dst.series
    }
}