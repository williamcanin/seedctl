pub fn slogan() -> String {
  let content = r#"
      		....                 ....
                 -+++++=:       .-=+++++
                  =+++++++:    =+++++++-
                   -++++==+-  ++==++++:
                     :=++=-+:===+++=:
                        .++:+=-+.
                            +:
                            =-
                         .-=++-:.
                       -++++-==++=:
                      =++ ._||. :+++-
                     :+++  |__]  +++:
                     -+++ _|__]  +++-
                      ++=   ||  -+++-
                       =++==:=++++:
                        .:=++++-:
                            :.
  "#;
  content.to_string()
}

pub fn program_name_banner(version: &str) -> String {
  let mut version_text = format!("version: {}", version);

  if version.is_empty() {
    version_text = "".to_string();
  }

  let content: String = format!(
    r#"
             ___             _  ___ _____ _
            / __| ___ ___ __| |/ __|_   _| |
            \__ \/ -_) -_) _` | (__  | | | |__
            |___/\___\___\__,_|\___| |_| |____| {version_text}
  "#
  );
  content
}
