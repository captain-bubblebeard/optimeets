use ndarray::prelude::*;

fn main() {
    let mut a: Array2<i32> = Array::zeros((2,2));

    for i in 0..a.nrows() {
        for j in 0..a.ncols() {
            a[[i,j]] += 1;
        }
    }

    println!("{:?}", a);
}
