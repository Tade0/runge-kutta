use std::f64;
extern crate rustfft;

mod solver;

fn main() {
    let mut y = f64::consts::PI - 0.1;
    for x in 0..1000 {
        y = y + solver::runge_kutta4( &test, x as f64, y, 0.01 );
        println!( "{:?}", y );
    }
}

fn test( x: f64, y: f64 ) -> f64 {
    -f64::sin( y )
}