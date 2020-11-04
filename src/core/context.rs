use {
    crate::Result,
    super::args::RawArgs,
    std::{
        env,
        convert::TryFrom,
        path::PathBuf
    },
    snafu::{
        ensure,
        ResultExt
    },
};

/// Validated user-specified launch context.
pub struct Context {
    root: PathBuf
}

impl TryFrom<RawArgs> for Context {
    type Error = crate::Error;

    /// Attempts to construct `Context` from unvalidated launch arguments.
    fn try_from(mut args: RawArgs) -> Result<Self> {     
        match args.path {
            Some(path) => {
                // Ensure specified path exists
                ensure!(path.exists(), crate::PathDNE { path });

                // TODO: Handle case where specified path points to directory
                
                args.path = Some(path.canonicalize().context(crate::IO)?);
            },
            None => { 
                // No path specified, default to current working directory
                args.path = Some(env::current_dir().context(crate::IO)?);
            }
        }

        Ok(Self {
            // Safe unwrap because all match arms resolve to an assignment of `Some(T)`
            root: args.path.unwrap()
        })
    }
}
