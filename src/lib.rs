mod core;
mod tui;

pub use {
    crate::core::{ 
        App,
        RawArgs,
        Context 
    },
    tui::{ 
        Buffer,
        Cell,
        Region
    },
};

use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("could not find path \'{}\'", path.display()))]
    PathDNE { path: std::path::PathBuf },

    IO { source: std::io::Error },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
