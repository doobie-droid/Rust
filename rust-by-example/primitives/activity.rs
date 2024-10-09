use std::fmt::{self, Display, Formatter};

// Recap: Add the fmt::Display trait to the Matrix struct in the above example, so that if you switch from printing the debug format {:?} to the display format {}, you see the following output:

// ( 1.1 1.2 )
// ( 2.1 2.2 )

// Add a transpose function using the reverse function as a template, which accepts a matrix as an argument, and returns a matrix in which two elements have been swapped. For example:

// Matrix:
// ( 1.1 1.2 )
// ( 2.1 2.2 )
// Transpose:
// ( 1.1 2.1 )
// ( 1.2 2.2 )

// The following struct is for the activity.
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        return write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3);
    }
}

fn transpose(matrix: Matrix) -> Matrix {
    let Matrix(a, b, c, d) = matrix;
    return Matrix(a, c, b, d);
}
fn main() {
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));
}
