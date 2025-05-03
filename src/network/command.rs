use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub enum Command {
    OPEN,
    JOIN {
        ticket: String
    }
}