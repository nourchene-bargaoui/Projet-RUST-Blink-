#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate panic_abort;

extern crate stm32f429i_disc as board;
extern crate embedded_hal as hal;

use cortex_m_rt::entry;

use board::hal::delay::Delay;
use board::hal::prelude::*;
use board::hal::stm32;
use board::gpio;
use board::gpio::gpiog::{PG13, PG14};


//use hal::digital::OutputPin;

use cortex_m::peripheral::Peripherals;

struct Leds {
   green:  PG13<gpio::Output<gpio::PushPull>>,
   orange: PG14<gpio::Output<gpio::PushPull>>,
}

#[entry]
fn main() -> ! {
    if let (Some(p), Some(cp)) = (stm32::Peripherals::take(), Peripherals::take()) {
        let gpiog = p.GPIOG.split();
        
        // Configure LED outputs  
        let mut leds = Leds {
          green: gpiog.pg13.into_push_pull_output(),
          orange: gpiog.pg14.into_push_pull_output(),
        };
    
        // Constrain clock registers
        let rcc = p.RCC.constrain();

        // Configure clock to 168 MHz (i.e. the maximum) and freeze it
        let clocks = rcc.cfgr.freeze();
        //let clocks = rcc.cfgr.sysclk(1.mhz()).freeze();

        // Get delay provider
        //let mut delay = Delay::new(cp.SYST, clocks);
        let mut delay = Delay::new(cp.SYST, clocks);

        loop {
            // Turn LED on
            leds.green.set_high();
            leds.orange.set_high();
            // Delay twice for half a second due to limited timer resolution
            delay.delay_ms(500_u16);
            delay.delay_ms(500_u16);
         
            // Turn LED off
            leds.orange.set_low();
            leds.green.set_low();
    
            // Delay twice for half a second due to limited timer resolution
            delay.delay_ms(500_u16);
            delay.delay_ms(500_u16);
        }
    }

    loop {}
}
