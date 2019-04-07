#![no_std]
#![no_main]

pub mod tm1637;

extern crate panic_semihosting;

use cortex_m_rt::entry;
use stm32f1xx_hal as hal;
use hal::prelude::*;
use hal::timer::Timer;
use nb::block;
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
    let dio1 = gpiob.pb9.into_open_drain_output(&mut gpiob.crh);
    let clk1 = gpiob.pb8.into_open_drain_output(&mut gpiob.crh);
    let dio2 = gpiob.pb1.into_open_drain_output(&mut gpiob.crl);
    let clk2 = gpiob.pb0.into_open_drain_output(&mut gpiob.crl);
    let dio3 = gpiob.pb7.into_open_drain_output(&mut gpiob.crl);
    let clk3 = gpiob.pb6.into_open_drain_output(&mut gpiob.crl);
    
    let mut tim1 = Timer::tim1(dp.TIM1, 1.mhz(), clocks, &mut rcc.apb2);
    let mut tim2 = Timer::tim2(dp.TIM2, 1.mhz(), clocks, &mut rcc.apb1);
    let mut tim3 = Timer::tim3(dp.TIM3, 1.mhz(), clocks, &mut rcc.apb1);
    let mut tm1637_1 = TM1637::new(dio1, clk1, &mut tim1);
    let mut tm1637_2 = TM1637::new(dio2, clk2, &mut tim2);
    let mut tm1637_3 = TM1637::new(dio3, clk3, &mut tim3);

    let mut syst = Timer::syst(cp.SYST, 2.hz(), clocks);
    let mut a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2];
    loop {
        tm1637_1.write_cmd(0x44);
        tm1637_1.write_data(0xc0, DIGIT[a[0]]);
        tm1637_1.write_data(0xc1, DIGIT[a[1]]);
        tm1637_1.write_data(0xc2, DIGIT[a[2]]);
        tm1637_1.write_data(0xc3, DIGIT[a[3]]);
        tm1637_1.write_cmd(0x8a);
        tm1637_2.write_cmd(0x44);
        tm1637_2.write_data(0xc0, DIGIT[a[4]]);
        tm1637_2.write_data(0xc1, DIGIT[a[5]]);
        tm1637_2.write_data(0xc2, DIGIT[a[6]]);
        tm1637_2.write_data(0xc3, DIGIT[a[7]]);
        tm1637_2.write_cmd(0x8a);
        tm1637_3.write_cmd(0x44);
        tm1637_3.write_data(0xc0, DIGIT[a[8]]);
        tm1637_3.write_data(0xc1, DIGIT[a[9]]);
        tm1637_3.write_data(0xc2, DIGIT[a[10]]);
        tm1637_3.write_data(0xc3, DIGIT[a[11]]);
        tm1637_3.write_cmd(0x8a);
        for i in 0..12 {
            a[i] = if a[i] >= 9 { 0 } else { a[i] + 1 };
        }
        block!(syst.wait()).unwrap();
    }
}
