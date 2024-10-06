/// Represents the different thread classes (1A, 2A, and 3A) for external threads.
///
/// - `Class1A`: Loose fit (with allowance).
/// - `Class2A`: General fit (with allowance).
/// - `Class3A`: Precision fit (no allowance).
enum Class {
    A1,
    A2,
    A3,
}

impl Class {
    fn allowance(&self) -> f64 {
        match *self {
            Class::A1 | Class::A2 => 0.3,
            Class::A3 => 0.0,
        }
    }
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
/// # Example:
/// ```rust
/// let allowance = calc_uts_thread_allowance(0.5, 1.0 / 28.0, Class::A2, None);
/// println!("Allowance: {}", allowance);
/// ```
fn calc_uts_thread_allowance(d: f64, p: f64, class: Class, le: Option<f64>) -> f64 {
    let le = le.unwrap_or(5.0 * p);
    let k1 = 0.0015;
    let k2 = 0.015;
    class.allowance() * (k1 * d.cbrt() + k1 * le.sqrt() + k2 * p.powi(2).cbrt())
}


fn calc_uts_thread(dia: f64, tpi: i32, class: Class, le: Option<f64>) -> f64 {
    let p = 1.0 / tpi as f64;
    let es = calc_uts_thread_allowance(dia, p, class, le);

    es
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let pitch = 1.0 / 28.0;
        println!("{:?}", calc_uts_thread_allowance(0.5, pitch, Class::A2, Some(0.4)));
    }
}