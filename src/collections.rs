use std::collections::HashMap;

#[derive(Debug)]
struct CentralTendency {
  median: f64,
  mode: i64,
}

pub fn run() {
  let vec = vec![
    1, 2, 3, 3, 3, 4, 3, 4, 4, 4, 4, 4, 4, 4, 4, 2, 2, 2, 2, 2, 0, 0, 0, 0,
  ];

  if let Some(central_tendency) = calc_central_tendencies(&vec) {
    println!("{:#?}", central_tendency);
  }
}

fn calc_central_tendencies(vec: &[i64]) -> Option<CentralTendency> {
  let Some(mode) = calc_mode(vec) else {
    return None;
  };

  let Some(median) = calc_median(vec) else {
    return None;
  };

  Some(CentralTendency { median, mode })
}

fn calc_mode(arr: &[i64]) -> Option<i64> {
  if arr.is_empty() {
    return None;
  }

  let mut map: HashMap<i64, usize> = HashMap::new();

  for &i in arr {
    let value = map.entry(i).or_insert(0);

    *value += 1;
  }

  if let Some((&first_key, &first_val)) = map.iter().next() {
    let mut mode: (i64, usize) = (first_key, first_val);

    for (key, value) in &map {
      if mode.1 < *value {
        mode = (*key, *value);
      }
    }

    return Some(mode.0);
  }

  None
}

fn calc_median(arr: &[i64]) -> Option<f64> {
  if arr.is_empty() {
    return None;
  }

  let mut sorted = arr.to_vec();
  sorted.sort_unstable();
  let len = sorted.len();

  if len % 2 == 0 {
    // even

    let l = sorted[(len / 2) - 1];
    let r = sorted[len / 2];

    Some((l + r) as f64 / 2.0)
  } else {
    // odd
    Some(sorted[len / 2] as f64)
  }
}
