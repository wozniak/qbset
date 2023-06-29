#![allow(unused)]

use std::io::{Read, Write};
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

trait WriteHelpers: Write {
    fn write_string(&mut self, str: &str) -> std::io::Result<()> {
        self.write_u16(str.len() as u16)?;
        self.write_all(str.as_bytes())
    }
    fn write_u16(&mut self, num: u16) -> std::io::Result<()> {
        self.write_all(&num.to_le_bytes())
    }
    fn write_u8(&mut self, num: u8) -> std::io::Result<()> {
        self.write_all(&[num])
    }
}
impl<T> WriteHelpers for T where T: Write {}

/// A quizbowl set that contains one or more packets.
#[derive(Debug, Clone)]
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
        let file_version = reader.read_u8()?;

        let custom_cats_len = reader.read_u8()? as usize;
        let mut custom_categories = Vec::with_capacity(custom_cats_len);
        for _ in 0..custom_cats_len {
            let name = reader.read_string()?;
            let general = Subcategory::try_from(reader.read_u8()?)?.broad_category();
            custom_categories.push(CustomCategory { name, general });
        }

        let packets_len = reader.read_u8()? as usize;
        let mut packets = Vec::with_capacity(packets_len);
        for _ in 0..packets_len {
            packets.push(Packet::read_from(&mut reader)?);
        }

        Ok(Self {
            name,
            year,
            custom_categories,
            packets,
        })
    }

    /// Write the file out to a Writer
    pub fn write_to<W: Write>(&self, mut writer: W) -> Result<()> {
        // write the file magic
        writer.write(b"QbSet\0")?;
        writer.write_string(&self.name)?;
        writer.write_u16(self.year)?;
        writer.write_u8(self.custom_categories.len() as u8)?;
        for custom in &self.custom_categories {
            writer.write_string(&custom.name)?;
            writer.write_u8(custom.general.as_subcat_other() as u8)?;
        }
        writer.write_u8(self.packets.len() as u8);
        for packet in &self.packets {
            packet.write_to(&mut writer)?;
        }
        Ok(())
    }
}
