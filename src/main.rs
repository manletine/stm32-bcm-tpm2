#![no_std]
#![no_main]

extern crate panic_semihosting;

use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;
use stm32f1xx_hal::pac::{CorePeripherals, Peripherals};
use stm32f1xx_hal::prelude::*;
use stm32f1xx_hal::delay::Delay;

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let cp = CorePeripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);

    let clocks = rcc
        .cfgr
        .use_hse(8.mhz())
        .sysclk(72.mhz())
        .hclk(72.mhz())
        .pclk1(36.mhz())
        .pclk2(72.mhz())
        .freeze(&mut flash.acr);
    
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    let mut delay = Delay::new(cp.SYST, clocks);
    
    loop {
       led.set_low().unwrap();
       delay.delay_ms(1000u32);

       led.set_high().unwrap();
       delay.delay_ms(1000u32);
    }
}
