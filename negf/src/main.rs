use nalgebra::{Complex, DMatrix};
use rand::Rng;

pub mod inputs;
pub mod outputs;
pub mod utils;

fn main() {
    // build a random symmetric matrix
    let mat = build_random_symmetric_matrix(90);

    // build the main function input
    let input = inputs::FullHamiltonianInput{
        hamiltonian: mat,
        left_size: 30,
        right_size: 30,
        device_size: 30,
    };
    // println!("Hello, world! Here's the input: {:?}", input);

    // get the transmssion matrix from this bad boy
    let t_mat = utils::calculate_transmission_matrix(input);
    // println!("T matrix is {:?}", t_mat);
    println!("Transmission function calculation complete!");
}

fn random_complex_number() -> Complex<f64> {
    let mut rng = rand::thread_rng();
    let c: Complex<f64> = Complex::new(
        rng.gen::<f64>(), rng.gen::<f64>()
    );
    c
}

// I still don't know how to build a statically typed matrix of a known size at compile time...
// This feels very weird, but this is fine for now, but I should try to figure it out later
fn build_random_symmetric_matrix(length: usize) -> DMatrix<Complex<f64>> {
    // no idea what 'S' is for still, so I should figure that out at some point
    // T is the data type, R is the row number, C is the column number.
    // weird that these are generic... not sure why you would want something other than an integer
    // number of rows/columns but whatever...
    let mut mat = DMatrix::zeros(length, length);
    for i in 0..length {
        for j in i..length {
            let mat_value = random_complex_number();
            mat[(i, j)] = mat_value;
            if i != j {
                mat[(j, i)] = mat_value;
            }
        }
    };

    // return the populated matrix
    mat
}
