#![allow(unused)]

use std::io::Read;
use thiserror::Error;

mod categories;
mod packet;

pub use categories::*;
pub use packet::*;

// our error type

#[derive(Error, Debug)]
pub enum Error {
    #[error("malformed input: {0}")]
    MalformedInput(String),
    #[error("IO error")]
    Io(#[from] std::io::Error),
    #[error("UTF-8 Error")]
    Utf8(#[from] std::string::FromUtf8Error),
}

pub type Result<T> = std::result::Result<T, Error>;

// helper trait around read that gives us a few convenience functions (not public!!!)
trait ReadHelpers: Read {
    fn read_string(&mut self) -> Result<String> {
        let len = self.read_u16()?;
        let mut vec = vec![0; len as usize];
        self.read_exact(&mut vec)?;
        Ok(String::from_utf8(vec)?)
    }
    fn read_u16(&mut self) -> std::io::Result<u16> {
        let mut n = [0u8; 2];
        self.read_exact(&mut n)?;
        Ok(u16::from_le_bytes(n))
    }
    fn read_u8(&mut self) -> std::io::Result<u8> {
        let mut byte = [0u8];
        self.read_exact(&mut byte)?;
        Ok(byte[0])
    }
}
impl<T> ReadHelpers for T where T: Read {}

/// A quizbowl set that contains one or more packets.
#[derive(Clone)]
pub struct Set {
    pub name: String,
    pub year: u16,
    custom_categories: Vec<CustomCategory>,
    pub packets: Vec<Packet>,
}

impl Set {
    /// Export this set to an HTML document.
    pub fn to_html(&self) -> String {
        unimplemented!()
    }

    /// Read in this set from some kind of reader (usually a file)
    pub fn from_reader<R: Read>(mut reader: R) -> Result<Self> {
        // check file magic
        let mut magic = [0u8; 6];

        reader.read(&mut magic)?;
        if &magic != b"QbSet\0" {
            return Err(Error::MalformedInput("invalid file magic".into()));
        }
        let name = reader.read_string()?;
        let year = reader.read_u16()?;
        let packets_len = reader.read_u8()? as usize;
        let file_version = reader.read_u8()?;

        let custom_cats_len = reader.read_u8()? as usize;
        let mut custom_categories = Vec::with_capacity(custom_cats_len);
        for _ in 0..custom_cats_len {
            let name = reader.read_string()?;
            let general = Subcategory::try_from(reader.read_u8()?)?.broad_category();
            custom_categories.push(CustomCategory { name, general });
        }

        let mut packets = Vec::with_capacity(packets_len);
        for _ in 0..packets_len {
            packets.push(Packet::read_from(&mut reader)?);
        }

        Ok(Self {
            name, year, custom_categories, packets
        })
    }
}
