/*This file is part of Tabularlib.

Tabularlib is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

Tabularlib is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with Tabularlib. If not, see <https://www.gnu.org/licenses/>. */

use crate::interp;
impl interp::Matr {

/* Method to fill a linearly indexed cell in the matrix. */
/* Accepts the target value and the desired index. */
    pub fn fill(&mut self, datau:i32, index: usize) {
        self.data[index] = datau;
    }

/* Method to fill a quadratically indexed cell in the matrix. */
/* Accepts the target value and the coordinates for the desired cell. */
    pub fn fillmn(&mut self, datau: i32, m:usize, n:usize) {
        let lin = self.getlin(m, n);
        self.fill(datau, lin);
    }

/* Method to get value from a linearly indexed cell in the matrix. */
/* Accepts the linear index of the desired cell. */
    pub fn peek(&self, index: usize) -> i32 {
        self.data[index]
    }

/* Method to get value from a quadratically indexed cell in the matrix. */
/* Accepts coordinates of the desired cell. */
    pub fn peekmn(&self, m: usize, n: usize) -> i32{
        let lin = self.getlin(m, n);
        self.getval(lin)
    }

/* A lot of these are just for pure user-friendliness. You can not include them, if you don't want them, and use the Matr.data property and methods in interp.rs directly instead. :) */
}