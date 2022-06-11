use hidapi::{HidApi, HidDevice, HidResult};

pub struct MCP2221 {
    dev: HidDevice,    
}


impl MCP2221 {
    pub fn new(vid: Option<u16>, pid: Option<u16>) -> Self {
        let vid = vid.unwrap_or(0x04D8);
        let pid = pid.unwrap_or(0x00DD);

        let hid = HidApi::new().unwrap();

        let dev = match hid.open(vid, pid) {
            Ok(dev) => dev,
            Err(e) => {
                eprintln!("Error: {}", e);
                panic!();
            },
        };

        return MCP2221 {dev};
    }

    pub fn device_info(&self) -> HidResult<()> { 
        println!("Manufacturer: {}", self.dev.get_manufacturer_string()?.as_deref().unwrap_or("Unset"));
        println!("Product: {}", self.dev.get_product_string()?.as_deref().unwrap_or("Unset"));
        println!("Serial: {}", self.dev.get_serial_number_string()?.as_deref().unwrap_or("Unset"));
        return Ok(());
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mcp = MCP2221::new(None, None);
        mcp.device_info().unwrap();
    }
}
