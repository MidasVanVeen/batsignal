use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Lists available battery devices and returns EXIT_SUCCESS
    #[arg(short, long)]
    pub list: bool,

    /// Wait until all conditions pass, sleeping for `interval` milliseconds before checking
    /// again.
    #[arg(long)]
    pub wait: bool,

    /// The interval to use when waiting. Specified in milliseconds.
    #[arg(long, default_value_t = 5000)]
    pub interval: u64,

    /// When this flag is set the program will print debug information to stdout.
    #[arg(long)]
    pub verbose: bool,

    /// What battery id to use, see `list`.
    #[arg(long)]
    pub id: Option<usize>,

    /// Specify a state condition, possible values are 'charging', 'discharging', 'empty' and 'full'.
    #[arg(long)]
    pub state: Option<battery::State>,
    /// Specify a not-state condition, possible values are 'charging', 'discharging', 'empty' and 'full'.
    #[arg(long)]
    pub not_state: Option<battery::State>,
    /// Specify a less-than condition on the remaining charge in percent.
    #[arg(long)]
    pub lt: Option<u8>,
    /// Specify a greater-than condition on the remaining charge in percent.
    #[arg(long)]
    pub gt: Option<u8>,
}
