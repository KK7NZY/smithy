pub fn truncate_float(f: f64, n: u32) -> f64 {
    let factor = 10_f64.powi(n as i32);
    (f * factor).round() / factor
}

#[cfg(test)]

mod tests {
    use crate::util::truncate_float;

    #[test]
    fn test_truncate_float() {
        assert_eq!(truncate_float(0.0011297934537308734, 4), 0.0011);
        assert_eq!(truncate_float(0.001196095376922672, 5), 0.00120);
    }
}