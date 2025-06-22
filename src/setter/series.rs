#[derive(Clone)]
pub struct Series {
    pub gradation: f64,
    pub series: Vec<f64>
}

impl Series {
    pub fn name(&self) -> String {
        let mut s = format!("Grad: {:<6.3} | ", self.gradation);
        let vals: Vec<String> = self.series.iter().map(|v| format!("{:.3}", v)).collect();
        s.push_str(&vals.join(" "));
        return s;
    }

    pub fn from_vec(gradation: f64, series: Vec<f64>) -> Series {
        return Series {
            gradation,
            series: series
        };
    }

    pub fn from_series(src: &Series) -> Series {
        return Series {
            gradation: src.gradation,
            series: src.series.clone()
        };
    }

    pub fn equal(&self, dst: &Series) -> bool {
        return self.series == dst.series;
    }
}