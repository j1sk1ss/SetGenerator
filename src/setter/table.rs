use super::fequal;
use super::series;

pub struct Table {
    pub body: Vec<series::Series>
}

impl Default for Table {
    fn default() -> Table {
        return Table {
            body: vec![
                series::Series::from_vec(0.001, vec![1., 1.001, 1.002, 1.003, 1.004, 1.005, 1.006, 1.007, 1.008, 1.009, 1.01]),
                series::Series::from_vec(0.005, vec![1., 1.005, 1.01]),
                series::Series::from_vec(0.01, (0..=50).map(|i| 1.0 + i as f64 * 0.01).collect()),
                series::Series::from_vec(0.1, (0..=20).map(|i| 1.0 + i as f64 * 0.1).collect()),
                series::Series::from_vec(0.5, (1..=50).map(|i| i as f64 * 0.5).collect()),
                series::Series::from_vec(1.0, (1..=25).map(|i| i as f64).collect()),
                series::Series::from_vec(5.0, vec![5., 10., 15., 20., 25., 30.]),
                series::Series::from_vec(10.0, (1..=10).map(|i| i as f64 * 10.0).collect()),
                series::Series::from_vec(25.0, vec![25., 50., 75., 100.]),
            ],
        }
    }
}

impl Table {
    pub fn new(series: Vec<series::Series>) -> Table {
        return Table {
            body: series
        }
    }

    pub fn empty() -> Table {
        return Table {
            body: vec![]
        }
    }

    pub fn add_series(&mut self, series: series::Series) -> bool {
        self.body.push(series);
        return true;
    }

    pub fn to_uniq(&mut self) -> bool {
        if self.body.len() == 0 {
            return false;
        }

        let mut unique: Vec<series::Series> = Vec::new();
        'outer: for s in &self.body {
            for u in &unique {
                if s.equal(u) {
                    continue 'outer;
                }
            }

            unique.push(s.clone());
        }

        self.body = unique;
        return true;
    }

    pub fn sort_series(&mut self) -> bool {
        if self.body.is_empty() {
            return false;
        }
    
        for series in self.body.iter_mut() {
            if series.series.len() <= 1 {
                continue;
            }
    
            series.series.sort_by(|a: &f64, b: &f64| a.partial_cmp(b).unwrap());
    
            let mut unique = Vec::with_capacity(series.series.len());
            unique.push(series.series[0]);
    
            for &value in series.series.iter().skip(1) {
                if !fequal(value, *unique.last().unwrap()) {
                    unique.push(value);
                }
            }
    
            series.series = unique;
        }
    
        return true;
    }    

    pub fn filter_series_by_range(&mut self, min: f64, max: f64) -> bool {
        if self.body.is_empty() {
            return false;
        }
    
        self.body.retain(|series: &series::Series| {
            !series.series.iter().any(|&v| v < min || v > max)
        });
    
        return true;
    }
}