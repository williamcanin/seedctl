use crate::meta;
use console::style;
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
  println!(
    "{}",
    style(format!("About: {}", "-".repeat(67))).cyan().bold()
  );
  println!("{}{}", style("- Version: ").bold().yellow(), meta::VERSION);
  println!(
    "{}{}",
    style("- Commit: ").bold().yellow(),
    meta::GIT_COMMIT
  );
  println!(
    "{}{}",
    style("- Build: ").bold().yellow(),
    meta::BUILD_PROFILE
  );
  println!(
    "{}{}",
    style("- Maintainer: ").bold().yellow(),
    meta::PROJECT_MAINTAINER
  );
  println!(
    "{}{}",
    style("- Repository: ").bold().yellow(),
    meta::PROJECT_REPOSITORY
  );
  println!(
    "{}{}/README.md",
    style("- Documentation: ").bold().yellow(),
    meta::PROJECT_REPOSITORY
  );
}
