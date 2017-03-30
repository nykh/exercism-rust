// naive approach:
// compute all consecutive products. return max
//
// sophisticated approach 1
// Product(x1...xn) o< x1, ..., xn
// each sliding from x(i)...x(i+n) to x(i+1)...x(i+n+1)
// can be determined as 
//    increase: x(i) < x(i+n+1)
//    decrease:      >
//    remain:        =
// During first pass, record the increase and decrease
// of products without computing product. Mark local
// maxima. Compute each local maxima and then return max

use std::cmp::*;

pub fn prod(ns: &[u32]) -> u32 {
    ns.iter().fold(1, |acc, n| acc * n)
}

pub fn lsp(xs: &str, n: usize) -> Result<u32, ()> {
    if xs.len() < n {
        return Err(())
    } else if n == 0 {
        return Ok(1)
    }

    let mut ds = Vec::new();
    for c in xs.chars() {
        if let Some(d) = c.to_digit(10) {
            ds.push(d);
        } else {
            return Err(())
        }
    }

    if xs.len() == n {
        return Ok(prod(ds.as_slice()))
    } 
    assert!(0 < n); assert!(n < xs.len());
    let num_prods: usize = ds.len() - n + 1;
    assert!(num_prods > 1);
    let mut change = Vec::<Ordering>::with_capacity(num_prods-1);
    unsafe { change.set_len(num_prods-1); }
    for i in 0..(num_prods - 1) {
        change[i] = ds[i + n].cmp(&ds[i]);
    }
    
    let mut maxima = Vec::<bool>::with_capacity(num_prods);
    unsafe { maxima.set_len(num_prods); }
    maxima[0] = change[0] == Ordering::Less;
    maxima[num_prods-1] = change[num_prods-2] == Ordering::Greater;
    for i in 1..(num_prods-1) {
        maxima[i] = change[i-1] == Ordering::Greater && change[i] == Ordering::Less;
    }

    if maxima.iter().any(|&b| b) {
        let prods: Vec<u32> = maxima.iter()
                      .enumerate()
                      .filter(|&(_, &b)| b)
                      .map(|(i, _)| prod(&ds[i..(i+n)]))
                      .collect();
    
        Ok(*prods.iter().max().unwrap())
    } else {
        // constant (no change, change=[EQUAL,...])
        Ok(prod(&ds[0..n]))
    }
}