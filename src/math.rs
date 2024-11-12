/*This file is part of Tabularlib.

Tabularlib is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

Tabularlib is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with Tabularlib. If not, see <https://www.gnu.org/licenses/>. */
use std::iter::zip;
use crate::interp;
use crate::interp::Matr;
impl Matr {
    pub fn sum(&self, addend: &Matr) -> Matr {
        assert_eq!(self.data.len(), addend.data.len());
        let mut res: Matr = Matr::def(self.i, self.j);
        for i in 0..res.data.len() {
            res.data[i] = self.data[i] + addend.data[i];
        }
        return res;
    }
    pub fn tpos(&self) -> Matr {
        let mut res: Matr = Matr::def(self.j, self.i);
        for i in 0..res.i {
            for j in 0..res.j {
                res.setmn(self.peekmn(i, j), j, i);
            }
        }
        res
    }
    pub fn kprod(&self, k: &i32) -> Matr {
        let mut res: Matr = Matr::def(self.i, self.j);
        for i in 0..res.data.len() {
            res.data[i] = self.data[i]*k;
        }
        res
    }
    pub fn prod(&self, factor: &Matr) -> Matr {
        assert_eq!(self.i, factor.j);
        let factor_t = factor.tpos();
        let mut res = Matr::def(self.j, factor.i);
        for i in 0..self.i-1 {
            for j in 0..factor_t.i-1 {
                res.setmn(self.slicerow(i).iter().zip(factor_t.slicerow(j).iter()).map(|(a, b)| a * b).fold(0, |acc, e| acc + e), i, j);
            }
        }
        res
    }
}