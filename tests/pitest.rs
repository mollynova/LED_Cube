use std::f64;
extern crate rppal;
use hound;
use std::thread;
use std::time::Duration;

//use rppal::gpio::{Gpio, Mode, Level};
use rppal::gpio::Gpio;
use std::error::Error;

// a test to make sure rppal is working properly
fn main() -> Result <(), Box<dyn Error>> {
  let gpio: u8 = 4;
  let mut test_gpio = Gpio::new()?.get(gpio)?.into_output();

  println!("Testing... blue light should come on for 1/4 second");
  test_gpio.set_high();
  thread::sleep(Duration::from_millis(250));
  test_gpio.set_low();

  println!("Testing... blue light should come on for 2 seconds");
  test_gpio.set_high();
  thread::sleep(Duration::from_millis(2000));
  test_gpio.set_low();

  Ok(())
}
