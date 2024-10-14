/*This file is part of Tabularlib.

Tabularlib is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

Tabularlib is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with Tabularlib. If not, see <https://www.gnu.org/licenses/>. */

use crate::interp;
use crate::interp::Matr;
impl Matr {
    pub fn sum(&self, addend: &Matr) -> Matr {
        if self.data.len() == addend.data.len() {
            let mut res: Matr = Matr::def(self.i, self.j);
            for i in 0..res.data.len() {
                res.data[i] = self.data[i] + addend.data[i];
            }
            return res;
        }
        else {
            panic!("Matrices must be of equal order to be summed.")
        }
    }
    pub fn tpos(&self) -> Matr {
        let mut res: Matr = Matr::def(self.i, self.j);
        for i in 0..self.data.len() {
            let targ = self.getmn(i);
            let tm = targ[1];
            let tn: usize = targ[0];
            res.data[res.getlin(tm, tn)] = self.data[i];
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
    pub fn prod(&self, factor: &Matr) {
        if self.i == factor.j {
            let mut res: Matr = Matr::def(factor.i, self.j);
            for i in res.i {
                let mut s:usize = 0;
                for j in 0..self.data.len() {
                    if j % self.j == 0 {
                        for k in s..j-1 {
                            //variable += self * factor
                        }
                        s = j-1;
                    }
                }
            }
        }
    }
}