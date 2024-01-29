use nalgebra::{DMatrix, Complex};
// goal of this file is to include the output structs after the NEGF calculation
// this means I have to think about exactly what this thing is supposed to output... uggg
// at first pass, I think it makes sense to output the current matrix and the transmission matrix

#[derive(Debug)]
pub struct TransmissionMatrix(pub Vec<DMatrix<Complex<f64>>>);

#[derive(Debug)]
pub struct CurrentMatrix(pub Vec<DMatrix<Complex<f64>>>);
