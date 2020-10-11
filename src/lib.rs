//! # git-cmd
//! This crate provides an easy way to abstract over the git cli tool. It works
//! by using the builder pattern to create the command you want to run in a type
//! safe way. Then by calling `make_cmd()` you're given the resulting
//! `std::process::Command` type which you can either execute as is or further
//! manipulate it with say env vars. It should be easy to get what you need in
//! a standardized interface instead of having to create a `Command` everytime
//! yourself and possibly messing up the arguments. The entry point to every
//! command is with the `Git` struct. Take a look at the docs there to get an
//! understanding of what the crate is currently capable of.

mod init;
pub use crate::init::*;

/// This type entry way to all the git commands. While you can just make the struct
/// like so: `let git = Git;` it's recommended to instead use it as part of the
/// builder patern which this crate utilizes. You should call the command you
/// want via a function call and build up your arguments. For example:
///
/// ```
/// # use git_cmd::Git;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // This is equivalent to calling `git init`
/// if Git::init().make_cmd().output().unwrap().status.success() {
///   println!("Yay we initialized a new repo!");
/// }
/// #   Ok(())
/// # }
/// ```
pub struct Git;
impl Git {
  /// Creates a builder for the `git init` subcommand
  pub fn init() -> GitInitBuilder {
    GitInitBuilder::new()
  }
}

