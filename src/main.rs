#![forbid(unsafe_code)]


use rand::{thread_rng, Rng};
use vilakuy::IdealAdc;


fn main() {

    let mut rng: rand::rngs::ThreadRng = thread_rng();
    let num_cases: i32 = 100000;

    for _n in 1..num_cases {
        let random_bit_depth: u32 = rng.gen_range(1..=32);
        // let adc = IdealAdc{
        //     bit_depth: random_bit_depth,
        //     normalized: false,
        //     ..Default::default()
        // };
        // let unsigned_random_value: f32 = rng.gen_range(adc.min() as f32  ..=adc.max() as f32);
        let signed_adc = IdealAdc{
            bit_depth: random_bit_depth, 
            signed: true,
            normalized: false,
        };
        let signed_random_value: f32 = rng.gen_range(signed_adc.min() as f32  ..=signed_adc.max() as f32);
        // println!("{}", adc);
        // println!("the unsigned value {} on bits is {}", unsigned_random_value, adc.to_bits_string(unsigned_random_value));
        // println!("the unsigned value recovered is {}", adc.to_value(adc.to_bits(unsigned_random_value)));
        println!("{}", signed_adc);
        println!("the signed value {} on bits is {}", signed_random_value, signed_adc.to_bits_string(signed_random_value));
        println!("the signed value recovered is {}", signed_adc.to_value(signed_adc.to_bits(signed_random_value)));
    }
/*     let signed_adc = IdealAdc{
        bit_depth: 12, 
        signed: true,
        normalized: true,
    };
    let value_test = -0.67;
    println!("{}", signed_adc);
    println!("the signed value {} on bits is {}", value_test, signed_adc.to_bits_string(value_test));
    println!("the signed value recovered is {}", signed_adc.to_value(signed_adc.to_bits(value_test))); */
 
}
