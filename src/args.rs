use lazy_static::lazy_static;
use structopt::StructOpt;

use std::path::PathBuf;

lazy_static! {
    pub static ref ARGS: RoomNodeOptions = RoomNodeOptions::from_args();
}

#[derive(StructOpt, Debug)]
#[structopt(name = "home-system-room-node")]
pub struct RoomNodeOptions {
    /// The port that http server listens to
    #[structopt(short, long, default_value = "8072")]
    pub port: u16,

    /// The name of location where the hardware is placed
    #[structopt(short, long)]
    pub location: String,

    /// The linux device name of I2C
    #[structopt(long, parse(from_os_str), default_value = "/dev/i2c-2")]
    pub i2c_device: PathBuf,
}
