#![forbid(unsafe_code)]
use std::{fmt};
use bitvec::{prelude::*, macros::internal::funty::Fundamental};

pub struct IdealAdc {
    pub bit_depth: u32,
    pub signed: bool,
}

impl Default for IdealAdc {
    fn default() -> IdealAdc {
        IdealAdc {
            bit_depth: 8,
            signed: false,
        }
    }
}

impl fmt::Display for IdealAdc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        write!(f, "{} ADC of ({} bits), MAX: {}, MIN: {}", if self.signed {"signed"} else {"unsigned"}, self.bit_depth, self.max(), self.min())
    }
}

impl IdealAdc {
    pub fn to_bits(&self, value: f32) -> Vec<bool>{
        let quantitized: i32 = self.quantize(value);
        
        self.encoder(quantitized)
    }
    pub fn to_value(&self, bits: Vec<bool>) -> f32 {
        let decoded = self.decoder(bits);
        self.continuized(decoded)
    }
    pub fn to_bits_string(&self, value: f32) -> String{
        let bits = self.to_bits(value);
        let mut bits_string = String::new();
        bits_string.push('[');
        for bit in bits {
            let bit_unsigner_integer = bit as u8;
            bits_string.push_str(bit_unsigner_integer.to_string().as_str());
        }
        bits_string.push(']');
        bits_string
    }
    fn quantize(&self, value: f32) -> i32 {
        let max = self.max() as f32;
        let min = self.min() as f32;
        if value > max {
            max.round() as i32
        } else if value < min {
            min.round() as i32
        } else {
            value.round() as i32
        }
    }
    fn continuized(&self, value: i32) -> f32 {
        value.as_f32()
    }
    fn encoder(&self, value: i32) -> Vec<bool> {
        let _bit_depth_usize = self.bit_depth as usize;
        let mut _bits: Vec<bool> = vec![false;_bit_depth_usize];
        let mut _bit_index: usize = 0;

        // decompose integer into array of byte
        let raw_value_bytes = value.to_le_bytes();
        // iterate each byte and view the bits
        for _byte in raw_value_bytes {
            let _bit_slice = _byte.view_bits::<Lsb0>();
            // append each bit into a vector of bits
            for _bool_slice in _bit_slice {
                // break when the vector of bits is already filled
                if _bit_index >= _bit_depth_usize {
                    break;
                }
                let _bit = !_bool_slice.as_bool();
                let bit_index_usize = _bit_index as usize;
                _bits[_bit_depth_usize-bit_index_usize-1]=_bit;
                _bit_index+=1;
            }
            if _bit_index >= _bit_depth_usize {
                break;
            }
        }
        _bits
    }
    fn decoder(&self, bits: Vec<bool>) -> i32 {
        let mut value = 0i32;
        for bit in &bits {
            print!("{}",bit.as_u8())
        }
        if self.signed {
            let mut bits_mut = bits;
            bits_mut.reverse();
            let option_sign = bits_mut.pop();
            bits_mut.reverse();
            let sign: bool = option_sign.unwrap_or(false);
            for bit in &bits_mut {
                print!("{}",bit.as_u8())
            }
            for (pos, bit) in bits_mut.iter().rev().enumerate() {
                let option_integ =  i32::checked_pow(2, pos.as_u32());
                let integ = match option_integ {
                    Some(integ) => integ,
                    None => i32::MAX,
                };
                println!("{}",bit);
                if sign {
                    value += !bit.as_i32() * integ;
                } else {
                    value += bit.as_i32() * integ;
                } 
            }
        } else {
            for (pos, bit) in bits.iter().rev().enumerate() {
                let option_integ =  i32::checked_pow(2, pos.as_u32());
                let integ = match option_integ {
                    Some(integ) => integ,
                    None => i32::MAX,
                };
                value += bit.as_i32() * integ;
            }
        }
        value
        
    }
    pub fn max(&self) -> i32{
        if self.signed {
            let option_max: Option<i32> = i32::checked_pow(2, self.bit_depth-1);
            match option_max {
                Some(max) => max-1,
                None => i32::MAX,
            }
        }
        else{
            let option_max: Option<i32> = i32::checked_pow(2, self.bit_depth);
            match option_max {
                Some(max) => max-1,
                None => i32::MAX
                ,
            }
        }
        
    }
    pub fn min(&self) -> i32{
        if self.signed {
            let option_min: Option<i32> = i32::checked_pow(2, self.bit_depth-1);
            match option_min {
                Some(min) => -min,
                None => i32::MIN,
            }
        }
        else{
            0
        }
    }
}
