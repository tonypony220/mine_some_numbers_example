use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Number of zeros in hash
    #[arg(short = 'N')]
    pub num_zeros: usize,

    /// Number of digits having N zeros in tail
    #[arg(short = 'F')]
    pub num_digits_to_find: usize,
}
