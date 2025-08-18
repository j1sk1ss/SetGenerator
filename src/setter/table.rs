use super::fequal;
use super::series;
use std::fs;
use std::path;

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
            ]
        };
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
        println!("[DEBUG] add_series(series.grad={} series.len()={})", series.gradation, series.series.len());
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
    
        for series in &mut self.body {
            if series.series[0] != min && min != 0. {
                series.series.insert(0, min);
            }
        }
    
        return true;
    }    

    pub fn save_table_as_rtf(&self, path: &str) -> std::io::Result<()> {
        use std::io::{Write, BufWriter};
        use std::fs::File;

        fn escape_rtf(s: &str) -> String {
            let mut result = String::new();
            for c in s.chars() {
                if c <= '\u{7F}' {
                    result.push(c);
                } 
                else {
                    result.push_str(&format!("\\u{}?", c as i32));
                }
            }
            result
        }

        let dir_path: path::PathBuf = path::Path::new("results").join(path);
        if let Some(parent) = dir_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let file: File = File::create(path)?;
        let mut writer: BufWriter<File> = BufWriter::new(file);

        writeln!(writer, r"{{\rtf1\ansi\ansicpg1251")?;
        writeln!(writer, r"{{\fonttbl {{\f0\fnil\fcharset204 Times New Roman;}}}}")?;
        writeln!(writer, r"\viewkind4\uc1")?;

        let col_widths: [i32; 3] = [ 1500, 1500, 6000 ];
        let mut accum: i32 = 0;
        let mut cellx: String = String::new();
        for &w in &col_widths {
            accum += w;
            cellx.push_str(&format!(r"\cellx{}", accum));
        }

        writeln!(writer, r"\trowd\trqc")?;
        writer.write_all(cellx.as_bytes())?;
        writeln!(
            writer,
            r"\intbl\b\f0\fs24 {}\cell {}\cell {}\cell\row",
            escape_rtf("№"),
            escape_rtf("Количество"),
            escape_rtf("Значения")
        )?;

        for (i, s) in self.body.iter().enumerate() {
            writeln!(writer, r"\trowd\trqc")?;
            writer.write_all(cellx.as_bytes())?;
            
            let values: String = s.series.iter()
                .map(|v| format!("{:.3}", v))
                .collect::<Vec<_>>()
                .join(" ");
            
            writeln!(
                writer,
                r"\intbl\f0\fs22 {}\cell {}\cell {}\cell\row",
                i + 1,
                s.series.len(),
                values
            )?;
        }

        writeln!(writer, r"}}")?;
        writer.flush()?;
        return Ok(());
    }
}