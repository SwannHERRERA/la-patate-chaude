use clap::Parser;

#[derive(Parser, Default, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct ServerArgs {
    /// Server IP
    #[clap(short, value_parser, default_value = "127.0.0.1")]
    pub ip: String,

    /// Server port
    #[clap(long, value_parser, default_value_t = 7878)]
    pub port: u16,

    /// Log level
    #[clap(short, long, value_parser, default_value = "info")]
    pub log_level: String,

    /// Game type
    #[clap(short, long, value_parser, default_value = "hashcash")]
    pub game_type: String,

    /// Round Duration in seconds
    #[clap(short, long, value_parser, default_value = "3")]
    pub round_duration: u64,
}
