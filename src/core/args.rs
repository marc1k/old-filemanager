use {
    std::path::PathBuf,
    clap::{
        crate_authors,
        crate_description,
        crate_version,
        Clap
    },
};

/// Raw, unvalidated launch arguments parsed from command line.
#[derive(Debug, Clap)]
#[clap(
    name = "opal",
    about = crate_description!(),
    version = crate_version!(),
    author = crate_authors!(),
)]
pub struct RawArgs {
    /// The path to open.
    #[clap(name = "PATH", index = 1, parse(from_os_str))]
    pub path: Option<PathBuf>
}
