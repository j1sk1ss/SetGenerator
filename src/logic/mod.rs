const EPSILON: f64 = 1e-9;
fn fequal(f: f64, s: f64) -> bool {
    return (f - s).abs() < EPSILON;
}

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
}

pub struct Table {
    pub body: Vec<Series>
}

impl Table {
    pub fn new(series: Vec<Series>) -> Self {
        Self {
            body: series
        }
    }

    pub fn add_series(&mut self, series: Series) -> bool {
        self.body.push(series);
        return true;
    }

    pub fn is_end(&self, index: i16, value: f64) -> bool {
        let idx = index as usize;
        if idx >= self.body.len() {
            return false;
        }

        let series = &self.body[idx];
        for &item in &series.series {
            if fequal(value + series.gradation, item) {
                return true;
            }
        }
        
        return false;
    }
}
