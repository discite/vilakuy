#![forbid(unsafe_code)]
use std::{fmt};
use bitvec::{prelude::*, macros::internal::funty::Fundamental};

pub struct IdealAdc {
    pub bit_depth: u32,
    pub signed: bool,
    pub normalized: bool,
}

impl Default for IdealAdc {
    fn default() -> IdealAdc {
        IdealAdc {
            bit_depth: 8,
            signed: false,
            normalized: false,
        }
    }
}

impl fmt::Display for IdealAdc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        write!(f, "{} ADC of ({} bits), MAX: {}, MIN: {}", if self.signed {"signed"} else {"unsigned"}, self.bit_depth, self.max(), self.min())
    }
}

impl IdealAdc {
    fn continuized(&self, value: i32) -> f32 {
        let max = self.max() as f32;
        let min = self.min() as f32;
        let mut value_mut = value as f32;
        if self.normalized {
            println!("{}", value_mut);
            value_mut = value_mut / (max-min);
            println!("{}", value_mut);
            value_mut
        } else {
            value as f32
        }
    }
    fn decoder(&self, bits: Vec<bool>) -> i32 {
        let mut value: i32 = 0i32;
        let sign = *bits.first().unwrap_or(&false);
        if self.signed && sign {
            value=!value;
        }
        bits.iter().for_each(|&bit| {
            value <<= 1;
            value ^= bit.as_i32();
        });
        value
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
    pub fn max(&self) -> i32{
        let max;
        if self.signed {
            let option_max: Option<i32> = i32::checked_pow(2, self.bit_depth-1);
            max = match option_max {
                Some(max) => max-1,
                None => i32::MAX,
            };
            max
        }
        else{
            let option_max: Option<i32> = i32::checked_pow(2, self.bit_depth);
            max = match option_max {
                Some(max) => max-1,
                None => i32::MAX,
            };
            max
        }
    }
    pub fn min(&self) -> i32{
        let min: i32;
        if self.signed {
            let option_min: Option<i32> = i32::checked_pow(2, self.bit_depth-1);
            min = match option_min {
                Some(min) => -min,
                None => i32::MIN,
            };
            min
        }
        else{
            0
        }
    }
    fn quantize(&self, value: f32) -> i32 {
        let max = self.max() as f32;
        let min = self.min() as f32;
        let mut value_mut = value;

        if self.normalized {
            let max_normalize:f32 = 1.0;
            let min_normalize:f32 = if self.signed { -1.0 } else { 0.0 };
            if value_mut > max_normalize {
                value_mut = max_normalize;
            } else if value_mut < min_normalize {
                value_mut = min_normalize;
            }
        } 

        if value_mut > max {
            max.round() as i32
        } else if value_mut < min {
            min.round() as i32
        } else {
            if self.normalized {
                value_mut = value_mut * (max-min);
                value_mut = value_mut.round();
                value_mut as i32
            } else {
                value_mut.round() as i32
            }
        }
    }
    pub fn to_bits(&self, value: f32) -> Vec<bool>{
        let quantitized: i32 = self.quantize(value);
        
        self.encoder(quantitized)
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
    pub fn to_value(&self, bits: Vec<bool>) -> f32 {
        let decoded = self.decoder(bits);
        self.continuized(decoded)
    }
}
