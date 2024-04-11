use pest::iterators::Pair;

use crate::parser::Rule;

pub fn get_pair_location(pair: &Pair<Rule>) -> (usize, usize) {
    let span = pair.as_span();
    (span.start(), span.end())
}
