#![no_std]
#![no_main]

pub mod tm1637;

extern crate panic_semihosting;

use stm32f1xx_hal as hal;
use cortex_m_rt::entry;
use hal::prelude::*;
use hal::timer::Timer;
use tm1637::TM1637;

static DIGIT: [u8; 10] = [0x3f,0x06,0x5b,0x4f,0x66,0x6d,0x7d,0x07,0x7f,0x6f];

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = hal::pac::Peripherals::take().unwrap();
    
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);
    let dio = gpiob.pb9.into_open_drain_output(&mut gpiob.crh);
    let clk = gpiob.pb8.into_open_drain_output(&mut gpiob.crh);
    
    let mut tim = Timer::tim1(dp.TIM1, 1.mhz(), clocks, &mut rcc.apb2);
    let mut tm1637 = TM1637::new(dio, clk, &mut tim);

    let mut syst = Timer::syst(cp.SYST, 2.hz(), clocks);
    let mut a = [1, 2, 3, 4];
    loop {
        tm1637.write_cmd(0x44);
        tm1637.write_data(0xc0, DIGIT[a[0]]);
        tm1637.write_data(0xc1, DIGIT[a[1]]);
        tm1637.write_data(0xc2, DIGIT[a[2]]);
        tm1637.write_data(0xc3, DIGIT[a[3]]);
        tm1637.write_cmd(0x8a);
        for i in 0..4 {
            a[i] = if a[i] >= 9 { 0 } else { a[i] + 1 };
        }
        nb::block!(syst.wait()).unwrap();
    }
}
