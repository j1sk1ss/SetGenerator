use super::series;
use std::fs::File;
use std::io::prelude::*;

pub struct Table {
    pub body: Vec<series::Series>
}

impl Table {
    pub fn new(series: Vec<series::Series>) -> Self {
        Self {
            body: series
        }
    }

    pub fn add_series(&mut self, series: series::Series) -> bool {
        self.body.push(series);
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