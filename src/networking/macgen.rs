use rand::RngCore;
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub struct MacAddress([u8; 6]);

impl Display for MacAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let octet = &self.0;
        // Converts each byte to hexadecimals and formats them with a colon
        write!(
            f,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            octet[0], octet[1], octet[2], octet[3], octet[4], octet[5]
        )
    }
}

impl MacAddress {
    pub fn new() -> MacAddress {
        let mut octets: [u8; 6] = [0; 6];
        rand::thread_rng().fill_bytes(&mut octets);
        // Sets the first octet to be a unicast address
        octets[0] |= 0b0000_0001;
        MacAddress(octets)
    }

    pub fn is_local(&self) -> bool {
        (self.0[0] & 0b0000_0010) == 0b0000_0010
    }

    pub fn is_unicast(&self) -> bool {
        (self.0[0] & 0b0000_0001) == 0b0000_0001
    }
}
