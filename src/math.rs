/*This file is part of Tabularlib.

Tabularlib is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

Tabularlib is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with Tabularlib. If not, see <https://www.gnu.org/licenses/>. */

use crate::interp;
use crate::utils;
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
            panic!("Matrices must be of equal order to be summed!")
        }
    }
    pub fn tpos(&self) -> Matr {
        let mut res: Matr = Matr::def(self.j, self.i);
        for i in 0..self.data.len() {
            let targ = self.getmn(i);
            let chk = res.getlin(targ[1], targ[0]);
            res.data[chk] = self.data[i];
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
        if self.i == factor.j {
            let mut res: Matr = Matr::def(factor.i, self.j);
            for targ in 0..res.data.len() {
                res.data[targ] = 0;
                for An in 0..self.j {
                    for Bm in 0..factor.i {
                        let x = res.getmn(targ);
                        res.fillmn(res.data[targ]+self.peekmn(x[0], An)*factor.peekmn(Bm, x[1]), x[0], x[1])
                    }
                }
            }
            return res;
        }
        else {
            panic!("Matrices A and B must have equal numbers of columns and rows respectively to be multiplied!")
        }
    }
}