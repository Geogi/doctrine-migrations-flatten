use clap::{crate_name, crate_version, crate_authors, crate_description};
use log::info;
use std::path::PathBuf;
use std::env;

fn main() -> Result<(), String> {
    env_logger::init();
    let app = clap::App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!());
    let mut version = vec![];
    app.write_version(&mut version)
        .map_err(|_| "Internal: could not read version.")?;
    let version = String::from_utf8_lossy(&version);
    info!("Running {}", version);
    let project_dir = project_dir()?;
    info!("Using project directory: {}", project_dir.to_string_lossy());
    let database_url = database_url()?;
    Ok(())
}

/// Finds the Symfony project directory.
///
/// Starting from the process current directory then its parents, looks for a file
/// named composer.json.
fn project_dir() -> Result<PathBuf, String> {
    let mut directory = env::current_dir()
        .map_err(|_| "Could not get current directory.")?;
    while !directory.join("composer.json").is_file() {
        if !directory.pop() {
            return Err("Could not find composer.json in current directory or its parents.".into());
        }
    }
    Ok(directory)
}

/// Finds the database URL.
///
/// Parses .env.local and .env to find out APP_ENV, then parses any .env.APP_ENV and
/// .env.APP_ENV.local to find out the DATABASE_URL.
fn database_url() -> Result<String, String> {
    unimplemented!()
}
