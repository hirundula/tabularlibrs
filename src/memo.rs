/*This file is part of Tabularlib.

Tabularlib is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

Tabularlib is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with Tabularlib. If not, see <https://www.gnu.org/licenses/>. */

/* This module is dedicated to providing temorary storage to matrices. I plan on making custom memory management in the future (when I learn more about that :'D). */

#[macro_export]
macro_rules! matr_createinst {
    ($name: expr, $i: expr, $j: expr) => {
        fn check_exists(&Vec<>) -> Result<(), &static str> {
            Ok(())
        }
        if let Err(e) = check_exists(StructHeap) {
            let StructHeap = Vec::new();
        }
        let mut $name = interp::Matr::def($i, $j)
        StructHeap.push($name)
    }
}