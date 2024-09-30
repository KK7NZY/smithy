use crate::types::Coord;

/// Calculates the positions of points on a bolt circle pattern.
///
/// This function computes the (x, y) coordinates of points evenly spaced around
/// a circle, using the provided diameter and number of points. It optionally
/// takes starting angle, and center coordinates for the circle.
///
/// # Parameters
///
/// - `dia`: Diameter of the bolt circle.
/// - `num`: Number of points to calculate.
/// - `st_angle`: Optional starting angle in degrees (default is 0).
/// - `xc`: Optional x-coordinate for the center of the circle (default is 0.0).
/// - `yc`: Optional y-coordinate for the center of the circle (default is 0.0).
///
/// # Returns
///
/// Returns an iterator that yields `Coord` values containing the x, y coordinates and the angle
/// for each point.
///
/// # Example
///
/// ```rust
/// // Example usage
/// ```
pub fn calc_bolt_circle(
    dia: f64,
    num: u32,
    st_angle: Option<f64>,
    xc: Option<f64>,
    yc: Option<f64>,
) -> impl Iterator<Item = Coord> {
    let st_angle = st_angle.unwrap_or_default();
    let xc = xc.unwrap_or_default();
    let yc = yc.unwrap_or_default();
    let step = 360.0 / num as f64;
    let rd = dia / 2.0;
    (0..num).map(move |i| {
        let ang = (st_angle + i as f64 * step).to_radians();
        let x = xc + rd * ang.cos();
        let y = yc + rd * ang.sin();
        Coord {
            x,
            y,
            z: None,
            angle: Some(ang.to_degrees()),
        }
    })
}

/// Calculates evenly spaced points between a start and end value.
///
/// This function generates an iterator of evenly spaced `f64` values starting from the given
/// `start` value and ending just before or at the `end` value. The spacing between values
/// is determined by the `step` parameter.
///
/// # Parameters
///
/// - `start`: The starting value for the spacing.
/// - `end`: The end limit for the spacing. The iterator will stop at or just before this value.
/// - `step`: The step size between each value in the sequence.
///
/// # Returns
///
/// Returns an iterator of `f64` values that are evenly spaced between the start and end values.
///
/// # Example
///
/// ```rust
/// use smithy::layout::calc_linear_spacing;
/// let points: Vec<_> = calc_linear_spacing(0.0, 5.0, 1.0).collect();
/// assert_eq!(points, [0.0, 1.0, 2.0, 3.0, 4.0, 5.0]);
/// ```
pub fn calc_linear_spacing(start: f64, end: f64, step: f64) -> impl Iterator<Item = f64> {
    (0..)
        .map(move |i| step * i as f64 + start)
        .take_while(move |&v| v <= end)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::round;

    #[test]
    fn test_calc_bolt_circle() {
        let actual = calc_bolt_circle(6.0, 5, Some(20.0), None, None)
            .map(|p| (round(p.angle.unwrap(), 1), round(p.x, 4), round(p.y, 4)))
            .collect::<Vec<_>>();
        let expected = vec![
            (20.0, 2.8191, 1.0261),
            (92.0, -0.1047, 2.9982),
            (164.0, -2.8838, 0.8269),
            (236.0, -1.6776, -2.4871),
            (308.0, 1.847, -2.364),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_calc_linear_spacing() {
        let start = 0.5;
        let end = 11.5;
        let actual = calc_linear_spacing(start, end, (end - start) / 4.0)
            .map(|v| round(v, 3))
            .collect::<Vec<_>>();
        let expected = vec![0.5, 3.25, 6.0, 8.75, 11.5];
        assert_eq!(actual, expected);
    }
}
