use regex::Regex;

fn main() {
    let emailoid = Regex::new(r"\w+\s*\S?(@|at)\S?\s*\w+\s*\S?(\.|dot)\S?\s+com")
        .unwrap_or_else(|_| unreachable!());
    let space = Regex::new(r"\s").unwrap_or_else(|_| unreachable!());

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
                eprintln!("{e}");
                None
            }
        })
        .map(|s| {
            emailoid
                .find_iter(&s)
                .map(|m| space.replace(m.as_str(), ""))
        });
}
