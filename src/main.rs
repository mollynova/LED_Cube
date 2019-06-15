extern crate num;
extern crate rustfft;
extern crate rppal;
use hound;
//use std::time::Duration;
mod hardware;

use rppal::gpio::Gpio;
use num::complex::Complex;
use rustfft::FFT;
use self::rustfft::FFTplanner;
use self::rustfft::num_traits::Zero;
use std::error::Error;
use std::thread;
use std::fs::File;
use std::io::BufReader;
use rodio::source;

// a test to make sure rppal is working properly
#[test]
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

fn main()  {
  // make vec to hold ordering of lights -- the raspi doesn't process things too quick, so it would make the lights laggy.
  // I'm handling that by having it do all of the calculations (FFTs) first, push the results to a vec, and then run the lights
  let mut lights = Vec::new();

  // create device to play back song while lights are going
  let device = rodio::default_output_device().unwrap();
  let file = File::open("wav_files/ImIntoYou.wav").unwrap();
  let source = rodio::Decoder::new(BufReader::new(file)).unwrap();

  // initialize hardware
  let pinout: hardware::Pinout = init_pinout();
  if let Ok(gpio)  = init_gpio(&pinout) {
  // I created the instance of GPIO in main() because the .get() method of Gpio
  // returns a Result, so if I did this in a separate function, I'd have to return
  // a result and wouldn't be able to return the GPIO itself

  // use 'hound' crate to read in a given .wav file
    let mut reader = hound::WavReader::open("wav_files/ImIntoYou.wav").unwrap();

  // build complex vector of samples in preparation for FFTs
    let samples = reader.samples::<i16>()
         .map(|x| Complex::new(x.unwrap() as f32, 0f32))
         .collect::<Vec<_>>();

  // compute the number of samples that there are per second of music for the given wav file
    let num_samples = samples.len();
    let seconds            = 270; // say each song is about 4 minutes
    let samples_per_second = (num_samples / seconds) as usize;

  // assign each color schema a numeric value to 'match' on
  // red = 0; ry = 1; y = 2; yg = 3; g = 4; gb = 5; b = 6
    let mut color = 0;

  // break vector of samples into vector of vectors of samples where each subvector is
  // approximately one second's worth of samples
    let chunked_signals: Vec<_> = samples.chunks_exact(samples_per_second).collect();

  // for each approximate second's worth of samples
    for chunk in chunked_signals {
      let temp_vec = chunk.to_vec();

    // run find_peak function on that second of samples
    // this function is going to perform an fft on the vector of samples
    // then, it's going to select the highest frequency tone from the resulting list of tones
    // and return its frequency
      if let Some(peak) = find_peak(temp_vec) {
      // run pick_color on the peak frequency returned from find_peak()
      // this will select the appropriate color(s) to light up based on the peak frequency
        color = pick_color(peak);
      // based on the result of pick_color, call the gpio struct's impl function for lighting
      // up the correct color(s)
        match color {
          0 => /*gpio.red_on()*/ lights.push(0),
          1 => /*gpio.ry_on()*/  lights.push(1),
          2 => /*gpio.yellow_on()*/ lights.push(2),
          3 => /*gpio.yg_on()*/ lights.push(3),
          4 => /*gpio.green_on()*/ lights.push(4),
          5 => /*gpio.gb_on()*/ lights.push(5),
          _ => /*gpio.blue_on()*/ lights.push(6),
        }
      }
    }
    // make a new thread to start actually playing the song, because otherwise the entire song is going to
    // play and THEN the lights will start going on. we want them to happen at the same time
    thread::spawn(move || {
      rodio::play_raw(&device, source.convert_samples());
    });

    run_lights(gpio, lights);
  }
  else {
    println!("Error setting BCM pins. Please check the following:");
    println!("  --Pinout is set to Raspi 3 B+ BCM pinout numbering");
    println!("  --LED cube is connected to Raspi");
  }
}

fn run_lights(mut gpio: hardware::GPIO, lights: Vec<u8>) {
  for light in lights {
    match light {
      0 => gpio.red_on(),
      1 => gpio.ry_on(),
      2 => gpio.yellow_on(),
      3 => gpio.yg_on(),
      4 => gpio.green_on(),
      5 => gpio.gb_on(),
      _ => gpio.blue_on(),
    }
  }
}

/* Function to initialize the pinout struct. it holds a u8 value for
   each of the 16 GPIO pins that will be used to control the LED cube on the raspi.
   Note that it needs to take BCM pin numbering to work correctly.

   Naming: r1 signifies the red lights in the lowest layer of the cube. r4 signifies the
   red lights in the top layer of the cube. The same convention is used for the other
   three colors: yellow, green, blue.
*/
fn init_pinout() -> hardware::Pinout {
  let pinout: hardware::Pinout = hardware::Pinout { r1_GPIO :  2, r2_GPIO :  3, r3_GPIO :  4, r4_GPIO : 17,
                                                    y1_GPIO : 27, y2_GPIO : 22, y3_GPIO : 10, y4_GPIO :  9,
                                                    g1_GPIO :  5, g2_GPIO :  6, g3_GPIO : 13, g4_GPIO : 19,
                                                    b1_GPIO : 12, b2_GPIO : 16, b3_GPIO : 20, b4_GPIO : 21 } ;

  pinout
}

// Function to initialize the gpio struct. it creates an instance of OutputPin for each
// of the 16 BCM pins in the "pinout" struct instance that I passed it, and sets
// each of them to output mode.
fn init_gpio(pinout: &hardware::Pinout) -> Result <hardware::GPIO, Box<dyn Error>> {
  let gpio = hardware::GPIO { r1_gpio: Gpio::new()?.get(pinout.r1_GPIO)?.into_output(),
                              r2_gpio: Gpio::new()?.get(pinout.r2_GPIO)?.into_output(),
                              r3_gpio: Gpio::new()?.get(pinout.r3_GPIO)?.into_output(),
                              r4_gpio: Gpio::new()?.get(pinout.r4_GPIO)?.into_output(),
                              y1_gpio: Gpio::new()?.get(pinout.y1_GPIO)?.into_output(),
                              y2_gpio: Gpio::new()?.get(pinout.y2_GPIO)?.into_output(),
                              y3_gpio: Gpio::new()?.get(pinout.y3_GPIO)?.into_output(),
                              y4_gpio: Gpio::new()?.get(pinout.y4_GPIO)?.into_output(),
                              g1_gpio: Gpio::new()?.get(pinout.g1_GPIO)?.into_output(),
                              g2_gpio: Gpio::new()?.get(pinout.g2_GPIO)?.into_output(),
                              g3_gpio: Gpio::new()?.get(pinout.g3_GPIO)?.into_output(),
                              g4_gpio: Gpio::new()?.get(pinout.g4_GPIO)?.into_output(),
                              b1_gpio: Gpio::new()?.get(pinout.b1_GPIO)?.into_output(),
                              b2_gpio: Gpio::new()?.get(pinout.b2_GPIO)?.into_output(),
                              b3_gpio: Gpio::new()?.get(pinout.b3_GPIO)?.into_output(),
                              b4_gpio: Gpio::new()?.get(pinout.b4_GPIO)?.into_output() };
  Ok(gpio)
}

fn pick_color(frequency: f32) -> i32 {
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
  count
}

// function to find the peak frequency from a vector of samples by performing
// a fast fourier transform on the samples. this part of my program borrows
// heavily from the sources listed in my README.md on github.com/mollynova/
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
