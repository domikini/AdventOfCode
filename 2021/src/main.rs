mod ac1_1;
mod ac1_2;
mod ac2_1;
mod ac2_2;
mod ac3_1;
mod ac3_2;
mod ac4_1;
mod ac4_2;

use std::mem;
use crate::ac1_1::ac1_1;
use crate::ac1_2::ac1_2;
use crate::ac2_1::ac2_1;
use crate::ac2_2::ac2_2;
use crate::ac3_1::ac3_1;
use crate::ac3_2::ac3_2;
use crate::ac4_1::ac4_1;
use crate::ac4_2::ac4_2;

fn main() {
    ac1_1();
    ac1_2();
    ac2_1();
    ac2_2();
    ac3_1();
    ac3_2();
    ac4_1();
    ac4_2();
}

