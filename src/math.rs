#[inline]
pub fn norm(v: &[f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

#[inline]
pub fn minus(v1: &[f64; 3], v2: &[f64; 3]) -> [f64; 3] {
    [v1[0] - v2[0], v1[1] - v2[1], v1[2] - v2[2]]
}

#[inline]
pub fn plus(v1: &[f64; 3], v2: &[f64; 3]) -> [f64; 3] {
    [v1[0] + v2[0], v1[1] + v2[1], v1[2] + v2[2]]
}

#[inline]
pub fn scale(v: &[f64; 3], s: f64) -> [f64; 3] {
    [v[0] * s, v[1] * s, v[2] * s]
}

#[inline]
pub fn normalize(v: &[f64; 3]) -> [f64; 3] {
    let n = norm(v);
    [v[0] / n, v[1] / n, v[2] / n]
}

#[inline]
pub fn dot(v1: &[f64; 3], v2: &[f64; 3]) -> f64 {
    v1[0] * v2[0] + v1[1] * v2[1] + v1[2] * v2[2]
}

#[inline]
pub fn cross(v1: &[f64; 3], v2: &[f64; 3]) -> [f64; 3] {
    [
        v1[1] * v2[2] - v1[2] * v2[1],
        v1[2] * v2[0] - v1[0] * v2[2],
        v1[0] * v2[1] - v1[1] * v2[0],
    ]
}

#[inline]
pub fn color_multiply(c: &[u8; 4], s: f64) -> [u8; 4] {
    [
        (c[0] as f64 * s) as u8,
        (c[1] as f64 * s) as u8,
        (c[2] as f64 * s) as u8,
        c[3],
    ]
}
