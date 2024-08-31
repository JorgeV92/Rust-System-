use std::f64::consts::PI;
use num_complex::Complex;


fn fft(A: &mut [Complex<f64>], invert: bool) {
    let n = A.len();
    if n == 1 {
        return 
    }

   let mut U: Vec<Complex<f64>> = Vec::with_capacity(n / 2);
   let mut V: Vec<Complex<f64>> = Vec::with_capacity(n / 2);

    for i in 0.. n/2 {
        U.push(A[2*i]); // even
        V.push(A[2*i+1]); // odd
    }
    fft(&mut U, invert);
    fft(&mut V, invert);

    let value = if invert {-1.0} else {1.0};
    let ang :f64 = 2.0 * PI / n as f64 * value;

    let mut w :Complex<f64> = Complex::new(1.0, 0.0);
    let wn :Complex<f64> = Complex::new(ang.cos(), ang.sin());

    for i in 0.. n/2 {
        A[i] = U[i] + w * V[i];
        A[i + n/2] = U[i] - w * V[i];
        if invert {
            A[i] /= 2.0;
            A[i + n/2] /= 2.0;
        }
        w *= wn;
    }

}

fn main() {
    println!("Fast Fourier Transforms");
    let mut A = vec![
        Complex::new(0.0, 0.0),
        Complex::new(1.0, 0.0),
        Complex::new(2.0, 0.0),
        Complex::new(3.0, 0.0),
        Complex::new(4.0, 0.0),
        Complex::new(5.0, 0.0),
        Complex::new(6.0, 0.0),
        Complex::new(7.0, 0.0),
    ];
    
    fft(&mut A, false);
    println!("{:?}", A);
}
