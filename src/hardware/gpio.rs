extern crate rppal;
use rppal::gpio::Gpio;
use std::thread;
use std::time::Duration;
#[derive(Debug)]

// struct with 16 OutputPins. this is a type from the rppal crate that
// allows you to specify whether the GPIO is intended for input or output,
// and allows you to toggle the lights
#[allow(non_camel_case_types)]
pub struct GPIO {
  pub r1_gpio: rppal::gpio::OutputPin,
  pub r2_gpio: rppal::gpio::OutputPin,
  pub r3_gpio: rppal::gpio::OutputPin,
  pub r4_gpio: rppal::gpio::OutputPin,
  pub y1_gpio: rppal::gpio::OutputPin,
  pub y2_gpio: rppal::gpio::OutputPin,
  pub y3_gpio: rppal::gpio::OutputPin,
  pub y4_gpio: rppal::gpio::OutputPin,
  pub g1_gpio: rppal::gpio::OutputPin,
  pub g2_gpio: rppal::gpio::OutputPin,
  pub g3_gpio: rppal::gpio::OutputPin,
  pub g4_gpio: rppal::gpio::OutputPin,
  pub b1_gpio: rppal::gpio::OutputPin,
  pub b2_gpio: rppal::gpio::OutputPin,
  pub b3_gpio: rppal::gpio::OutputPin,
  pub b4_gpio: rppal::gpio::OutputPin,
}

impl GPIO {
  pub fn red_on(&mut self) {
    self.r1_gpio.set_high();
    thread::sleep(Duration::from_millis(250));
    self.r1_gpio.set_low();
    self.r2_gpio.set_high();
    thread::sleep(Duration::from_millis(250));
    self.r2_gpio.set_low();
    self.r3_gpio.set_high();
    thread::sleep(Duration::from_millis(250));
    self.r3_gpio.set_low();
    self.r4_gpio.set_high();
    thread::sleep(Duration::from_millis(250));
    self.r4_gpio.set_low();
  }

  pub fn yellow_on(&mut self) {
    self.y1_gpio.set_high();
    thread::sleep(Duration::from_millis(250));
    self.y1_gpio.set_low();
    self.y2_gpio.set_high();
    thread::sleep(Duration::from_millis(250));
    self.y2_gpio.set_low();
    self.y3_gpio.set_high();
    thread::sleep(Duration::from_millis(250));
    self.y3_gpio.set_low();
    self.y4_gpio.set_high();
    thread::sleep(Duration::from_millis(250));
    self.y4_gpio.set_low();
  }

  pub fn green_on(&mut self) {
    self.g1_gpio.set_high();
    thread::sleep(Duration::from_millis(250));
    self.g1_gpio.set_low();
    self.g2_gpio.set_high();
    thread::sleep(Duration::from_millis(250));
    self.g2_gpio.set_low();
    self.g3_gpio.set_high();
    thread::sleep(Duration::from_millis(250));
    self.g3_gpio.set_low();
    self.g4_gpio.set_high();
    thread::sleep(Duration::from_millis(250));
    self.g4_gpio.set_low();
  }

  pub fn blue_on(&mut self) {
    self.b1_gpio.set_high();
    thread::sleep(Duration::from_millis(250));
    self.b1_gpio.set_low();
    self.b2_gpio.set_high();
    thread::sleep(Duration::from_millis(250));
    self.b2_gpio.set_low();
    self.b3_gpio.set_high();
    thread::sleep(Duration::from_millis(250));
    self.b3_gpio.set_low();
    self.b4_gpio.set_high();
    thread::sleep(Duration::from_millis(250));
    self.b4_gpio.set_low();
  }

  pub fn ry_on(&mut self) {
    self.r1_gpio.set_high();
    self.y4_gpio.set_high();
    thread::sleep(Duration::from_millis(250));
    self.r1_gpio.set_low();
    self.y4_gpio.set_low();

    self.r2_gpio.set_high();
    self.y3_gpio.set_high();
    thread::sleep(Duration::from_millis(250));
    self.r2_gpio.set_low();
    self.y3_gpio.set_low();

    self.r3_gpio.set_high();
    self.y2_gpio.set_high();
    thread::sleep(Duration::from_millis(250));
    self.r3_gpio.set_low();
    self.y2_gpio.set_low();

    self.r4_gpio.set_high();
    self.y1_gpio.set_high();
    thread::sleep(Duration::from_millis(250));
    self.r4_gpio.set_low();
    self.y1_gpio.set_low();
  }

  pub fn yg_on(&mut self) {
    self.y1_gpio.set_high();
    self.g4_gpio.set_high();
    thread::sleep(Duration::from_millis(250));
    self.y1_gpio.set_low();
    self.g4_gpio.set_low();

    self.y2_gpio.set_high();
    self.g3_gpio.set_high();
    thread::sleep(Duration::from_millis(250));
    self.y2_gpio.set_low();
    self.g3_gpio.set_low();

    self.y3_gpio.set_high();
    self.g2_gpio.set_high();
    thread::sleep(Duration::from_millis(250));
    self.y3_gpio.set_low();
    self.g2_gpio.set_low();

    self.y4_gpio.set_high();
    self.g1_gpio.set_high();
    thread::sleep(Duration::from_millis(250));
    self.y4_gpio.set_low();
    self.g1_gpio.set_low();
  }

  pub fn gb_on(&mut self) {
    self.g1_gpio.set_high();
    self.b4_gpio.set_high();
    thread::sleep(Duration::from_millis(250));
    self.g1_gpio.set_low();
    self.b4_gpio.set_low();

    self.g2_gpio.set_high();
    self.b3_gpio.set_high();
    thread::sleep(Duration::from_millis(250));
    self.g2_gpio.set_low();
    self.b3_gpio.set_low();

    self.g3_gpio.set_high();
    self.b2_gpio.set_high();
    thread::sleep(Duration::from_millis(250));
    self.g3_gpio.set_low();
    self.b2_gpio.set_low();

    self.g4_gpio.set_high();
    self.b1_gpio.set_high();
    thread::sleep(Duration::from_millis(250));
    self.g4_gpio.set_low();
    self.b1_gpio.set_low();
  }
}

