#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m_rt::entry;
use panic_semihosting as _;

use nrf52840_pac as device;

mod pic;

//

const CLOCK: u32 = 64_000_000;
const D10MS: u32 = CLOCK / 1000 * 10;
const DELTA: u32 = 1;

#[entry]
fn main() -> !
{
    let peri = nrf52840_pac::Peripherals::take().unwrap();

    // ----- ----- ----- ----- -----

    let p0 = peri.P0;

    p0.outset.write(|w| w
        .pin30().set_bit() // CS
        .pin2().set_bit() // RES
    );

    p0.outclr.write(|w| w
        .pin28().set_bit() // D/C
        .pin11().set_bit() // EN
    );

    // CS
    p0.pin_cnf[30].write(|w| w
        .dir().output()
        .input().disconnect()
        .pull().disabled()
        .drive().s0s1()
        .sense().disabled()
    );
    
    // D/C
    p0.pin_cnf[28].write(|w| w
        .dir().output()
        .input().disconnect()
        .pull().disabled()
        .drive().s0s1()
        .sense().disabled()
    );
    
    // EN
    p0.pin_cnf[11].write(|w| w
        .dir().output()
        .input().disconnect()
        .pull().disabled()
        .drive().s0s1()
        .sense().disabled()
    );
    
    // RES
    p0.pin_cnf[2].write(|w| w
        .dir().output()
        .input().disconnect()
        .pull().disabled()
        .drive().s0s1()
        .sense().disabled()
    );
    
    // BUSY
    p0.pin_cnf[3].write(|w| w
        .dir().input()
        .input().connect()
        .pull().disabled()
        .sense().disabled()
    );

    // ----- ----- ----- ----- -----

    let spi0 = peri.SPI0;

    // SCLK
    spi0.psel.sck.write(|w| unsafe { w
        .pin().bits(31)
        .port().clear_bit()
        .connect().connected()
    } );

    // SDI
    spi0.psel.mosi.write(|w| unsafe { w
        .pin().bits(29)
        .port().clear_bit()
        .connect().connected()
    } );

    spi0.frequency.write(|w| w.frequency().m4());

    spi0.enable.write(|w| w.enable().enabled());

    // ----- ----- ----- ----- -----
    
    let epd = Epd::new(p0, spi0);

    epd.init();

    loop {
        //epd.pic_display();
        epd.pic_display_ferris();
        epd.refresh();
        asm::delay(CLOCK);
        epd.pic_display_clear();
        epd.refresh();
    }
}

const LUT_VCOM0: [u8; 15] = [
    0x0E, 0x14, 0x01, 0x0A, 0x06, 0x04, 0x0A, 0x0A,
    0x0F, 0x03, 0x03, 0x0C, 0x06, 0x0A, 0x00
];

const LUT_W: [u8; 15] = [
    0x0E, 0x14, 0x01, 0x0A, 0x46, 0x04, 0x8A, 0x4A,
    0x0F, 0x83, 0x43, 0x0C, 0x86, 0x0A, 0x04
];

const LUT_B: [u8; 15] = [
    0x0E, 0x14, 0x01, 0x8A, 0x06, 0x04, 0x8A, 0x4A,
    0x0F, 0x83, 0x43, 0x0C, 0x06, 0x4A, 0x04
];

const LUT_G1: [u8; 15] = [
    0x8E, 0x94, 0x01, 0x8A, 0x06, 0x04, 0x8A, 0x4A,
    0x0F, 0x83, 0x43, 0x0C, 0x06, 0x0A, 0x04
  ];

const LUT_G2: [u8; 15] = [
    0x8E, 0x94, 0x01, 0x8A, 0x06, 0x04, 0x8A, 0x4A,
    0x0F, 0x83, 0x43, 0x0C, 0x06, 0x0A, 0x04
];

const LUT_VCOM1: [u8; 15] = [
    0x03, 0x1D, 0x01, 0x01, 0x08, 0x23, 0x37, 0x37,
    0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
];

const LUT_RED0: [u8; 15] = [
    0x83, 0x5D, 0x01, 0x81, 0x48, 0x23, 0x77, 0x77,
    0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
];

const LUT_RED1: [u8; 15] = [
    0x03, 0x1D, 0x01, 0x01, 0x08, 0x23, 0x37, 0x37,
    0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
];

enum EpdPacket
{
    Command(u8),
    Data(u8)
}

struct Epd
{
    p0: device::P0,
    spi0: device::SPI0
}

impl Epd
{
    fn new(p0: device::P0, spi0: device::SPI0) -> Epd
    {
        Epd {
            p0,
            spi0
        }
    }

