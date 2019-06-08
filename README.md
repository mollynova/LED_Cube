Raspberry Pi Raver Cube
Author: Molly Novash

OVERVIEW:

Raspberry Pi B+ Raver Cube

For my project, I built a 4x4x4 LED cube with gradient colors. Each layer of the cube has 4 sets of 4 LED
lights soldered together. Each set will be uniquely addressable (16 unique addresses). The cube connects via
gpio wires to raspberry pi B+ GPIO pins.

The raspberry pi runs software I wrote which takes a .wav file as input and transforms it into a vector of samples.
This vector of samples gets subdivided into a vector of vectors of samples, each of which has approximately one second
of music's worth of samples, based on the length of the song.

I use RustFFT to perform a fast fourier transform (FFT) on each chunk of samples, deconstructing the sine wave into the
individual tones it is composed of. I select the highest of these tones, and turn on the appropriate colored lights on
the cube. Lowest tones are red, low-middle are yellow, middle-high are green, and high are blue.

I have the lights move in a gradient-type pattern from one end of the cube to the other. Since I have 4 layers in
the cube, and each FFT is on one second's worth of samples, I have each layer turn on one after another for 0.25 seconds.

The overall effect is a kind of "light show" based on the frequency of the song.

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
"WHAT DOESN'T WORK/LESSONS LEARNED"

Everything works as intended. I tested my program out when the cube was still on the bread board and it worked as expected.
I tested all of the LEDs to make sure that none of them were burnt out. I'm still not completely finished (as of Friday 6/7)
with the hardware, but I will have it done by the end of the weekend.

I learned a lot of lessons with this project, the main one probably being: if you know virtually nothing about hardware,
building an LED cube will take a lot longer than you think it will.

I tried out a lot of different ideas as far as how best to connect the lights and make the whole thing basically symmetrical.
I ended up using some copper tubing for the outline of the cube, which looks cool, but if I could go back I probably would
have skipped that step and just used bare wire for the whole thing, because it took a long time to put together and it's much
more difficult to solder thick copper than regular wire since you have to get it so hot. I learned a neat trick on youtube
for making wire stiff to use it structurally- you cut a piece of wire, insert it into a drill like a bit, hold the other end
with plyers, and run the drill for a bit. It twists it right up and you end up with a rigid piece of wire. That's how I made
the internal structure of the cube.

One thing that took a lot more time than I expected was all the soldering. I initially soldered all of my LEDs into chains of
four by connecting anode directly to cathode, and I didn't realize that a lot of the LEDs I picked up at the EPL had different
lengths of anode and cathode (I think some of them come from different sources) so my chains ended up being different sizes
and I had to go back and redo most of them (I should have noticed that).

I learned a lot about how modules work in Rust with this project, and it was good experience looking around for different crates
that might be useful and cloning them, and figuring out how to edit my dependencies in Cargo.toml to get everything working.
I also solidified some error checking concepts while doing my testing. Rust's Result type reminds me a lot of Haskell, and for
the most part I think it's intuitive.

I re-learned that I should map out a project better ahead of time instead of trying to dissect it once the majority of it is
working. Karla, I am sorry.

I also learned a lot more about FFTs. There's definitely a difference between watching a youtube video and listening about it
in class versus actually having to get it working. I think this part of the project was really useful because it cleared a
lot of the concepts up for me, which was really valuable.

---------------
Works sampled / some lines of code pulled from:

https://rickyhan.com/jekyll/update/2018/02/06/rust-guitar-pedal-effects-dsp.html
http://siciarz.net/24-days-rust-hound/

---------------
Special thanks to Christopher Clark and the EPL for their help with the hardware end of this project.
