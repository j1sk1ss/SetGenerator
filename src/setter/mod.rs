pub mod series;
pub mod table;

const EPSILON: f64 = 1e-9;
fn fequal(f: f64, s: f64) -> bool {
    return (f - s).abs() < EPSILON;
}

fn is_possible_end(series: &series::Series, value: f64) -> i32 {
    for i in 1..series.series.len() {
        if fequal(series.series[i], series.gradation + value) {
            return i as i32;
        }
    }

    return -1;
}

pub fn generate_series(src: &[series::Series]) -> table::Table {
    if src.is_empty() {
        return table::Table::empty();
    }

    println!("[DEBUG] generate_series()");
    for i in src {
        println!("[DEBUG] series.len()={}, series.grad={}", i.series.len(), i.gradation);
    }

    let mut table: table::Table = table::Table::new(vec![]);
    table.add_series(series::Series::from_series(&src[0]));

    let mut found_any: bool;
    for i in 1..(src.len() - 1) {
        let curr: &series::Series = &src[i];
        let next: &series::Series = &src[i + 1];

        found_any = false;
        for j in 1..curr.series.len() {
            if is_possible_end(next, curr.series[j]) >= 0 {
                found_any = true;
                table.add_series(series::Series::from_vec(curr.gradation, curr.series[..=j].to_vec()));
            }
        }

        if !found_any {
            println!("[WARN] No series end found! curr.grad={}, next.grad={}", curr.gradation, next.gradation);
            return table::Table::empty();
        }
    }

    table.add_series(series::Series::from_series(&src[src.len() - 1]));
    return table;
}

pub fn separate_table_by_grad(t: &table::Table) -> Option<Vec<table::Table>> {
    if t.body.is_empty() {
        return None;
    }

    let mut groups: Vec<(f64, table::Table)> = Vec::new();
    for s in &t.body {
        let s_copy: series::Series = series::Series::from_series(s);
        if let Some((_, group_table)) = groups.iter_mut().find(|(grad, _)| fequal(*grad, s.gradation)) {
            group_table.body.push(s_copy);
            continue;
        }

        groups.push((
            s.gradation,
            table::Table::new(vec![s_copy])
        ));
    }

    return Some(groups.into_iter().map(|(_, table)| table).collect());
}

pub fn generate_sets(possible_series: &table::Table) -> Option<table::Table> {
    let sep_tables: Option<Vec<table::Table>> = separate_table_by_grad(possible_series);
    if sep_tables.is_none() {
        return None;
    }

    println!("[DEBUG] sep_tables.len()={}", sep_tables.as_ref().unwrap().len());
    let grad_tables: Vec<table::Table> = sep_tables.unwrap();
    let series_counts: Vec<usize> = grad_tables.iter().map(|t: &table::Table| t.body.len()).collect();
    if series_counts.iter().any(|&count| count == 0) {
        return None;
    }

    let total_combinations: usize = series_counts.iter().product();
    let mut result: table::Table = table::Table::new(Vec::with_capacity(total_combinations));
    let mut indices: Vec<usize> = vec![0usize; grad_tables.len()];
    println!("[DEBUG] total_combinations={}", total_combinations);

    for _ in 0..total_combinations {
        let mut valid = true;
        let total_values: usize = indices.iter().enumerate()
            .map(|(i, &idx)| grad_tables[i].body[idx].series.len())
            .sum();
    
        let mut nseries: series::Series = series::Series::from_vec(0., Vec::with_capacity(total_values));
        let mut max_prev: Option<f64> = None;
    
        for (i, &idx) in indices.iter().enumerate() {
            let s: &series::Series = &grad_tables[i].body[idx];
            let mut slice = &s.series[..];
    
            if let Some(maxv) = max_prev {
                let cut_pos = slice.iter().position(|&x| x >= maxv).unwrap_or(slice.len());
                slice = &slice[cut_pos..];
            }
    
            if slice.is_empty() {
                valid = false;
                break;
            }
    
            nseries.series.extend_from_slice(slice);
            max_prev = Some(slice.iter().cloned().fold(f64::NEG_INFINITY, f64::max));
        }
    
        if valid {
            result.body.push(nseries);
        }
    
        for i in (0..indices.len()).rev() {
            indices[i] += 1;
            if indices[i] < series_counts[i] {
                break;
            } 
            else {
                indices[i] = 0;
            }
        }
    }    

    println!("[DEBUG] Result table series count: {}", result.body.len());
    return Some(result);
}
