pub fn runge_kutta4( fx: &Fn(f64, f64) -> f64, x: f64, y: f64, h: f64 ) -> f64 {
    let k1 = h * fx( x, y );
    let k2 = h * fx( x + h / 2.0, y + k1 / 2.0 );
    let k3 = h * fx( x + h / 2.0, y + k2 / 2.0 );
    let k4 = h * fx( x + h, y + k3 );

    ( k1 + 2.0 * k2 + 2.0 * k3 + k4 ) / 6.0
}
