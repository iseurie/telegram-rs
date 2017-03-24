use error::*;
use byteorder::{WriteBytesExt, LittleEndian, BigEndian};
use std::any::Any;

macro_rules! impl_serialize {
    ($type:path, $write:path) => {
        impl Serialize for $type {
            #[inline]
            fn serialize_to(&self, buffer: &mut Vec<u8>) -> Result<()> {
                $write(buffer, *self)?;

                Ok(())
            }
        }
    };
}

pub trait Serialize {
    /// Serialize to the passed buffer.
    fn serialize_to(&self, buffer: &mut Vec<u8>) -> Result<()>;
}

impl Serialize for bool {
    #[inline]
    fn serialize_to(&self, buffer: &mut Vec<u8>) -> Result<()> {
        if *self {
            buffer.write_i32::<LittleEndian>(-1720552011)?;
        } else {
            buffer.write_i32::<LittleEndian>(-1132882121)?;
        }

        Ok(())
    }
}

impl Serialize for i8 {
    #[inline]
    fn serialize_to(&self, buffer: &mut Vec<u8>) -> Result<()> {
        buffer.push(*self as u8);

        Ok(())
    }
}

impl Serialize for u8 {
    #[inline]
    fn serialize_to(&self, buffer: &mut Vec<u8>) -> Result<()> {
        buffer.push(*self);

        Ok(())
    }
}

impl_serialize!(i16, WriteBytesExt::write_i16<LittleEndian>);
impl_serialize!(i32, WriteBytesExt::write_i32<LittleEndian>);
impl_serialize!(i64, WriteBytesExt::write_i64<LittleEndian>);

impl_serialize!(u16, WriteBytesExt::write_u16<LittleEndian>);
impl_serialize!(u32, WriteBytesExt::write_u32<LittleEndian>);
impl_serialize!(u64, WriteBytesExt::write_u64<LittleEndian>);

impl_serialize!(f32, WriteBytesExt::write_f32<LittleEndian>);
impl_serialize!(f64, WriteBytesExt::write_f64<LittleEndian>);

impl Serialize for i128 {
    #[inline]
    fn serialize_to(&self, buffer: &mut Vec<u8>) -> Result<()> {
        (*self as u128).serialize_to(buffer)
    }
}

impl Serialize for u128 {
    #[inline]
    fn serialize_to(&self, buffer: &mut Vec<u8>) -> Result<()> {
        buffer.write_u64::<BigEndian>((*self >> 64) as u64)?;
        buffer.write_u64::<BigEndian>(*self as u64)?;

        Ok(())
    }
}

impl Serialize for String {
    fn serialize_to(&self, buffer: &mut Vec<u8>) -> Result<()> {
        let len = self.len();

        if len <= 253 {
            // If L <= 253, the serialization contains one byte with the value of L,
            // then L bytes of the string followed by 0 to 3 characters containing 0,
            // such that the overall length of the value be divisible by 4,
            // whereupon all of this is interpreted as a sequence
            // of int(L/4)+1 32-bit little-endian integers.

            buffer.push(len as u8);
        } else {
            // If L >= 254, the serialization contains byte 254, followed by 3
            // bytes with the string length L in little-endian order, followed by L
            // bytes of the string, further followed by 0 to 3 null padding bytes.

            buffer.push(254);
            buffer.write_uint::<LittleEndian>(len as u64, 3)?;
        }

        // Write each character in the string
        buffer.extend(self.as_bytes());

        // [...] string followed by 0 to 3 characters containing 0,
        // such that the overall length of the value be divisible by 4 [...]
        let rem = len % 4;
        if rem > 0 {
            for _ in 0..(4 - rem) {
                buffer.push(0);
            }
        }

        Ok(())
    }
}

impl<T: Serialize> Serialize for Vec<T> {
    fn serialize_to(&self, buffer: &mut Vec<u8>) -> Result<()> {
        // Write type identifier (for Vec)
        buffer.write_u32::<LittleEndian>(0x1cb5c415u32)?;

        // Write length
        let len = buffer.len() as u32;
        buffer.write_u32::<LittleEndian>(len)?;

        // Write elements
        for element in self {
            // FIXME: Ensure vector elements are serialized as bare types
            element.serialize_to(buffer)?;
        }

        Ok(())
    }
}

impl Serialize for Box<Any> {
    fn serialize_to(&self, buffer: &mut Vec<u8>) -> Result<()> {
        match self.downcast_ref::<Box<Serialize>>() {
            Some(as_ser) => as_ser.serialize_to(buffer),

            None => {
                // FIXME: Return an error
                panic!("Serialize not implemented")
            }
        }
    }
}
