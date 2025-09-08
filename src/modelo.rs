use std::f64::consts::E;

/// Calcula el peso de un organismo con la función Gompertz.
/// t = edad (días)
/// A = peso máximo
/// B = tasa de crecimiento
/// M = tiempo de inflexión
pub fn gompertz(t: f64, a: f64, b: f64, m: f64) -> f64 {
    a * (- (b * (m - t)).exp()).exp()
}
