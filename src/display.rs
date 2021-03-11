
//! This is the display abstract for the 8x8 matrix

pub struct Display {

    /// The actual bytes sent to the matrix via the GPIO
    bytes: [u8; 8]

}


impl Display {

    pub fn new() -> Self {
        Self{bytes: [0u8; 8]}
    }

    /// Clears the display
    pub fn clear(&mut self) {
        self.bytes.fill(0);
    }

    /// Set the value of a pixel on or off
    pub fn set(&mut self, x: u8, y: u8, b: bool) {
        if b {
            self.bytes[x as usize] |= 1 << y;
        } else {
            self.bytes[x as usize] &= !(1 << y);
        }
    }


    /// Temp measure ? so can hack on the screen
    pub fn borrow_bytes(&self) -> &[u8] {
        &self.bytes
    }

    


}