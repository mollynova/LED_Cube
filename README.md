Raspberry Pi Raver Cube
Author: Molly Novash

OVERVIEW:

Raspberry Pi B+ Raver Cube

For my project, I'm building a 4x4x4 LED cube with gradient colors. Each layer of the cube will have 4 sets of 4 LED
lights soldered together. Each set will be uniquely addressable (16 unique addresses). The cube will be connected via
ribbon cable to raspberry pi B+ GPIO pins.

The raspberry pi will be running software which takes a wav file as input and transforms it into a vector of samples.
This vector of samples will be subdivided into a vector of vectors of samples, each of which has approximately one second
of music's worth of samples, based on the length of the song.

I use RustFFT to perform a fast fourier transform (FFT) on each chunk of samples, deconstructing the sine wave into the
individual tones it is comprised of. I select the highest of these tones, and turn on the appropriate colored lights on
the cube. Lowest tones are red, low-middle are yellow, middle-high are green, and high are blue.

----------
SOFTWARE:

Typically, frequencies in music fall in the range of 30Hz ~ 3500Hz. This would be the low end of bass versus
the higher end of a violin. After completing most of the software for this project, I wrote a tester function that
printed the highest frequency per second of music for a given .wav file on several samples of music, and rarely got
a result higher than 500Hz. Because of this, I decided on a testing range of 30Hz - 500Hz. Anything higher than
500Hz will still light up the color associated with "high frequency." I divided this range into seven equal
chunks. Each color of LED lights is associated with a "chunk" of frequency ranges: Red, Red-Yellow, Yellow,
Yellow-Green, Green, Green-Blue, and Blue.

As stated in the overview, I use the 'hound' crate to parse a .wav file into a vector of samples. Each sample
is essentially a sine wave representing the particular frequency of the sample (tone). Based on the length of
the song and the total number of samples I read in, I subdivided the vector of samples into a vector of vectors
of samples, where each subvector has approximately one second's worth of samples. Then, I send that subvector
of samples to a function that performs a fast fourier transform (FFT) on the vector in order to detect the
individual notes (frequencies) that the sample is composed of. For this, I used the crate RustFFT. It then selects
the highest of these frequencies and passes that to a function that decides the appropriate color of lights to turn
on for that frequency.

I use the rppal crate to initialize my hardware components and to turn on the lights. rppal uses BCM pin numbering
for GPIO, so I set 16 constant BCM pin values, for which I associate 4 with each color. Then, I set each of those
pins to output mode so that they're ready to toggle my LEDs.

----------
HARDWARE:

Layout of the cube:

  -4x4x4 LEDs soldered together, which can be thought of as four 4x4 flat planes of LEDs.
   The bottom layer is "layer 1" and the top is "layer 4"

   The layers are laid out as follows, where R = red, Y = yellow, G = green, B = blue:

   Layer 1      Layer 2      Layer 3      Layer 4

   B G Y R      G B R Y      Y R B G      R Y G B
   G B R Y      B G Y R      R Y G B      Y R B G
   Y R B G      R Y G B      B G Y R      G B R Y
   R Y G B      Y R B G      G B R Y      B G Y R

   For each layer, like colors are all connected together (along with transistors, resistors, and a 12V 3A power supply)
   Each layer has 4 lights of each color
   Each grouping of 4 connected lights per layer is uniquely addressable
   This requires 16 total GPIO pins. 4 pins are associated with each color, one for each layer of the cube. They're toggled
   with the rppal crate for accessing GPIO pins in Rust.

   I'm using a 12V 3A external power supply for this project. Red Green and Yellow lights are getting 10V of power and blue
   lights are getting 12V.

---------------
TESTING:

For the first part of my testing, I'm just making sure my FFT is working correctly. I used "audiocheck.net" to
generate and download 1-second-long .wav files at 400Hz, 1400Hz, 2200Hz, and 3000Hz. I wrote a test function to
make sure that the result of the FFT function was equal to the frequency of the .wav file being tested.

My next test, once I knew the algorithm was working properly, was to make sure my rppal code was working as well. I did this in
the breadboard stage of hardware development. I attached my external power supply to power and ground on my breadboard. Then, I
connected the middle transistor pin on a single row of lights to a GPIO pin on my raspberry pi. I wrote a simple tester function
that would use rppal syntax to set that particular GPIO pin to "output" mode, and turn on the lights.

Once I'd gotten the algorithm working and the lights triggering correctly, everything else was basically just repetition.

---------------
Works sampled / some lines of code pulled from:

https://rickyhan.com/jekyll/update/2018/02/06/rust-guitar-pedal-effects-dsp.html
http://siciarz.net/24-days-rust-hound/

---------------
Special thanks to Christopher Clark and the EPL for their help with the hardware end of this project.
