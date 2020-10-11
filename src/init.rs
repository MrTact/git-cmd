use std::path::PathBuf;
use std::process::Command;

/// The builder for the `git init` command created by calling `Git::init()`
pub struct GitInitBuilder {
  quiet: bool,
  bare: bool,
  template: Option<PathBuf>,
  initial_branch: Option<String>,
  separate_git_dir: Option<PathBuf>,
  object_format: Option<Hash>,
  shared: Option<Shared>,
  directory: Option<PathBuf>,
}

impl GitInitBuilder {
  /// Internal function used by `Git`. `Git::init()` is just a wrapper around
  /// this function.
  pub(crate) fn new() -> Self {
    GitInitBuilder {
      quiet: false,
      bare: false,
      template: None,
      initial_branch: None,
      separate_git_dir: None,
      object_format: None,
      shared: None,
      directory: None,
    }
  }

  /// Only print error and warning messages; all other output will be suppressed
  pub fn quiet(mut self) -> Self {
    self.quiet = true;
    self
  }

  /// Create a bare repository. If GIT_DIR environment is not set, it is set to
  /// the current working directory.
  pub fn bare(mut self) -> Self {
    self.bare = true;
    self
  }

  /// Specify the directory from which templates will be used. Note that this is
  /// meant to be a template for what the `.git` directory looks like. Anything
  /// that's in the given directory will be copied into the `.git` directory of
  /// the newly initialized repository. Useful for example if you have hooks
  /// you want to use in all your repos
  pub fn template(mut self, path: impl Into<PathBuf>) -> Self {
    self.template = Some(path.into());
    self
  }

  /// Use the specified name for the initial branch in the newly created
  /// repository. If not specified, fall back to the default name: master.
  pub fn initial_branch(mut self, name: impl Into<String>) -> Self {
    self.initial_branch = Some(name.into());
    self
  }

  /// Instead of initializing the repository as a directory to either `$GIT_DIR`
  /// or `./.git/`, create a text file there containing the path to the actual
  /// repository. This file acts as filesystem-agnostic Git symbolic link to the
  /// repository.
  ///
  /// If this is reinitialization the repository will be move to the specified
  /// path.
  pub fn separate_git_dir(mut self, path: impl Into<PathBuf>) -> Self {
    self.separate_git_dir = Some(path.into());
    self
  }

  /// Specify the given object format (hash algorithm) for the repository. The
  /// valid values are sha1 and (if enabled) sha256. sha1 is the default.
  pub fn object_format(mut self, obj: Hash) -> Self {
    self.object_format = Some(obj);
    self
  }

  /// Specify that the Git repository is to be shared amongst several users.
  /// This allows users belonging to the same group to push into that
  /// repository. When specified, the config variable `core.sharedRepository`
  /// is set so that files and directories under $GIT_DIR are created with the
  /// requested permissions. When not specified, Git will use permissions
  /// reported by umask(2).
  pub fn shared(mut self, shared: Shared) -> Self {
    self.shared = Some(shared);
    self
  }

  /// Set a specific directory for the command. Using this is equivalent to `git
  /// init my_dog_bukka` where `my_dog_bukka` is the argument passed to this
  /// function.
  pub fn directory(mut self, path: impl Into<PathBuf>) -> Self {
    self.directory = Some(path.into());
    self
  }

  /// Consume the builder and output an `std::process::Command` object you can
  /// add env vars to etc. The command will not execute till you tell it to. See
  /// the standard library docs for more details
  pub fn make_cmd(self) -> Command {
    let mut cmd = Command::new("git");
    cmd.arg("init");

    if self.quiet {
      cmd.arg("--quiet");
    }

    if self.bare {
      cmd.arg("--bare");
    }

    if let Some(path) = self.template {
      cmd.arg("--template");
      cmd.arg(path);
    }
    if let Some(path) = self.separate_git_dir {
      cmd.arg("--separate-git-dir");
      cmd.arg(path);
    }
    if let Some(name) = self.initial_branch {
      cmd.arg("--initial-branch");
      cmd.arg(name);
    }
    if let Some(obj) = self.object_format {
      cmd.arg("--object-format");
      let arg = match obj {
        Hash::Sha1 => "sha1",
        Hash::Sha256 => "sha256",
      };
      cmd.arg(arg);
    }
    if let Some(shared) = self.shared {
      let arg = match shared {
        Shared::Umask => "umask".into(),
        Shared::False => "false".into(),
        Shared::Group => "group".into(),
        Shared::True => "true".into(),
        Shared::All => "all".into(),
        Shared::World => "world".into(),
        Shared::Everybody => "everybody".into(),
        // Formats the octal to the correct form of 0XXX
        Shared::Octal(perm) => format!("{:04o}", perm),
      };
      cmd.arg(format!("--shared={}", arg));
    }
    if let Some(path) = self.directory {
      cmd.arg(path);
    }
    cmd
  }
}

/// Options for the `shared` function. Note the default is Umask.
pub enum Shared {
  /// Use permissions reported by umask(2).
  Umask,
  /// Equivalent to `Umask`
  False,
  /// Make the repository group-writable, (and g+sx, since the git group may be
  /// not the primary group of all users). This is used to loosen the
  /// permissions of an otherwise safe umask(2) value. Note that the umask still
  /// applies to the other permission bits (e.g. if umask is 0022, using group
  /// will not remove read privileges from other (non-group) users).
  /// See `Octal` for how to exactly specify the repository permissions.
  Group,
  /// Equivalent to `Group`
  True,
  All,
  /// Equivalent to `All`
  World,
  /// Equivalent to `All`
  Everybody,
  /// You need to make sure you use a valid mode between the numbers 0o000 and
  /// 0o777 when using this otherwise you'll cause an error.
  /// An `0ctal` is a number where each file will have mode 0oxxx. 0oxxx will
  /// override users' umask(2) value (and not only loosen permissions as group
  /// and all does). 0o640 will create a repository which is group-readable,
  /// but not group-writable or accessible to others. 0o660 will create a repo
  /// that is readable and writable to the current user and group, but
  /// inaccessible to others.
  Octal(u16)
}

/// Which hash you want the repo to use when calling `object_format`. `Sha1` is
/// the default and `Sha256` might not be available if the cli tool was not built
/// with the option.
pub enum Hash {
  /// Ojects will use a sha1 hash
  Sha1,
  /// Ojects will use a sha256 hash
  Sha256
}
