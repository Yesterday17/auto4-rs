pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub fn to_ass_style_formatted(&self) -> String {
        format!("&H{:2X}{:2X}{:2X}{:2X}", self.a, self.b, self.g, self.r)
    }
}

pub struct Time(u32);

impl Time {
    /// Get millisecond, rounded to centi-second precision
    /// Always round up for 5ms because the range is [start, stop)
    pub fn get(&self) -> u32 {
        (self.0 + 5) - (self.0 + 5) % 10
    }

    pub fn get_ass_formatted(&self, ms_precision: bool) -> String {
        // let mut ret = String::with_capacity(10 + ms_precision);
        todo!()
    }
}