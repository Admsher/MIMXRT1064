//! build.rs

use imxrt_rt::{Family, RuntimeBuilder};

fn main() {
    // The iMXRT1010EVK has 16 MiB of flash.
    RuntimeBuilder::from_flexspi(Family::Imxrt1064, 4 * 1024 * 1024)
        .build()
        .unwrap();
}
