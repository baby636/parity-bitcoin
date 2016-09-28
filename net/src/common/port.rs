use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use ser::{Serializable, Stream, Deserializable, Reader, Error as ReaderError};

#[derive(Debug, PartialEq)]
pub struct Port(u16);

impl From<u16> for Port {
	fn from(port: u16) -> Self {
		Port(port)
	}
}

impl From<Port> for u16 {
	fn from(port: Port) -> Self {
		port.0
	}
}

impl Serializable for Port {
	fn serialize(&self, stream: &mut Stream) {
		stream.write_u16::<BigEndian>(self.0).unwrap();
	}
}

impl Deserializable for Port {
	fn deserialize(reader: &mut Reader) -> Result<Self, ReaderError> where Self: Sized {
		Ok(try!(reader.read_u16::<BigEndian>().map(Port)))
	}
}

#[cfg(test)]
mod	tests {
	use ser::{serialize, deserialize};
	use super::Port;

	#[test]
	fn test_port_serialize() {
		assert_eq!(serialize(&Port::from(1)), "0001".into());
		assert_eq!(serialize(&Port::from(0x1234)), "1234".into());
	}

	#[test]
	fn test_port_deserialize() {
		assert_eq!(Port::from(1), deserialize(&[0x00, 0x01]).unwrap());
		assert_eq!(Port::from(0x1234), deserialize(&[0x12, 0x34]).unwrap());
	}
}
