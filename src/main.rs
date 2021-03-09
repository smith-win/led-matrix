#[macro_use]
extern crate log;
extern crate wiringpi;

use wiringpi::{WiringPi, pin::{OutputPin, Value}};

/// DS Pin of 74HC595(Pin14)
const PIN_DATA: u16 = 0;
//ST_CP Pin of 74HC595(Pin12)
const PIN_LATCH: u16 = 2;
//SH_CP Pin of 74HC595(Pin11)
const PIN_CLOCK: u16 = 3;



/// writes one line of the image
//fn write_line(data_pin: &OutputPin<WiringPi<wiringpi::pin::WiringPi>>, clock_pin: &OutputPin<WiringPi<wiringpi::pin::WiringPi>>, val: u8) {
fn write_line<T>(data_pin: &OutputPin<T>, clock_pin: &OutputPin<T>, val: u8) 
    where T: wiringpi::pin::Pin{    
        
    const DELAY : std::time::Duration = std::time::Duration::from_micros(10);
    (0u8..8u8).into_iter()
        .for_each(
            |i| {
                clock_pin.digital_write(Value::Low);
                let pinval = match 0x80u8 & (val << i) {
                    0x80 => Value::High,
                    _ => Value::Low,
                };
                data_pin.digital_write(pinval);
                clock_pin.digital_write(Value::High);
                std::thread::sleep(DELAY);
            }
        )
}

/// Writes image to the Matrix
fn write_image(img: &[u8; 8], wp: &WiringPi<wiringpi::pin::WiringPi>) {

    const DELAY : std::time::Duration = std::time::Duration::from_millis(1);
    let mut x = 0x80u8;

    let data_pin = wp.output_pin(PIN_DATA);
    let latch_pin = wp.output_pin(PIN_LATCH);
    let clock_pin = wp.output_pin(PIN_CLOCK);

    for b in img {
        latch_pin.digital_write(Value::Low);
        write_line(&data_pin, &clock_pin, *b);
        write_line(&data_pin, &clock_pin, !x);

        latch_pin.digital_write(Value::High);

        x >>= 1;
        std::thread::sleep(DELAY);
    }

}


fn main() {
    env_logger::init();
    
    info!("Starting");

    //Setup WiringPi with its own pin numbering order
    let pi = wiringpi::setup();


    // smily face
    let face: [u8; 8] = [0x1c,0x22,0x51,0x45,0x45,0x51,0x22,0x1c];
    for b in &face {
        println!("{:08b}", b);
    }

    loop {
        write_image(&face, &pi);
    }
       

}
