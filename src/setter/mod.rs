pub mod series;
pub mod table;

const EPSILON: f64 = 1e-9;
fn fequal(f: f64, s: f64) -> bool {
    return (f - s).abs() < EPSILON;
}

fn is_possible_end(series: &series::Series, value: f64) -> bool {
    for i in 1..series.series.len() {
        if fequal(series.series[i], series.gradation + value) {
            return true;
        }
    }

    return false;
}

pub fn generate_series(src: &[series::Series]) -> Option<table::Table> {
    if src.is_empty() {
        return None;
    }

    let mut table: table::Table = table::Table::new(vec![]);
    table.add_series(series::Series::from_series(&src[0]));

    for i in 1..(src.len() - 1) {
        let prev_count: usize = table.body.len();
        let curr: &series::Series = &src[i];
        let next: &series::Series = &src[i + 1];

        for j in 1..curr.series.len() {
            if is_possible_end(next, curr.series[j]) {
                let possible = series::Series::from_vec((j + 1) as f64, curr.series[..j].to_vec());
                table.add_series(possible);
            }
        }

        if table.body.len() == prev_count {
            return None;
        }
    }

    table.add_series(series::Series::from_series(&src[src.len() - 1]));
    return Some(table);
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
    let grad_tables = separate_table_by_grad(possible_series)?;
    let series_counts: Vec<usize> = grad_tables.iter().map(|t: &table::Table| t.body.len()).collect();
    if series_counts.iter().any(|&count| count == 0) {
        return None;
    }

    let total_combinations: usize = series_counts.iter().product();
    let mut result = table::Table { body: Vec::with_capacity(total_combinations) };
    let mut indices = vec![0usize; grad_tables.len()];

    for _ in 0..total_combinations {
        let total_values: usize = indices.iter().enumerate()
            .map(|(i, &idx)| grad_tables[i].body[idx].series.len())
            .sum();

        let mut new_series = series::Series {
            gradation: 0.0,
            series: Vec::with_capacity(total_values),
        };

        for (i, &idx) in indices.iter().enumerate() {
            let s = &grad_tables[i].body[idx];
            new_series.series.extend_from_slice(&s.series);
        }

        result.body.push(new_series);
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

    return Some(result);
}
