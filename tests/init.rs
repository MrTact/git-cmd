use git_cmd::*;
use tempdir::TempDir;
use std::{fs, env};

#[test]
fn git_init() {
  let dir = TempDir::new("git_init").unwrap();
  env::set_current_dir(dir.path()).unwrap();
  assert_eq!(&env::current_dir().unwrap(), dir.path());
  let out = Git::init()
    .make_cmd()
    .output()
    .unwrap();
  assert!(!out.stdout.is_empty());
  assert!(out.status.success());
}

#[test]
fn git_init_directory() {
  let dir = TempDir::new("git_init").unwrap();
  let out = Git::init()
    .directory(dir.path())
    .make_cmd()
    .output()
    .unwrap();
  assert!(!out.stdout.is_empty());
  assert!(out.status.success());
}

#[test]
fn git_init_quiet() {
  let dir = TempDir::new("git_init").unwrap();
  let out = Git::init()
    .quiet()
    .directory(dir.path())
    .make_cmd()
    .output()
    .unwrap();
  assert!(out.stdout.is_empty());
  assert!(out.status.success());
}

#[test]
fn git_init_initial_branch() {
  let dir = TempDir::new("git_init").unwrap();
  let out = Git::init()
    .directory(dir.path())
    .initial_branch("main")
    .make_cmd()
    .output()
    .unwrap();
  let head = fs::read_to_string(dir.path().join(".git").join("HEAD")).unwrap();
  assert_eq!(head.as_str(), "ref: refs/heads/main\n");
  assert!(!out.stdout.is_empty());
  assert!(out.status.success());
}

#[test]
fn git_init_bare() {
  let dir = TempDir::new("git_init").unwrap();
  let out = Git::init()
    .directory(dir.path())
    .bare()
    .make_cmd()
    .output()
    .unwrap();
  assert!(dir.path().join("HEAD").exists());
  assert!(!out.stdout.is_empty());
  assert!(out.status.success());
}

#[test]
fn git_init_object_format() {
  let dir = TempDir::new("git_init").unwrap();
  let out = Git::init()
    .directory(dir.path())
    .object_format(Hash::Sha1)
    .make_cmd()
    .output()
    .unwrap();
  assert!(!out.stdout.is_empty());
  assert!(out.status.success());

  let dir2 = TempDir::new("git_init").unwrap();
  let out = Git::init()
    .directory(dir2.path())
    .object_format(Hash::Sha256)
    .make_cmd()
    .output()
    .unwrap();
  // Since git is not always built with the option we need to test the failure
  // case. By virtue of having the error we know that the command was shelled
  // out and fed to git correctly
  if out.status.success() {
    assert!(!out.stdout.is_empty());
  } else {
    assert_eq!(
      "fatal: The hash algorithm sha256 is not supported in this build.\n",
      String::from_utf8_lossy(&out.stderr)
    );
  }
}

#[test]
fn git_init_separate_git_dir() {
  let dir = TempDir::new("git_init").unwrap();
  let dir2 = TempDir::new("git_init").unwrap();
  let out = Git::init()
    .directory(dir.path())
    .separate_git_dir(dir2.path())
    .make_cmd()
    .output()
    .unwrap();
  assert!(dir2.path().join("HEAD").exists());
  assert!(!out.stdout.is_empty());
  assert!(out.status.success());
}

#[test]
// template dir is really confusing. Basically it will copy anything that exists
// from a directory into .git. This is NOT meant to be a template for how an
// init repo looks like for your code, but how your .git folder will look like.
// This is perfect for things like hooks and what not, but the docs don't make
// it really seem like that at first glance.
fn git_init_template() {
  let dir = TempDir::new("git_init").unwrap();
  let temp_file = dir.path().join("template");
  let out = Git::init()
    .directory(dir.path())
    .make_cmd()
    .output()
    .unwrap();
  assert!(!out.stdout.is_empty());
  assert!(out.status.success());
  fs::write(&temp_file, b"gotta have some kind of commit").unwrap();

  let dir2 = TempDir::new("git_init").unwrap();
  let temp_file2 = dir2.path().join(".git").join("template");
  let out = Git::init()
    .directory(dir2.path())
    .template(dir.path())
    .make_cmd()
    .output()
    .unwrap();
  assert!(!out.stdout.is_empty());
  assert!(out.status.success());
  assert_eq!(fs::read_to_string(temp_file2).unwrap().as_str(), "gotta have some kind of commit");
}

#[test]
// TODO: Find a good way to test this beyond running the command for each of them
fn git_init_shared() {
  let init = |s| {
    let dir = TempDir::new("git_init").unwrap();
    let out = Git::init()
      .directory(dir.path())
      .shared(s)
      .make_cmd()
      .output()
      .unwrap();
    assert!(!out.stdout.is_empty());
    assert!(out.status.success());
  };

  init(Shared::All);
  init(Shared::Everybody);
  init(Shared::False);
  init(Shared::Group);
  init(Shared::Octal(0o777));
  init(Shared::True);
  init(Shared::Umask);
  init(Shared::World);
}
