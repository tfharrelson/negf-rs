use std::time::Instant;

use nalgebra::{DMatrix, MatrixView, Complex, SymmetricEigen, Dyn, DVector};

use crate::inputs::FullHamiltonianInput;
use crate::outputs::TransmissionMatrix;

// goal is to create functions that convert inputs to outputs
// let's start with the full hamiltonian representation
const ETA: f64 = 0.0001;

pub fn calculate_transmission_matrix(mat: FullHamiltonianInput) -> TransmissionMatrix {
    // the transmission matrix is first started by finding the full green's function
    // I don't have a better way than finding eigs from the full hamiltonian right now
    // maybe there's a better/more efficient way that doesn't require diagonalizing the H
    let lh = mat.left_hamiltonian();
    let rh = mat.right_hamiltonian();
    let li = mat.left_interaction_matrix();
    let ri = mat.right_interaction_matrix();

    // construct the omega vector, then find the green's matrix for each omega
    let full_green_matrix = calculate_green_function(mat.hamiltonian);

    // now that we have the green's function, we can use it to find the broadening funcs
    // first we need the self energy matrices
    // TODO: need to create an input that has references to all components of the hamiltonian that
    // will need to be used. This will likely come from the BlockForm. The full one is using a
    // bunch of matrix views and references which sort of don't work
    let left_green_matrix = calculate_green_function(lh);
    let right_green_matrix = calculate_green_function(rh);

    // need to get the interaction matrices which connect the left and right leads to the device
    // part of the hamiltonian
    let left_self_energy: Vec<DMatrix<_>> = left_green_matrix
        .into_iter()
        .map(|lg| &li.adjoint() * lg * &li)
        .collect();
    let right_self_energy: Vec<DMatrix<_>> = right_green_matrix
        .into_iter()
        .map(|rg| &ri.adjoint() * rg * &ri)
        .collect();

    // now equipped with the interaction matrices, we can construct the broadening funcs
    // the broadening func is the imaginary part of the self energies
    //println!("left self energy: {:?}", left_self_energy);
    //println!("right self energy: {:?}", right_self_energy);
    println!("self energies calculated!");
    let imag_unit: Complex<f64> = Complex{re: 0., im: 1.};
    let left_broadening_func: Vec<DMatrix<Complex<f64>>> = left_self_energy.into_iter().map(|lse| &lse - &lse.adjoint() * imag_unit).collect();
    let right_broadening_func: Vec<DMatrix<Complex<f64>>> = right_self_energy.into_iter().map(|rse| &rse - &rse.adjoint() * imag_unit).collect();
    //println!("left broadening function for device: {:?}", left_broadening_func);
    //println!("right broadening function for device: {:?}", right_broadening_func);
    println!("broadening funcs calculated!");

    // now we just need to extract the device portions of the full greens matrix, and then we are
    // ready to construct the final transmission matrix across the device
    let device_green: Vec<MatrixView<Complex<f64>, Dyn, Dyn>> = full_green_matrix
        .iter()
        .map(|g| g.view((mat.left_size, mat.left_size), (mat.device_size, mat.device_size)))
        .collect();
    //println!("green function for device: {:?}", device_green.iter().map(|d| DMatrix::from(d)));

    // construct the final transmission matrix and return!
    return TransmissionMatrix ( 
        device_green
        .into_iter()
        .zip(left_broadening_func.into_iter())
        .zip(right_broadening_func.into_iter())
        .map(|((d, l), r)| d.adjoint() * l * d * r)
        .collect()
    )
}


pub fn calculate_green_function(mat: DMatrix<Complex<f64>>) -> Vec<DMatrix<Complex<f64>>> {
    // calculate the symmetric eigendecomp for the matrix -> only do this once!
    println!("diagonalizing hamiltonian...");
    let now = Instant::now();
    let results: SymmetricEigen<Complex<f64>, Dyn> = mat.symmetric_eigen();
    println!("hamiltonian diagonalized! Time taken: {:?}", now.elapsed());

    // smartly construct the omega vector based on the min/max eigvals
    let min_eig = results.eigenvalues.min();
    let max_eig = results.eigenvalues.max();

    // get relevant data to construct a 100 element omega vector
    // TODO: convert the hardcoded 100 into a function argument at some point
    let num_omega: usize = 100;
    let mut omega_vec = Vec::new();
    let mut curr_eig = 0.99 * min_eig;
    let delta_omega = (max_eig - min_eig) / num_omega as f64;

    while curr_eig < 1.01 * max_eig {
        omega_vec.push(curr_eig);
        curr_eig += delta_omega;
    };

    // calculate green's matrix for each omega
    return omega_vec.into_iter().map(|w| calculate_green_matrix(&results.eigenvalues, &results.eigenvectors, w)).collect()
}


pub fn calculate_green_matrix(eigenvalues: &DVector<f64>, eigenvectors: &DMatrix<Complex<f64>>, omega: f64) -> DMatrix<Complex<f64>> {

    // need to invert the eigenvalues to construct the green matrix
    // subtract inv eigval matrix from omega matrix
    let now = Instant::now();
    let inner_diagonal:Vec<Complex<f64>> = eigenvalues.into_iter().map(|e| 1. / (omega - Complex {re: 0., im: 1.} * ETA - e)).collect();

    let inner_matrix: DMatrix<Complex<f64>> = DMatrix::from_diagonal(
        &DVector::from(inner_diagonal)
    );

    // construct the greens matrix from the eigenvector matrix and the inverse eigval matrix
    // the OG matrix is U.G.U^dag where U is the eigvec matrix, and G is the eigval diagonal matrix
    // the inverse is easy from this form: U^dag.G^-1.U where the inverse of G is easy b/c it's
    // diagonal
    let g_matrix = eigenvectors * inner_matrix * eigenvectors.adjoint();
    println!("calculated green's matrix for a given omega! Time taken: {:?}", now.elapsed());
    return g_matrix
}
