mod util;
use util::unwrapable_r;

use regex::Regex;

use std::process::ExitCode;

fn main() -> ExitCode {
    let is_emailoid = unwrapable_r(Regex::new(
        r"\w++\s*+\S??(@|at)\S??\s*+(?:[a-zA-Z\d\-]++\s*+\S??(\.|dot)\S??\s*+)++[a-zA-Z\d\-]{1,24}+",
    ));
    let space = unwrapable_r(Regex::new(r"\s"));
    let atoid = unwrapable_r(Regex::new(r"\S??(@|at)\S??"));
    let dotoid = unwrapable_r(Regex::new(r"\S??(\.|dot)\S??"));

    let mut stat = ExitCode::SUCCESS;

    std::env::args()
        // ignore program name
        .skip(1)
        // files should be small,
        // otherwise blame the user
        .map(std::fs::read_to_string)
        .filter_map(|f| match f {
            Ok(s) => Some(s),
            // warn on error
            Err(e) => {
                stat = ExitCode::FAILURE;
                eprintln!("{e}");
                None
            }
        })
        .for_each(|file_content| {
            // find all sub-strs that look like email addresses
            for emailoid in is_emailoid.find_iter(&file_content) {
                let em_no_space = space.replace_all(emailoid.as_str(), "");
                let em_sane_at = atoid.replace_all(&em_no_space, "@");
                let email = dotoid.replace_all(&em_sane_at, ".");
                println!("{email}");
            }
        });
    stat
}
