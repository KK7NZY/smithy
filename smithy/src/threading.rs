use crate::util::truncate_float;
/// Represents the different thread classes (1A, 2A, and 3A) for external threads.
///
/// - A1: Loose fit (with allowance).
/// - A2: General fit (with allowance).
/// - A3 Precision fit (no allowance).
pub enum ThreadClass {
    A1,
    A2,
    A3,
}

/// Calculates the thread allowance for Unified Thread Standard (UTS) external threads.
///
/// The thread allowance is calculated using the formula:
///
/// ```markdown
/// es = 0.3 × [ 0.0015 × ³√D + 0.0015 × √LE + 0.015 × ³√P² ]
/// ```
///
/// Where:
/// - `D` is the Nominal Diameter (in inches),
/// - `P` is the Pitch (calculated as 1 / TPI),
/// - `LE` is the Length of Engagement (if not specified, defaults to `5 × P`).
/// - `0.3` is the standard factor applied to the pitch diameter tolerance for Class 1A and 2A threads.
///
/// # Parameters
/// - d: Nominal Diameter (D), in inches.
/// - p: Pitch (P), calculated as `1 / TPI` (Threads Per Inch).
/// - class: The thread class (1A, 2A, or 3A). Class 3A has no allowance.
/// - le: Length of Engagement (LE). If not provided, defaults to `5 × P`.
///
/// # Example
/// ```rust
/// ```
pub fn calc_uts_allowance(d: f64, p: f64, class: &ThreadClass, le: Option<f64>) -> f64 {
    let le = le.unwrap_or(5.0 * p);
    let k1 = 0.0015;
    let k2 = 0.015;
    let n = match class {
        ThreadClass::A1 | ThreadClass::A2 => 0.3,
        ThreadClass::A3 => return 0.0,
    };
    n * (k1 * d.cbrt() + k1 * le.sqrt() + k2 * p.powi(2).cbrt())
}

/// Calculates the base tolerance (T) from which other tolerances are derived.
///
/// This function calculates the base tolerance based on the nominal diameter (D),
/// pitch (P), and length of engagement (LE). The formula used is:
///
/// ```markdown
/// T = 0.0015 × ³√D + 0.0015 × √LE + 0.015 × ³√P²
/// ```
///
/// # Parameters:
/// - d: The nominal diameter (D) in inches.
/// - p: The pitch (P), calculated as `1 / TPI` (threads per inch).
/// - le: The length of engagement (LE), typically measured in inches.
///
/// # Returns:
/// - `f64`: The base tolerance value (T), from which other tolerances are derived.
///
/// # Example:
/// ```rust
/// ```
fn calc_uts_base_tolerance(d: f64, p: f64, le: f64) -> f64 {
    let k1 = 0.0015;
    let k2 = 0.015;
    k1 * d.cbrt() + k1 * le.sqrt() + k2 * p.powi(2).cbrt()
}

#[derive(Debug, Default)]
pub struct UnifiedThreadCalc {
    p: f64,
    d_min: f64,
    d_max: f64,
    d2: f64,
    d2_min: f64,
    d2_max: f64,
    h: f64,
    es: f64,
    t: f64,
    td: f64,
    td2: f64,
    le: f64,
}

pub fn calc_uts_thread(
    d: f64,
    tpi: u32,
    class: &ThreadClass,
    le: Option<u32>,
) -> UnifiedThreadCalc {
    let p = 1.0 / tpi as f64;
    let le = le.unwrap_or(9) as f64 * p;
    let es = calc_uts_allowance(d, p, class, Some(le));
    let d_max = d - es; // Max. Major Dia.
    let t = calc_uts_base_tolerance(d, p, le);
    let td = match class {
        // Tolerance for External Major Diameter
        ThreadClass::A1 => 0.3 * t,
        ThreadClass::A2 | ThreadClass::A3 => 0.06 * p.powi(2).cbrt(),
    };
    let d_min = d_max - td; // Min. Major Dia.
    let h = 0.866025404 * p; // Height Triangle
    let d2 = d - 2.0 * ((3.0 / 8.0) * h); // External Pitch Dia.
    let d2_max = d2 - es; // Max. Pitch Dia.
    let td2 = match class {
        // Tolerance for External Pitch Diameter
        ThreadClass::A1 => 1.5 * t,
        ThreadClass::A2 => t,
        ThreadClass::A3 => 0.75 * t,
    };
    let d2_min = d2_max - td2; // Min. Pitch Dia.
    UnifiedThreadCalc {
        p,
        le,
        es,
        d_min,
        d_max,
        t,
        td,
        td2,
        d2,
        d2_min,
        d2_max,
        h,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_uts_thread_allowance() {
        let pitch = 1.0 / 28.0;
        let es = truncate_float(
            calc_uts_allowance(0.5, pitch, &ThreadClass::A2, Some(0.4)),
            6,
        );
        assert_eq!(es, 0.00113);

        let pitch = 1.0 / 20.0;
        let es = truncate_float(
            calc_uts_allowance(0.25, pitch, &ThreadClass::A1, Some(0.0125)),
            6,
        );
        assert_eq!(es, 0.000945);

        let es = calc_uts_allowance(0.25, pitch, &ThreadClass::A3, Some(0.0125));
        assert_eq!(es, 0.0);
    }

    #[test]
    fn test_calc_uts_thread() {
        let n = calc_uts_extern_thread(0.250, 20, &ThreadClass::A2, Some(9));
        println!("{:?}", n);
    }
}
