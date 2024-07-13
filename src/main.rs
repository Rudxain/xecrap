use regex::Regex;

fn main() {
    let re = Regex::new(r"(?-u:\w+\s*.?(at|@).?\s*\w+\s*.?(\.|dot).?\s+com)")
        .unwrap_or_else(|_| unreachable!());

    for file in std::env::args()
        // ignore program name
        .skip(1)
        // files should be small,
        // otherwise we blame the user
        .map(std::fs::read_to_string)
        // ignore errors
        .filter_map(Result::ok)
    {}
}
