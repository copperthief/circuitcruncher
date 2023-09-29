use std::fmt;
use fraction::Fraction;

#[derive(Debug, Clone)]        
struct Matrix<const R : usize, const C : usize> {
    rows : [[Fraction; C]; R]
}

impl<const R : usize, const C : usize> fmt::Display for Matrix<R, C> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut diagram = "".to_owned();

        for r in 1..=R {
            for c in 1..=C {
                diagram = diagram + &self.get(r, c).to_string() + " ";
            }
            diagram = diagram + "\n";
        }

        write!(f, "{}", diagram)
    }
}

impl<const R : usize, const C : usize> Matrix<R, C> {
    
    fn new(rows : [[Fraction; C]; R]) -> Matrix<R, C> {
        Matrix { rows }
    }

    fn get(&self, r : usize, c : usize) -> Fraction {
        self.rows[r - 1][c - 1] //how to access element in 2D array
    }

    fn set(&mut self, r : usize, c : usize, f : Fraction) {
        self.rows[r - 1][c - 1] = f;
    }

    fn scale_row(&self, row : usize, scalar : Fraction) -> Matrix<R, C> {
        let mut scaled = self.clone();

        for c in 1..=C {
            scaled.set(row, c, scalar * self.get(row, c));
        }

        scaled
    }

    fn add_rows(&self, scalar : Fraction, addend_row : usize, addee_row : usize) -> Matrix<R, C> {
        let mut result = self.clone();

        for c in 1..=C {
            result.set(addee_row, c, result.get(addee_row, c) + scalar * self.get(addend_row, c));
        }

        result
    }


    fn row_echelon(&self) -> Matrix<R, C> {
        let mut result = self.clone();

        for c in 1..C {

            let mut pivot = 0;
            for r in 1..=R {
                if result.get(r, c) != Fraction::from(0) {
                    let mut pivot_in_previous_column = false;
                    for x in 1..c {
                        if result.get(r, x) != Fraction::from(0) { pivot_in_previous_column = true; }
                    }
                    if !pivot_in_previous_column {
                    pivot = r;
                        break;
                    }
                }
            }

            if pivot == 0 { continue; }

            for r in 1..=R {
                if r != pivot {
                    result = result.add_rows( -result.get(r, c)/result.get(pivot, c), pivot, r);
                }
            }
        }
        
        for r in 1..=R {
            let mut scalar = Fraction::from(1);
            for c in 1..=C {
                let s = result.get(r, c);
                if s != Fraction::from(0) {
                    scalar = Fraction::from(1) / s;
                    break;
                }
            }
            result = result.scale_row(r, scalar);
        } 

        result

    }

}


fn main() {
    
    let a1 = [Fraction::from(-3), Fraction::from(1), Fraction::from(2), Fraction::from(0)];
    let a2 = [Fraction::from(1), Fraction::from(-6), Fraction::from(3), Fraction::from(0)];
    let a3 = [Fraction::from(2), Fraction::from(3), Fraction::from(-6), Fraction::from(0)];

    let A = [a1, a2, a3];

    let matrix = Matrix::new(A);

    let echeloned = matrix.row_echelon();

    println!("{}", matrix);
    println!("{}", echeloned);

}
