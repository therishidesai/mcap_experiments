use std::fs;

use serde_derive::{Serialize, Deserialize};
use serde_cbor;

use anyhow::{Context, Result};
use camino::Utf8Path;
use memmap::Mmap;

#[derive(Serialize, Deserialize, Debug)]
struct Pose {
    x: f32,
    y: f32,
    z: f32,
}

fn map_mcap<P: AsRef<Utf8Path>>(p: P) -> Result<Mmap> {
    let fd = fs::File::open(p.as_ref()).context("Couldn't open MCAP file")?;
    unsafe { Mmap::map(&fd) }.context("Couldn't map MCAP file")
}

fn main() -> Result<()> {
    let mapped: Mmap = map_mcap("out.mcap")?;

    for message in mcap::MessageStream::new(&mapped)? {
        let m = message?;
        let deserialized_pose: Pose = serde_cbor::from_slice(&m.data).unwrap();
        println!("{:?}", deserialized_pose);
        // Or whatever else you'd like to do...
    }
    Ok(())
}
