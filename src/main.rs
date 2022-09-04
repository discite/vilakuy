
use rand::{thread_rng, Rng};
use vilakuy::IdealAdc;


fn main() {
    /*let mut bit_reader = BitReader::new(&raw_bytes);
    let extracted_value: i32 = bit_reader.read_i32(32).unwrap();
    println!("{}", extracted_value);*/

    //let slice_of_u8 = &[0b0000_0000];
    //let mut bit_reader = BitReader::new(slice_of_u8);
    //let a_single_bit = bit_reader.read_u8(1).unwrap();

    
    let mut rng: rand::rngs::ThreadRng = thread_rng();
    let num_cases: i32 = 100000;

    for _n in 1..num_cases {
        let random_bit_depth: u32 = rng.gen_range(1..=32);
        let adc = IdealAdc{
            bit_depth: random_bit_depth, 
            ..Default::default()
        };
        let unsigned_random_value: f32 = rng.gen_range(adc.min() as f32  ..=adc.max() as f32);
        let signed_adc = IdealAdc{
            bit_depth: random_bit_depth, 
            signed: true,
        };
        let signed_random_value: f32 = rng.gen_range(signed_adc.min() as f32  ..=signed_adc.max() as f32);
        println!("{}", adc);
        println!("the unsigned value {} on bits is {}", unsigned_random_value, adc.to_bits_string(unsigned_random_value));
        println!("{}", signed_adc);
        println!("the signed value {} on bits is {}", signed_random_value, signed_adc.to_bits_string(signed_random_value));
    } 
}