    fn init(&self)
    {
        self.w21_init();

        self.w21_write(EpdPacket::Command(0x01));
        self.w21_write(EpdPacket::Data(0x07));
        self.w21_write(EpdPacket::Data(0x00));
        self.w21_write(EpdPacket::Data(0x08));
        self.w21_write(EpdPacket::Data(0x00));
        
        self.w21_write(EpdPacket::Command(0x06));
        self.w21_write(EpdPacket::Data(0x07));
        self.w21_write(EpdPacket::Data(0x07));
        self.w21_write(EpdPacket::Data(0x07));
        
        self.w21_write(EpdPacket::Command(0x04));
        self.chkstatus();
    
        self.w21_write(EpdPacket::Command(0x00));
        self.w21_write(EpdPacket::Data(0xcf));
    
        self.w21_write(EpdPacket::Command(0x50));
        self.w21_write(EpdPacket::Data(0x17));
        
        self.w21_write(EpdPacket::Command(0x30));
        self.w21_write(EpdPacket::Data(0x39));
        
        self.w21_write(EpdPacket::Command(0x61));
        self.w21_write(EpdPacket::Data(0xc8));
        self.w21_write(EpdPacket::Data(0x00));
        self.w21_write(EpdPacket::Data(0xc8));
        
        self.w21_write(EpdPacket::Command(0x82));
        self.w21_write(EpdPacket::Data(0x30));
        
        self.lut_bw();
        self.lut_red();
    }

    fn w21_init(&self)
    {
        self.p0.outclr.write(|w| w.pin11().set_bit());

        self.p0.outclr.write(|w| w.pin2().set_bit());
        asm::delay(D10MS);
        self.p0.outset.write(|w| w.pin2().set_bit());
        asm::delay(D10MS);
    }

    fn w21_cs(&self, set: bool)
    {
        if set {
            self.p0.outset.write(|w| w.pin30().set_bit());
        }
        else {
            self.p0.outclr.write(|w| w.pin30().set_bit());
        }
    }

    fn w21_dc(&self, set: bool)
    {
        if set {
            self.p0.outset.write(|w| w.pin28().set_bit());
        }
        else {
            self.p0.outclr.write(|w| w.pin28().set_bit());
        }
    }

    fn w21_write(&self, pkt: EpdPacket)
    {
        asm::delay(DELTA);

        self.w21_cs(false);

        match pkt {
            EpdPacket::Command(value) => {
                self.w21_dc(false);
                self.spi0.txd.write(|w| unsafe { w.txd().bits(value) } );
            },
            EpdPacket::Data(value) => {
                self.w21_dc(true);
                self.spi0.txd.write(|w| unsafe { w.txd().bits(value) } );
            }
        }

        while self.spi0.events_ready.read().events_ready().bit_is_clear() {
            asm::delay(DELTA);
        }
        self.spi0.rxd.read();
        self.spi0.events_ready.write(|w| w.events_ready().clear_bit());

        self.w21_cs(true);
    }

    fn chkstatus(&self)
    {
        asm::delay(D10MS);

        while self.p0.in_.read().pin3().is_low() {
            asm::delay(DELTA);
        }

        asm::delay(D10MS);
    }

    fn lut_bw(&self)
    {
        self.w21_write(EpdPacket::Command(0x20));
        for v in &LUT_VCOM0 {
            self.w21_write(EpdPacket::Data(*v));
        }

        self.w21_write(EpdPacket::Command(0x21));
        for v in &LUT_W {
            self.w21_write(EpdPacket::Data(*v));
        }

        self.w21_write(EpdPacket::Command(0x22));
        for v in &LUT_B {
            self.w21_write(EpdPacket::Data(*v));
        }

        self.w21_write(EpdPacket::Command(0x23));
        for v in &LUT_G1 {
            self.w21_write(EpdPacket::Data(*v));
        }

        self.w21_write(EpdPacket::Command(0x24));
        for v in &LUT_G2 {
            self.w21_write(EpdPacket::Data(*v));
        }
    }

    fn lut_red(&self)
    {
        self.w21_write(EpdPacket::Command(0x25));
        for v in &LUT_VCOM1 {
            self.w21_write(EpdPacket::Data(*v));
        }

        self.w21_write(EpdPacket::Command(0x26));
        for v in &LUT_RED0 {
            self.w21_write(EpdPacket::Data(*v));
        }

        self.w21_write(EpdPacket::Command(0x27));
        for v in &LUT_RED1 {
            self.w21_write(EpdPacket::Data(*v));
        }
    }

    fn _pic_display(&self)
    {
        self.w21_write(EpdPacket::Command(0x10));
        for i in 0..(200*200/4) {
            let y = (i * 4) / 200;
            let p = if y < 100 {0x00} else {0xff};
            self.w21_write(EpdPacket::Data(p as u8));
        }

        self.w21_write(EpdPacket::Command(0x13));
        for i in 0..(200*200/8) {
            let x = (i * 8) % 200;
            let p = if x < 100 {0x00} else {0xff};
            self.w21_write(EpdPacket::Data(p as u8));
        }
    }

    fn pic_display_ferris(&self)
    {
        self.w21_write(EpdPacket::Command(0x10));
        for p in pic::PIC_BLACK.iter() {
            self.w21_write(EpdPacket::Data(*p));
        }

        self.w21_write(EpdPacket::Command(0x13));
        for p in pic::PIC_RED.iter() {
            self.w21_write(EpdPacket::Data(*p));
        }
    }

    fn pic_display_clear(&self)
    {
        self.w21_write(EpdPacket::Command(0x10));
        for _ in 0..(200*200/4) {
            self.w21_write(EpdPacket::Data(0xff));
        }

        self.w21_write(EpdPacket::Command(0x13));
        for _ in 0..(200*200/8) {
            self.w21_write(EpdPacket::Data(0xff));
        }
    }

    fn refresh(&self)
    {
        self.w21_write(EpdPacket::Command(0x12));
        self.chkstatus();
    }
}
