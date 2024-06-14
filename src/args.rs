use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Args)]
pub struct StringArgs {
    #[arg(short, long, default_value_t = 20)]
    pub length: u32,

    #[arg(name="type", short, long, value_enum, default_value_t = StringType::Ascii)]
    pub mode: StringType,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum StringType {
    Ascii,
    Letters,
    Numbers,
    Extended,
    Hex,
}

#[derive(Args)]
pub struct NumberArgs {
    pub min: isize,
    pub max: isize,
}

#[derive(Args)]
pub struct BooleanArgs {
    #[arg(name="type", short, long, value_enum, default_value_t = BooleanType::TrueFalse)]
    pub mode: BooleanType,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum BooleanType {
    TrueFalse,
    YesNo,
    Numeric,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short, long, default_value_t = 1)]
    pub count: u32,
}

#[derive(Subcommand)]
pub enum Commands {
    String(StringArgs),
    Number(NumberArgs),
    Boolean(BooleanArgs),
}
