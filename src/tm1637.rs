use embedded_hal::{timer::CountDown, digital::{OutputPin, InputPin}};
use nb::block;

pub struct TM1637<'chip, DIO, CLK, TIMER> {
    dio: DIO,
    clk: CLK,
    timer: &'chip mut TIMER,
}

impl<'chip, DIO, CLK, TIMER> TM1637<'chip, DIO, CLK, TIMER> 
where 
    DIO: OutputPin + InputPin,
    CLK: OutputPin,
    TIMER: CountDown,
{
    pub fn new(dio: DIO, clk: CLK, timer: &'chip mut TIMER) -> Self {
        Self { dio, clk, timer }
    }

    pub fn free(self) -> (DIO, CLK) {
        (self.dio, self.clk)
    }
    
    pub fn chip_start(&mut self) {
        self.clk.set_high();
        self.dio.set_high();
        block!(self.timer.wait()).unwrap();
        self.dio.set_low();
        block!(self.timer.wait()).unwrap();
        self.clk.set_low();
        block!(self.timer.wait()).unwrap();
    }

    pub fn chip_stop(&mut self) {
        self.clk.set_low();
        block!(self.timer.wait()).unwrap();
        self.dio.set_low();
        block!(self.timer.wait()).unwrap();
        self.clk.set_high();
        block!(self.timer.wait()).unwrap();
        self.dio.set_high();
        block!(self.timer.wait()).unwrap();
    }

    fn write_bit(&mut self, bit: bool) {
        self.clk.set_low();
        block!(self.timer.wait()).unwrap();
        if bit {
            self.dio.set_high();
        } else {
            self.dio.set_low();
        }
        block!(self.timer.wait()).unwrap();
        self.clk.set_high();
        block!(self.timer.wait()).unwrap();
    }

    fn write_byte(&mut self, byte: u8) {
        for i in 0..8 {
            self.write_bit((byte >> i) & 0x01 != 0);
        }
        self.clk.set_low();
        block!(self.timer.wait()).unwrap();
        self.dio.set_high();
        block!(self.timer.wait()).unwrap();
        self.clk.set_high();
        block!(self.timer.wait()).unwrap();
        while self.dio.is_high() {}
    }

    pub fn write_cmd(&mut self, cmd: u8) {
        self.chip_start();
        self.write_byte(cmd);
        self.chip_stop();
    }

    pub fn write_data(&mut self, addr: u8, data: u8) {
        self.chip_start();
        self.write_byte(addr);
        self.write_byte(data);
        self.chip_stop();
    }
}
