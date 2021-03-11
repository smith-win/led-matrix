#[macro_use]
extern crate log;
extern crate wiringpi;
extern crate font8x8;

mod display;


use wiringpi::{WiringPi, pin::{OutputPin, Value}};
use font8x8::UnicodeFonts;
use display::Display;

/// DS Pin of 74HC595(Pin14)
const PIN_DATA: u16 = 0;
//ST_CP Pin of 74HC595(Pin12)
const PIN_LATCH: u16 = 2;
//SH_CP Pin of 74HC595(Pin11)
const PIN_CLOCK: u16 = 3;


/// writes one line of the image
fn write_line<T>(data_pin: &OutputPin<T>, clock_pin: &OutputPin<T>, val: u8) 
    where T: wiringpi::pin::Pin {
        
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
fn write_image(img: &[u8], wp: &WiringPi<wiringpi::pin::WiringPi>) {
    assert_eq!(8, img.len());
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

/// Font 8x8 need rotating 90 degrees
fn transpose_glyph(glyph: &[u8;8]) -> [u8;8] {

    let mut result = [0u8; 8];

    let mut y_val = 128;
    for y in 0..8 {
        let b = glyph[y];

        let mut test = 1u8;
        for x in 0..8 {
            // println!("test {} , yval {}, result[x] {}", test, y_val, result[x]);
            if b & test == test {
                result[x] |= y_val;
            }
            test <<= 1;
        }
        y_val >>= 1;
    }

    result

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
    println!();
    let face2 = transpose_glyph(&face);
    for b in &face2 {
        println!("{:08b}", b);
    }

    println!();
    let face2 = transpose_glyph(&font8x8::BASIC_FONTS.get('f').unwrap());
    for b in &face2 {
        println!("{:08b}", b);
    }


    // this is the message
    let message = "ffff-fff Hello Everybody";

    // convert it into a &[u8; 8]
    let mut vec:Vec<u8> = Vec::new();
    for c in message.chars() {
        if let Some(glyph) = font8x8::BASIC_FONTS.get(c) {
            vec.extend_from_slice( &transpose_glyph(&glyph));
        }
    }
    // append a space at the end
    if let Some(glyph) = font8x8::BASIC_FONTS.get(' ') {
        vec.extend_from_slice( &transpose_glyph(&glyph));
    }

    loop {
        let mut display = Display::new();

        for y in 0..8 {
            display.clear();
            for x in 0..8 {
                display.set(x, y, true);
                (0..8).into_iter()
                    .for_each( |_x| write_image( display.borrow_bytes(), &pi));
            }
        }
    }

    
    // std::process::exit(0);

    // loop {

    //     let windows = vec.windows(8);

    //     for w in windows {
    //         let x = &w[0..8];
    //         (0..8).into_iter()
    //             .for_each( |_x| write_image(x, &pi));
    //     }
    // }
      

}
