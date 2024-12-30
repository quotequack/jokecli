use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct JokerArgs {
    /// Enter a delay between command and print joke
    pub delay: u64,
    /// Select a category:
    /// 1. Misc
    /// 2. Programming
    /// 3. Dark
    /// 4. Pun
    pub category: String,
}