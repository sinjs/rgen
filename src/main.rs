mod args;

use std::iter::repeat_with;

use args::{BooleanType, Cli, Commands, StringType};
use clap::Parser;
use rand::Rng;

const ASCII_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

const HEX_CHARSET: &[u8] = b"abcdef0123456789";

const EXTENDED_CHARSET: &[u8] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+-_$#/@!";

const LETTERS_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

const NUMBERS_CHARSET: &[u8] = b"0123456789";

fn main() {
    let cli = Cli::parse();

    let mut rng = rand::thread_rng();

    for _ in 0..cli.count {
        match cli.command {
            Commands::String(ref args) => {
                let charset = match args.mode {
                    StringType::Ascii => ASCII_CHARSET,
                    StringType::Extended => EXTENDED_CHARSET,
                    StringType::Letters => LETTERS_CHARSET,
                    StringType::Numbers => NUMBERS_CHARSET,
                    StringType::Hex => HEX_CHARSET,
                };

                let one_char = || char::from(charset[rng.gen_range(0..charset.len())]);
                let result: String = repeat_with(one_char).take(args.length as usize).collect();

                println!("{}", result);
            }

            Commands::Number(ref args) => {
                let number = rng.gen_range(args.min..args.max);

                println!("{}", number);
            }

            Commands::Boolean(ref args) => {
                let boolean: bool = rng.gen();
                let result = match args.mode {
                    BooleanType::Numeric => {
                        if boolean {
                            "1"
                        } else {
                            "0"
                        }
                    }
                    BooleanType::TrueFalse => {
                        if boolean {
                            "true"
                        } else {
                            "false"
                        }
                    }
                    BooleanType::YesNo => {
                        if boolean {
                            "yes"
                        } else {
                            "no"
                        }
                    }
                };

                println!("{}", result);
            }
        };
    }
}
