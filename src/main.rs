use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;

#[derive(Debug,Clone)]
struct Shape {
    pub color: u8,
    pub data: [[u8;3];3],
}

impl Display for Shape {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let d = &self.data;
        write!(f, "{} {} {} ({})\n{} {} {} \n{} {} {}\n",
               d[0][0],d[0][1],d[0][2], self.color,
               d[1][0],d[1][1],d[1][2],
               d[2][0],d[2][1],d[2][2])
    }
}

impl Shape {
    fn new (color: u8, data: [[u8;3];2]) -> Shape {
        Shape { color, data: [data[0],data[1],[0,0,0]]}
    }

    fn flip(&self) -> Shape {
        let d = &self.data;
        assert!(d[2][0]==0 && d[2][1]==0 && d[2][2]==0);
        let mut s = Shape {
            color: 1 - self.color,
            data: d.clone()
        };
        for i in 0..3 {
            s.data[0][i] = (d[1][i] & 1) | ((d[0][i] & 1) << 1);
            s.data[1][i] = ((d[1][i] & 2) >> 1) | (d[0][i] & 2);
        }

        s
    }
    fn rotate(&self) -> Shape {
        let d = &self.data;
        Shape {
            color: self.color,
            data:[
                [d[2][0],d[1][0],d[0][0]],
                [d[2][1],d[1][1],d[0][1]],
                [d[2][2],d[1][2],d[0][2]],
            ]
        }
    }

    fn create_group(&self) -> Vec<Shape> {
        let mut v  = Vec::new();
        let mut s = self.clone();
        for _ in 0..4 {
            let mut z = s.clone();
            for _ in 0..4 {
                v.push(z.clone());
                z = z.rotate();
            }
            s = s.flip();
        }
        v
    }
}

#[derive(Debug)]
struct Assembly {
    data : [[u8;9];9]
}

impl Assembly {
    fn new(data : [[u8;9];9]) -> Assembly {
        Assembly { data }
    }

    fn can_place(&self, row: usize, col: usize, shape: &Shape) -> bool {
        if ((row + col)&1) as u8 != shape.color {
            return false;
        }
        for i in 0..3 {
            for j in 0..3 {
                if (self.data[row+i][col+j] & shape.data[i][j])!=0 {
                    return false;
                }
            }
        }
        true
    }

    fn place(&mut self, row: usize, col: usize, shape: &Shape) {
        for i in 0..3 {
            for j in 0..3 {
                let cell = &mut self.data[row+i][col+j];
                assert!((*cell & shape.data[i][j])==0);
                *cell |= shape.data[i][j];
            }
        }
    }

    fn unplace(&mut self, row: usize, col: usize, shape: &Shape) {
        for i in 0..3 {
            for j in 0..3 {
                let cell = &mut self.data[row+i][col+j];
                assert!((*cell & shape.data[i][j])==shape.data[i][j]);
                *cell &= shape.data[i][j]^3;
            }
        }
    }

    fn find_most_constrained_cell(&self) -> (usize,usize,u8) {
        let mut r = (0,0,0);
        let mut max = -1;
        for i in 2..7 {
            for j in 2..7 {
                for &z in &[1, 2] {
                    if (self.data[i][j] & z) == 0 {
                        let count =
                            hamming_weight(self.data[i][j] ^ z)
                                + hamming_weight(self.data[i - 1][j])
                                + hamming_weight(self.data[i + 1][j])
                                + hamming_weight(self.data[i][j - 1])
                                + hamming_weight(self.data[i][j + 1]);
                        if count > max {
                            max = count;
                            r = (i, j, z);
                        };
                    }
                }
            }
        }
        r
    }
}
impl Display for Assembly {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for row in &self.data {
            for cell in row {
                write!(f, "{} ", cell)?;
            }
            write!(f, "\n")?;
        }
        Result::Ok(())
    }

}

fn hamming_weight(x : u8) -> i32 {
    ((x & 1) + (x >> 1)) as i32
}

fn backtrack(asm: &mut Assembly, shape_groups: &Vec<Vec<Shape>>,
             used_shapes: &mut Vec<bool>,
             solution: &mut Vec<(usize,usize,usize,usize)>) -> bool {
    let (row, col, z) = asm.find_most_constrained_cell();
    if z==0 {
        return true;
    }
    for sgi in 0..used_shapes.len() {
        if !used_shapes[sgi] {
            for si in 0..shape_groups[sgi].len() {
                let s = &shape_groups[sgi][si];
                for i in 0..3 {
                    for j in 0..3 {
                        if (s.data[i][j] & z)!=0 {
                            if asm.can_place(row-i,col-j,s) {
                                solution.push((sgi,si, row-i, col-j));
                                asm.place(row-i, col-j, s);
                                used_shapes[sgi] = true;
                                let r = backtrack(asm, shape_groups, used_shapes, solution);
                                used_shapes[sgi] = false;
                                asm.unplace(row-i, col-j, s);
                                if r { return true; }
                                solution.pop();
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

fn main() {
    let shapes = vec![
        Shape::new(0, [
            [1, 1, 1],
            [0, 3, 0]]),
        Shape::new(0, [
            [3, 1, 1],
            [2, 0, 0]]),
        Shape::new(1, [
            [2, 0, 0],
            [3, 1, 1]]),
        Shape::new(1, [
            [3, 1, 0],
            [0, 1, 1]]),
        Shape::new(1, [
            [0, 0, 1],
            [1, 1, 3]]),
        Shape::new(1, [
            [1, 1, 3],
            [0, 1, 0]]),
        Shape::new(0, [
            [0, 1, 0],
            [2, 3, 1]]),
        Shape::new(1, [
            [1, 0, 0],
            [1, 3, 1]]),
        Shape::new(0, [
            [0, 1, 3],
            [1, 1, 0]]),
        Shape::new(1, [
            [1, 3, 0],
            [0, 1, 1]]),
    ];

    let mut asm = Assembly::new([
        [3, 3, 3, 3, 3, 3, 3, 3, 3],
        [3, 3, 3, 3, 3, 3, 3, 3, 3],
        [3, 3, 0, 0, 0, 0, 0, 3, 3],
        [3, 3, 0, 0, 0, 0, 0, 3, 3],
        [3, 3, 0, 0, 0, 0, 0, 3, 3],
        [3, 3, 0, 0, 0, 0, 0, 3, 3],
        [3, 3, 0, 0, 0, 0, 0, 3, 3],
        [3, 3, 3, 3, 3, 3, 3, 3, 3],
        [3, 3, 3, 3, 3, 3, 3, 3, 3],
    ]);

    let shape_groups : Vec<Vec<Shape>> = shapes.iter().map(Shape::create_group).collect();

    let mut used_shapes : Vec<bool> = shapes.iter().map(|_|false).collect();

    let mut solution = Vec::new();
    if backtrack(&mut asm, &shape_groups, &mut used_shapes, &mut solution) {
        println!("{}", asm);
        for (sgi,si,row,col) in solution {
            println!("{}",shape_groups[sgi][si]);
            asm.place(row,col, &shape_groups[sgi][si]);
            println!("{}",asm);
        }
    } else {
        println!("No solution");
    }
}
