use regex::Regex;

fn main() {
    let emailoid = Regex::new(r"\w+\s*\S?(@|at)\S?\s*\w+\s*\S?(\.|dot)\S?\s+com")
        .unwrap_or_else(|_| unreachable!());
    let space = Regex::new(r"\s").unwrap_or_else(|_| unreachable!());
    let atoid = Regex::new(r"\S?(@|at)\S?").unwrap_or_else(|_| unreachable!());

    for f_cont in std::env::args()
        // ignore program name
        .skip(1)
        // files should be small,
        // otherwise blame the user
        .map(std::fs::read_to_string)
        .filter_map(|f| match f {
            Ok(s) => Some(s),
            // warn on error
            Err(e) => {
                eprintln!("{e}");
                None
            }
        })
    {
        emailoid
            .find_iter(&f_cont)
            .map(|m| space.replace_all(m.as_str(), ""));
    }
}
