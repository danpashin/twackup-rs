pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IO error")]
    Io(#[from] std::io::Error),

    #[error("Importing requires executing apt command. Please, consider switching to root user.")]
    NotRunningAsRoot,

    #[error("plist error")]
    Plist(#[from] plist::Error),
}
