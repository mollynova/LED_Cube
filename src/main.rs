use std::f64;
extern crate num;
extern crate rustfft;
extern crate rppal;
use hound;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use std::error::Error;
use num::complex::Complex;
use rustfft::FFT;
use self::rustfft::FFTplanner;
use self::rustfft::num_traits::Zero;

// a test to make sure rppal is working properly
fn test_rppal() -> Result <(), Box<dyn Error>> {
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

fn main() {
  // initialize hardware
  init_hardware();

  // use 'hound' crate to read in a given .wav file
  let mut reader = hound::WavReader::open("wav_files/ImIntoYou.wav").unwrap();

  // build complex vector of samples in preparation for FFT
  let samples = reader.samples::<i16>()
       .map(|x| Complex::new(x.unwrap() as f32, 0f32))
       .collect::<Vec<_>>();

  // compute the number of samples that there are per second of music for the given wav file
  let num_samples = samples.len();
  let seconds            = 270; // say each song is about 4 minutes
  let samples_per_second = (num_samples / seconds) as usize;

  let chunked_signals: Vec<_> = samples.chunks_exact(samples_per_second).collect();

  for chunk in chunked_signals {
    let temp_vec = chunk.to_vec();
    if let Some(peak) = find_peak(temp_vec) {
      // calculate the range of the peak
      pick_color(peak);
    }
  }
}

fn init_hardware() -> Result <(), Box<dyn Error>> {
  // set GPIO pins values for each color as global constants
  // note that "gpio" from rppal uses BCM pin numbering

  let r1_GPIO: u8 = 2;
  let r2_GPIO: u8 = 3;
  let r3_GPIO: u8 = 4;
  let r4_GPIO: u8 = 17;

  let y1_GPIO: u8 = 27;
  let y2_GPIO: u8 = 22;
  let y3_GPIO: u8 = 10;
  let y4_GPIO: u8 = 9;

  let g1_GPIO: u8 = 5;
  let g2_GPIO: u8 = 6;
  let g3_GPIO: u8 = 13;
  let g4_GPIO: u8 = 19;

  let b1_GPIO: u8 = 12;
  let b2_GPIO: u8 = 16;
  let b3_GPIO: u8 = 20;
  let b4_GPIO: u8 = 21;

  // declare the 16 GPIO pins and set their modes to "output"
  let mut r1_gpio = Gpio::new()?.get(r1_GPIO)?.into_output();
  let mut r2_gpio = Gpio::new()?.get(r2_GPIO)?.into_output();
  let mut r3_gpio = Gpio::new()?.get(r3_GPIO)?.into_output();
  let mut r4_gpio = Gpio::new()?.get(r4_GPIO)?.into_output();

  let mut y1_gpio = Gpio::new()?.get(y1_GPIO)?.into_output();
  let mut y2_gpio = Gpio::new()?.get(y2_GPIO)?.into_output();
  let mut y3_gpio = Gpio::new()?.get(y3_GPIO)?.into_output();
  let mut y4_gpio = Gpio::new()?.get(y4_GPIO)?.into_output();

  let mut g1_gpio = Gpio::new()?.get(g1_GPIO)?.into_output();
  let mut g2_gpio = Gpio::new()?.get(g2_GPIO)?.into_output();
  let mut g3_gpio = Gpio::new()?.get(g3_GPIO)?.into_output();
  let mut g4_gpio = Gpio::new()?.get(g4_GPIO)?.into_output();

  let mut b1_gpio = Gpio::new()?.get(b1_GPIO)?.into_output();
  let mut b2_gpio = Gpio::new()?.get(b2_GPIO)?.into_output();
  let mut b3_gpio = Gpio::new()?.get(b3_GPIO)?.into_output();
  let mut b4_gpio = Gpio::new()?.get(b4_GPIO)?.into_output();

  Ok(())
}

fn pick_color(frequency: f32) -> () {
  /*
    Note: 30 = target red freq, 108.3 = target y/r freq, 186.6 = target yellow freq, 265.0 = target yg freq
          343.2 = target green freq, 421.5 = target gb freq, 500.0 = target blue freq
  */

  let mut vec = Vec::new();
  vec.push(30.0);
  vec.push(108.3);
  vec.push(186.6);
  vec.push(265.0);
  vec.push(343.2);
  vec.push(421.5);
  vec.push(500.0);

  let mut closeness: f32 = 600.0;
  let mut count: i32 = 0;
  for x in &vec {
    if (x - frequency).abs() < closeness {
      closeness = (x - frequency).abs();
      count = count + 1;
    }
  }
  count = count - 1;
  select_light(count);
}

fn select_light(val: i32) -> () {
  match val {
    0 => light_red(),
    1 => light_ry(),
    2 => light_yellow(),
    3 => light_yg(),
    4 => light_green(),
    5 => light_gb(),
    6 => light_blue(),
    _ => light_blue(),
  }
}

fn light_red() {
  r1_gpio.set_high();
  thread::sleep(Duration::from_millis(250));
  r1_gpio.set_low();
  r2_gpio.set_high();
  thread::sleep(Duration::from_millis(250));
  r2_gpio.set_low();
  r3_gpio.set_high();
  thread::sleep(Duration::from_millis(250));
  r3_gpio.set_low();
  r4_gpio.set_high();
  thread::sleep(Duration::from_millis(250));
  r4_gpio.set_low();
}

fn light_ry() {
  r1_gpio.set_high();
  y4_gpio.set_high();
  thread::sleep(Duration::from_millis(250));
  r1_gpio.set_low();
  y4_gpio.set_low();

  r2_gpio.set_high();
  y3_gpio.set_high();
  thread::sleep(Duration::from_millis(250));
  r2_gpio.set_low();
  y3_gpio.set_low();

  r3_gpio.set_high();
  y2_gpio.set_high();
  thread::sleep(Duration::from_millis(250));
  r3_gpio.set_low();
  y2_gpio.set_low();

  r4_gpio.set_high();
  y1_gpio.set_high();
  thread::sleep(Duration::from_millis(250));
  r4_gpio.set_low();
  r4_gpio.set_low();

}

fn light_yellow() {
  y1_gpio.set_high();
  thread::sleep(Duration::from_millis(250));
  y1_gpio.set_low();
  y2_gpio.set_high();
  thread::sleep(Duration::from_millis(250));
  y2_gpio.set_low();
  y3_gpio.set_high();
  thread::sleep(Duration::from_millis(250));
  y3_gpio.set_low();
  y4_gpio.set_high();
  thread::sleep(Duration::from_millis(250));
  y4_gpio.set_low();

}

fn light_yg() {
  y1_gpio.set_high();
  y4_gpio.set_high();
  thread::sleep(Duration::from_millis(250));
  y1_gpio.set_low();
  g4_gpio.set_low();

  y2_gpio.set_high();
  g3_gpio.set_high();
  thread::sleep(Duration::from_millis(250));
  y2_gpio.set_low();
  g3_gpio.set_low();

  y3_gpio.set_high();
  g2_gpio.set_high();
  thread::sleep(Duration::from_millis(250));
  y3_gpio.set_low();
  g2_gpio.set_low();

  y4_gpio.set_high();
  g1_gpio.set_high();
  thread::sleep(Duration::from_millis(250));
  y4_gpio.set_low();
  g1_gpio.set_low();
}

fn light_green() {
  g1_gpio.set_high();
  thread::sleep(Duration::from_millis(250));
  g1_gpio.set_low();
  g2_gpio.set_high();
  thread::sleep(Duration::from_millis(250));
  g2_gpio.set_low();
  g3_gpio.set_high();
  thread::sleep(Duration::from_millis(250));
  g3_gpio.set_low();
  g4_gpio.set_high();
  thread::sleep(Duration::from_millis(250));
  g4_gpio.set_low();

}

fn light_gb() {
  g1_gpio.set_high();
  b4_gpio.set_high();
  thread::sleep(Duration::from_millis(250));
  g1_gpio.set_low();
  b4_gpio.set_low();

  g2_gpio.set_high();
  b3_gpio.set_high();
  thread::sleep(Duration::from_millis(250));
  g2_gpio.set_low();
  b3_gpio.set_low();

  g3_gpio.set_high();
  b2_gpio.set_high();
  thread::sleep(Duration::from_millis(250));
  g3_gpio.set_low();
  b2_gpio.set_low();

  g4_gpio.set_high();
  b1_gpio.set_high();
  thread::sleep(Duration::from_millis(250));
  g4_gpio.set_low();
  b1_gpio.set_low();
}

fn light_blue() {
  b1_gpio.set_high();
  thread::sleep(Duration::from_millis(250));
  b1_gpio.set_low();
  b2_gpio.set_high();
  thread::sleep(Duration::from_millis(250));
  b2_gpio.set_low();
  b3_gpio.set_high();
  thread::sleep(Duration::from_millis(250));
  b3_gpio.set_low();
  b4_gpio.set_high();
  thread::sleep(Duration::from_millis(250));
  b4_gpio.set_low();
}

fn find_peak(chunk: Vec<num::Complex<f32>>) -> Option<f32> {
    let mut sim = chunk.clone();
    let mut output: Vec<Complex<f32>> = vec![Complex::zero(); chunk.len()];
    let mut planner = FFTplanner::new(false);
    let fft = planner.plan_fft(chunk.len());

    fft.process(&mut sim[..], &mut output[..]);
    let max_peak = output.iter()
         .take(chunk.len() / 2)
         .enumerate()
         .max_by_key(|&(_, freq) | freq.norm() as u32);
    if let Some((i, _)) = max_peak {
       let bin = 44100f32 / chunk.len() as f32;
       Some(i as f32 * bin)
    } else {
       None
    }
}








