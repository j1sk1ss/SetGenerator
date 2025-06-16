use super::fequal;
use super::series;
use super::series::Series;

use std::fs::File;
use std::io::prelude::*;
use std::usize;

pub struct Table {
    pub body: Vec<series::Series>
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

    pub fn default() -> Table {
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
    
        self.body.retain(|series: &Series| {
            !series.series.iter().any(|&v| v < min || v > max)
        });
    
        return true;
    }

    pub fn print(&self, highlight: usize, prnt: &dyn Fn(&str, i32)) -> bool {
        if self.body.is_empty() {
            return false;
        }
    
        let mut y = 3;
        prnt("+------+---------+-------------------------------+", 0);
        prnt("| Num  | Grad-on |            Values             |", 1);
        prnt("+------+---------+-------------------------------+", 2);
    
        for (i, s) in self.body.iter().enumerate() {
            if s.series.is_empty() {
                continue;
            }
    
            let mut line = String::new();
            if i != highlight {
                line += &format!("| {:4} | {:7.3} |", i, s.gradation);
            } 
            else {
                line += &format!("> {:4} | {:7.3} |", i, s.gradation);
            }
    
            for (j, val) in s.series.iter().enumerate() {
                if j > 0 {
                    line += " ";
                }
                line += &format!("{:.3}", val);
            }
    
            prnt(&line, y);
            y += 1;
        }
    
        prnt("+------+---------+-------------------------------+", y);
        return true;
    }     

    pub fn save(&self, fp: &mut File) -> std::io::Result<()> {
        for series in &self.body {
            fp.write_all(&series.gradation.to_le_bytes())?;
    
            let count = series.series.len() as u64;
            fp.write_all(&count.to_le_bytes())?;
    
            for &val in &series.series {
                fp.write_all(&val.to_le_bytes())?;
            }
        }
    
        return Ok(());
    }
}