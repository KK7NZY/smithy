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

impl ThreadClass {}

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
pub fn calc_uts_thread_allowance(d: f64, p: f64, class: &ThreadClass, le: Option<f64>) -> f64 {
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

pub fn calc_uts_thread(dia: f64, tpi: u32, class: &ThreadClass, le_p: Option<u32>) -> (f64, f64) {
    let p = 1.0 / tpi as f64;
    let le = le_p.unwrap_or(9) as f64 * p;
    let es = calc_uts_thread_allowance(dia, p, class, Some(le));
    let d_max = dia - es; // Max. Major Dia.
    let t = calc_uts_base_tolerance(dia, p, le);
    let td = match class {
        ThreadClass::A1 => 0.3 * t,
        ThreadClass::A2 | ThreadClass::A3 => 0.06 * p.powi(2).cbrt(),
    };
    let d_min = d_max - td; // Min. Major Dia.
    // let td2 = match class {
    //
    // }

    (d_min, d_max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_uts_thread_allowance() {
        let pitch = 1.0 / 28.0;
        let es = truncate_float(
            calc_uts_thread_allowance(0.5, pitch, &ThreadClass::A2, Some(0.4)),
            6,
        );
        assert_eq!(es, 0.00113);

        let pitch = 1.0 / 20.0;
        let es = truncate_float(
            calc_uts_thread_allowance(0.25, pitch, &ThreadClass::A1, Some(0.0125)),
            6,
        );
        assert_eq!(es, 0.000945);

        let es = calc_uts_thread_allowance(0.25, pitch, &ThreadClass::A3, Some(0.0125));
        assert_eq!(es, 0.0);
    }

    #[test]
    fn test_calc_uts_thread() {
        let n = calc_uts_thread(0.250, 20, &ThreadClass::A2, Some(9));
        println!("{:?}", n);
    }
}
