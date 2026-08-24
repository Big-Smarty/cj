use std::fmt::Display;

use anyhow::Result;

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct PciId {
    pub domain: u32,
    pub bus: u32,
    pub device: u32,
    pub function: u32,
}

impl PciId {
    pub fn new(domain: u32, bus: u32, device: u32, function: u32) -> Self {
        Self {
            domain,
            bus,
            device,
            function,
        }
    }
}

impl TryFrom<&str> for PciId {
    type Error = anyhow::Error;

    fn try_from(bus_id: &str) -> Result<Self, Self::Error> {
        let (domain, remainder) = bus_id
            .split_once(':')
            .ok_or(anyhow::Error::msg("Failed to parse domain ID"))?;
        let (bus, remainder) = remainder
            .split_once(':')
            .ok_or(anyhow::Error::msg("Failed to parse bus ID"))?;
        let (device, function) = remainder
            .split_once('.')
            .ok_or(anyhow::Error::msg("Failed to parse device ID"))?;

        Ok(Self::new(
            u32::from_str_radix(domain, 16)?,
            u32::from_str_radix(bus, 16)?,
            u32::from_str_radix(device, 16)?,
            u32::from_str_radix(function, 16)?,
        ))
    }
}

impl Display for PciId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{:04x}:{:02x}:{:02x}.{:x}",
            self.domain, self.bus, self.device, self.function,
        ))
    }
}
