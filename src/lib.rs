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
        
        write!(f, "{} ADC of {} levels ({} bits), MAX: {}, MIN: {}", if self.signed {"signed"} else {"unsigned"}, self.levels(), self.bit_depth, self.max(), self.min())
    }
}

impl IdealAdc {
    pub fn to_bits(&self, value: f32) -> Vec<bool>{
        let _sampled: f32 = self._sample(value);
        let _holded: f32 = self._hold(_sampled);
        let _quantitized: i32 = self._quantize(_holded);
        let _encoded = self._encoder(_quantitized);
        return _encoded;
    }
    pub fn to_bits_string(&self, value: f32) -> String{
        let _bits = self.to_bits(value);
        let mut _bits_string = String::new();
        _bits_string.push('[');
        for _bit in _bits {
            let _bit_unsigner_integer = _bit as u8;
            _bits_string.push_str(_bit_unsigner_integer.to_string().as_str());
        }
        _bits_string.push(']');
        return _bits_string;
    }
    fn _sample(&self, value: f32) -> f32 {
        return value;
    }
    fn _hold(&self, value: f32) -> f32 {
        return value;
    }
    fn _quantize(&self, value: f32) -> i32 {
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
    fn _encoder(&self, value: i32) -> Vec<bool> {
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
        return _bits;
    }
    fn levels(&self) -> u64 {
        let _option_level: Option<u64> = u64::checked_pow(2, self.bit_depth);
        match _option_level {
            Some(level) => level,
            None => u32::MAX as u64,
        }
    }
    pub fn max(&self) -> i64{
        let _levels: i64 =(self.levels() as i64).try_into().unwrap();
        if self.signed {
            return _levels/2-1;
        }
        else{
            return _levels-1;
        }
    }
    pub fn min(&self) -> i64{
        let _levels: i64 =(self.levels() as i64).try_into().unwrap();
        if self.signed {
            return self.max()-_levels+1;
        }
        else{
            return self.max()-_levels+1;
        }
    }
}
