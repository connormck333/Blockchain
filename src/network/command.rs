use clap::Parser;

#[derive(Parser, Debug)]
pub enum Command {
    OPEN,
    JOIN {
        ticket: String
    }
}