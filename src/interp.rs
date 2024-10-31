/*This file is part of Tabularlib.

Tabularlib is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

Tabularlib is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with Tabularlib. If not, see <https://www.gnu.org/licenses/>. */

/* This is the core module of Tabularlib. It's used to receive data about the abstract matrix's height and width, as well as interpret abstract quadratic coordinates into the data vector's linear indexation. */

/* Struct definition for the abstract matrix. Backbone of the library. */

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
        let n = targ % self.j;
        [m, n]
    }

/* Method to transform the 2d m * n coordinates of a cell into its linear indexation to make it machine-readable. */
/* Accepts two values of the row and column the object is positioned in and returns the target's linear index. */
    pub fn getlin(&self, m: usize, n: usize) -> usize {
        self.i * m + n
    }
}