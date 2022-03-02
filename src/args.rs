use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Source gitlab URL
    pub source: String,
    /// Source gitlab API token
    pub source_token: String,
    /// Destination gitlab URL
    pub dest: String,
    /// Destination gitlab API Token
    pub dest_token: String,
    /// API source username
    pub assignee: String,
    /// API destination username
    #[clap(short, long)]
    pub dest_user: Option<String>,
    /// Answer yes to all questions
    #[clap(short, long)]
    pub yes: bool,
}
