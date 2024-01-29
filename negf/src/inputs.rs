extern crate nalgebra as na;
use na::{Complex, DMatrix};

// create structs for all valid inputs into the NEGF method
// these will include a full hamiltonian matrix along with sizes for leads and device partitions
// also include sub-matrix specification directly: folks can pass in left/right hamiltonians,
// interaction hamiltonians, and device hamiltonian

#[derive(Debug, Clone)]
pub struct FullHamiltonianInput {
    pub hamiltonian: DMatrix<Complex<f64>>,
    pub left_size: usize,
    pub right_size: usize,
    pub device_size: usize,
}

impl FullHamiltonianInput {
    pub fn left_interaction_matrix(&self) -> DMatrix<Complex<f64>>  {
        return DMatrix::from(self.hamiltonian.view((0, self.left_size), (self.left_size, self.device_size)));
    }
    pub fn right_interaction_matrix(&self) -> DMatrix<Complex<f64>> {
        return DMatrix::from(self.hamiltonian.view(
            (self.left_size, self.left_size + self.device_size), 
            (self.device_size, self.right_size)
        ));
    }
    pub fn left_hamiltonian(&self) -> DMatrix<Complex<f64>> {
        return DMatrix::from(self.hamiltonian.view((0, 0), (self.left_size, self.left_size)));
    }
    pub fn right_hamiltonian(&self) -> DMatrix<Complex<f64>> {
        let start = self.left_size + self.device_size;
        return DMatrix::from(self.hamiltonian.view((start, start), (self.right_size, self.right_size)));
    }
    pub fn device_hamiltonian(&self) -> DMatrix<Complex<f64>> {
        let start = self.left_size;
        return DMatrix::from(self.hamiltonian.view((start, start), (self.device_size, self.device_size)));
    }
}

// separate specifications hamiltonian blocks
#[derive(Debug, Clone)]
pub struct BlockHamiltonianInput {
    pub device: DMatrix<Complex<f64>>,
    pub left_lead: DMatrix<Complex<f64>>,
    pub right_lead: DMatrix<Complex<f64>>,
    pub left_interaction: DMatrix<Complex<f64>>,
    pub right_interaction: DMatrix<Complex<f64>>,
}
