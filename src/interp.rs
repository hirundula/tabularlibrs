/*This file is part of Tabularlib.

Tabularlib is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

Tabularlib is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with Tabularlib. If not, see <https://www.gnu.org/licenses/>. */

/* This is the core module of Tabularlib. It's used to receive data about the abstract matrix's height and width, as well as interpret abstract quadratic coordinates into the data vector's linear indexation. */

/* Struct definition for the abstract matrix. Backbone of the library. */

use std::collections::HashMap;
pub struct Matr {
    pub i: usize,
    pub j: usize,
    pub data: Vec<i32>,
}

impl Matr {

    /* User-friendly object definition method. No need to write all the property names. */
    /* Accepts the desired matrix height and width. */
    pub fn def(i: usize, j: usize) -> Self {
        Self {
            i,
            j,
            data: vec![0; i * j],
        }
    }

    /* Method to transform the linear vector indexation of a cell into its 2d m * n coordinates, where m are the rows, and n are the columns. */
    /* Accepts the target's linear index and returns a tuple with the 2d coordinates. */
    pub fn getmn(&self, targ: usize) -> [usize; 2] {
        let m = targ / self.i;
        let n = targ % self.i;
        //println!("{} {}", m, n);
        [m, n]
    }

    /* Method to transform the 2d m * n coordinates of a cell into its linear indexation to make it machine-readable. */
    /* Accepts two values of the row and column the object is positioned in and returns the target's linear index. */
    pub fn getlin(&self, m: usize, n: usize) -> usize {
        self.i * m + n
    }

    /* Method to fill a linearly indexed cell in the matrix. */
    /* Accepts the target value and the desired index. */
    pub fn set(&mut self, datau: i32, index: usize) {
        self.data[index] = datau;
    }

    /* Method to fill a quadratically indexed cell in the matrix. */
    /* Accepts the target value and the coordinates for the desired cell. */
    pub fn setmn(&mut self, datau: i32, m: usize, n: usize) {
        self.set(datau, self.getlin(m, n));
    }

    /* Method to get value from a linearly indexed cell in the matrix. */
    /* Accepts the linear index of the desired cell. */
    pub fn peek(&self, index: usize) -> i32 {
        self.data[index]
    }

    /* Method to get value from a quadratically indexed cell in the matrix. */
    /* Accepts coordinates of the desired cell. */
    pub fn peekmn(&self, m: usize, n: usize) -> i32{
        self.peek(self.getlin(m, n))
    }
    pub fn slicerow(&self, row: usize) -> &[i32] {
        let mut output: &[i32]  = &[];
        for i in 0..self.i-1 {
            if i == row{
                output = &self.data[self.getlin(row, 0)..self.getlin(row, self.j+1)]
            }
        }
        output
    }
    pub fn mapcol(&self, col: usize) -> HashMap<usize, &i32> {
        let mut col_map = HashMap::new();
        for j in 0..self.j {
            if j == col {
                for i in 0..self.i-1 {
                    println!("{}", self.getlin(i, j));
                    col_map.insert(self.getlin(i, j), &self.data[self.getlin(i, j)]);
                }
                return col_map
            }
        }
        panic!()
    }
    pub fn build(&self) -> Vec<&[i32]> {
        let mut slicestore = Vec::new();
        for i in 0..self.j {
            slicestore.push(self.slicerow(i))
        }
        slicestore
    }
}