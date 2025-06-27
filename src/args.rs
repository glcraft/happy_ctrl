use clap::{Parser, Subcommand};

#[derive(Subcommand)]
pub enum Command {
    /// Turns on the device
    PowerOn,
    /// Turns off the device
    PowerOff,
    /// Set custom color
    SetRGB {
        ///Amount of red (0-255)
        r: u8,
        ///Amount of green (0-255)
        g: u8,
        ///Amount of blue (0-255)
        b: u8,
    },
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Sets the bluetooth adapter name to use
    #[arg(short, long)]
    pub adapter: Option<String>,
    /// Sets the bluetooth device name to connect on
    pub name: String,
    /// Increase the verbosity
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
    /// Command type to send to the device
    #[command(subcommand)]
    pub command: Command,
}
