// #[cfg(any(feature = "esp32", feature = "esp32s2"))]
// macro_rules! soc_address_in_bus {
//     ($bus_name:ident, $vaddr:ident) => {{
//         (concat_idents!($bus_name, _ADDRESS_LOW)..concat_idents!($bus_name, _ADDRESS_HIGH))
//             .contains(&$vaddr)
//     }};
// }

#[inline(always)]
fn soc_address_in_bus(range: BusRange, vaddr: u32) -> bool {
    (range.low..=range.high).contains(&vaddr)
}

#[derive(Copy, Clone)]
struct BusRange {
    low: u32,
    high: u32,
}

#[cfg(feature = "esp32")]
mod soc {
    use crate::mmu_ll::{BusRange, SOC_DRAM1_CACHE_ADDRESS_HIGH, SOC_DRAM1_CACHE_ADDRESS_LOW, SOC_DROM0_CACHE_ADDRESS_HIGH, SOC_DROM0_CACHE_ADDRESS_LOW, SOC_IRAM0_CACHE_ADDRESS_HIGH, SOC_IRAM0_CACHE_ADDRESS_LOW, SOC_IRAM1_CACHE_ADDRESS_HIGH, SOC_IRAM1_CACHE_ADDRESS_LOW, SOC_IROM0_CACHE_ADDRESS_HIGH, SOC_IROM0_CACHE_ADDRESS_LOW};

    pub(crate) const DROM0: BusRange = BusRange {
        low: SOC_DROM0_CACHE_ADDRESS_LOW,
        high: SOC_DROM0_CACHE_ADDRESS_HIGH,
    };

    pub(crate) const IRAM0: BusRange = BusRange {
        low: SOC_IRAM0_CACHE_ADDRESS_LOW,
        high: SOC_IRAM0_CACHE_ADDRESS_HIGH,
    };

    pub(crate) const IRAM1: BusRange = BusRange {
        low: SOC_IRAM1_CACHE_ADDRESS_LOW,
        high: SOC_IRAM1_CACHE_ADDRESS_HIGH,
    };

    pub(crate) const IROM0: BusRange = BusRange {
        low: SOC_IROM0_CACHE_ADDRESS_LOW,
        high: SOC_IROM0_CACHE_ADDRESS_HIGH,
    };

    pub(crate) const DRAM1: BusRange = BusRange {
        low: SOC_DRAM1_CACHE_ADDRESS_LOW,
        high: SOC_DRAM1_CACHE_ADDRESS_HIGH,
    };
}

#[cfg(feature = "esp32")]
mod esp32;
#[cfg(feature = "esp32")]
pub use esp32::*;

#[cfg(feature = "esp32s2")]
mod esp32s2;
#[cfg(feature = "esp32s2")]
pub use esp32s2::*;

#[cfg(feature = "esp32s3")]
mod esp32s3;
#[cfg(feature = "esp32s3")]
pub use esp32s3::*;

#[cfg(feature = "esp32c2")]
mod esp32c2;
#[cfg(feature = "esp32c2")]
pub use esp32c2::*;

#[cfg(feature = "esp32c3")]
mod esp32c3;
#[cfg(feature = "esp32c3")]
pub use esp32c3::*;

#[cfg(feature = "esp32c6")]
mod esp32c6;
#[cfg(feature = "esp32c6")]
pub use esp32c6::*;

#[cfg(feature = "esp32h2")]
mod esp32h2;
#[cfg(feature = "esp32h2")]
pub use esp32h2::*;

#[cfg(not(any(
    feature = "esp32",
    feature = "esp32s2",
    feature = "esp32s3",
    feature = "esp32c2",
    feature = "esp32c3",
    feature = "esp32c6",
    feature = "esp32h2"
)))]
mod not_selected;

#[cfg(not(any(
    feature = "esp32",
    feature = "esp32s2",
    feature = "esp32s3",
    feature = "esp32c2",
    feature = "esp32c3",
    feature = "esp32c6",
    feature = "esp32h2"
)))]
pub use not_selected::*;
