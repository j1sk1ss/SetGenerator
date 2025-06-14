pub struct Series {
    pub gradation: f64,
    pub series: Vec<f64>
}

impl Series {
    pub fn new(series_count: usize, gradation: f64) -> Self {
        Self {
            gradation,
            series: vec![0.0; series_count]
        }
    }

    pub fn from_series(src: &Series) -> Self {
        Self {
            gradation: src.gradation,
            series: src.series.clone()
        }
    }

    pub fn equal(&self, dst: &Series) -> bool {
        self.series == dst.series
    }
}