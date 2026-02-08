use crate::meta;

pub enum CliAction {
  Version,
  About,
  Run,
}

pub fn parse_args() -> CliAction {
  let args: Vec<String> = std::env::args().collect();

  if args.iter().any(|a| a == "--version" || a == "-V") {
    CliAction::Version
  } else if args.iter().any(|a| a == "--about" || a == "--help") {
    CliAction::About
  } else {
    CliAction::Run
  }
}

pub fn print_version() {
  println!(
    "{} {} ({} {})",
    meta::PROJECT_NAME,
    meta::VERSION,
    meta::GIT_COMMIT,
    meta::GIT_DATE
  );
}

pub fn print_about() {
  crate::utils::slogan(false, false);
  println!();
  println!("Version: {}", meta::VERSION);
  println!("Commit: {}", meta::GIT_COMMIT);
  println!("Build: {}", meta::BUILD_PROFILE);
  println!("Maintainer: {}", meta::PROJECT_MAINTAINER);
  println!("Repository (canonical): {}", meta::PROJECT_REPOSITORY);
  println!("Documentation: {}/README.md", meta::PROJECT_REPOSITORY);
}
