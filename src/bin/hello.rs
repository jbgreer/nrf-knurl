#![no_main]
#![no_std]

use nrf_knurl as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Hello, world!");

    nrf_knurl::exit()
}
