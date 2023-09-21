#[derive(Debug, Default)]
struct Matrix<const R : usize, const C : usize> {
    rows : [[f32; C]; R]
}

impl<const R : usize, const C : usize> Matrix<R, C> {
    
    fn elem(&self, r : usize, c : usize) -> f32 {
        self.rows[r][c]; //how to access element in 2D array???

    fn rows(&self) {
        self.rows.clone().into_iter()
    }

    fn transpose(&self) -> Matrix<C, R> {
        transpose = Matrix<C, R>::default();
        
        for row in self.rows() {
            for column in row {
                transpose.elem = self.rows[column]; 
            }
        }
    }

    fn num_rows(&self) -> usize {
        self.rows.len()
    }

    fn num_columns(&self) -> usize {
        self.rows[0].len()
    }

    //fn scale_row(

    fn row_echelon(&self) /* -> Matrix<R, C> */ {
       println!("{}", format!("{self:?}"));

       // for column in 0..(num_columns - 1) {
            

    }

}


fn main() {

    let A : Vec<Vec<f32>> = vec!(
        vec!(2.0, 5.0, 7.0),
        vec!(3.0, 3.0, 9.0),
        vec!(4.0, 8.0, 1.0)
    );

    let b : Vec<f32> = vec!(6.0, 1.0, 1.0);

    let matrix = Matrix { rows : A };

    matrix.row_echelon();
}
