use nalgebra::{storage::Owned, U6};
use rand::{prelude::ThreadRng, Rng};
type Matrix6x6 = nalgebra::Matrix<f64, U6, U6, Owned<f64, U6, U6>>;

fn main() {
    let mut rng = rand::thread_rng();
    let mut mat = rand_mat(&mut rng);

    for _ in 0..500000 {
        let mat1 = rand_mat(&mut rng);
        let mat2 = rand_mat(&mut rng);
        mat += mat1 * mat2;
    }

    println!("{}", mat);
}

fn rand_mat(rng: &mut ThreadRng) -> Matrix6x6 {
    Matrix6x6::from_fn(|_, _| rng.gen())
}
