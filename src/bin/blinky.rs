#![no_main]
#![no_std]

#[allow(unused_imports)]
use panic_halt;

use rtfm::app;

use nrf52840_hal::{clocks, gpio, prelude::*};

use nrf52840_pac as pac;

#[app(device = nrf52840_pac)]
const APP: () = {
    static mut TIMER: pac::TIMER0 = ();
    static mut LED_1: gpio::Pin<gpio::Output<gpio::PushPull>> = ();
    static mut LED_2: gpio::Pin<gpio::Output<gpio::PushPull>> = ();
    static mut STATE: bool = false;

    #[init]
    fn init() {
        let p0 = gpio::p0::Parts::new(device.P0);
        // Configure to use external clocks, and start them
    
        clocks::Clocks::new(device.CLOCK)
            .enable_ext_hfosc()
            .set_lfclk_src_external(clocks::LfOscConfiguration::NoExternalNoBypass)
            .start_lfclk();
        let timer = device.TIMER0;
        timer.mode.write(|w| w.mode().timer());
        timer.bitmode.write(|w| w.bitmode()._32bit());
        timer.shorts.write(|w| w.compare0_clear().enabled().compare0_stop().disabled());
        timer.prescaler.write(|w| unsafe { w.prescaler().bits(4) });
        timer.cc[0].write(|w| unsafe { w.bits(250_000)});
        timer.intenset.write(|w| w.compare0().set());
        timer.tasks_clear.write(|w| w.tasks_clear().set_bit());
        timer.tasks_start.write(|w| w.tasks_start().set_bit());
                             
        TIMER = timer;
        LED_1 = p0.p0_24.degrade().into_push_pull_output(gpio::Level::Low);
        LED_2 = p0.p0_23.degrade().into_push_pull_output(gpio::Level::High);
    }
                                            
    #[interrupt(resources = [LED_1, LED_2, TIMER, STATE],)]
    fn TIMER0() {
       *resources.STATE = !*resources.STATE;
        resources.TIMER.events_compare[0].reset();
                                            
        if *resources.STATE {
            let _ = (*resources.LED_1).set_high();
            let _ = (*resources.LED_2).set_low();
        } else {
            let _ = (*resources.LED_1).set_low();
            let _ = (*resources.LED_2).set_high();
        }
    }
};

