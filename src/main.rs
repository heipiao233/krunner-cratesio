use std::{fmt::Display, iter::once};

use crates_io::{Crate, Registry};
use curl::easy::Easy;
use krunner::{Match, MatchIcon, MatchType, RunnerExt};

#[derive(krunner::Action, Debug, Clone)]
enum Action {
    #[action(id = "cratesio", title = "crates.io", icon = "cratesio")]
    CratesIo,
    #[action(id = "docsrs", title = "docs.rs", icon = "docsrs")]
    DocsRs,
    #[action(id = "librs", title = "lib.rs", icon = "librs")]
    LibRs,
}

struct Runner(Registry);

struct Error(String);

impl<E> From<E> for Error
where
    E: std::error::Error,
{
    fn from(value: E) -> Self {
        Self(value.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn crate_to_match(from: &Crate, query: String) -> Match<Action> {
    let (ty, relevance) = if from.name == query {
        (MatchType::ExactMatch, 0.7)
    } else {
        (MatchType::PossibleMatch, 0.5)
    };
    Match {
        id: from.name.clone(),
        title: from.name.clone(),
        subtitle: from.description.clone(),
        icon: MatchIcon::ByName("cratesio".to_string()),
        ty,
        relevance: relevance,
        urls: vec![
            format!("https://crates.io/crate/{}", from.name),
            format!("https://lib.rs/{}", from.name),
            format!("https://docs.rs/{}", from.name),
        ],
        category: None,
        multiline: false,
        actions: vec![Action::CratesIo, Action::DocsRs, Action::LibRs],
    }
}

impl krunner::Runner for Runner {
    type Action = Action;
    type Err = Error;

    fn matches(&mut self, query: String) -> Result<Vec<Match<Self::Action>>, Self::Err> {
        if !query.starts_with("crate ") {
            return Ok(Vec::new());
        }
        let query = query[6..].to_string();
        let matches = self.0.search(query.as_str(), 10)?.0;
        let matches = matches
            .iter()
            .map(|i| crate_to_match(i, query.clone()))
            .chain(once(Match {
                id: format!("search {query}"),
                title: format!("Search {query}"),
                subtitle: None,
                icon: MatchIcon::ByName("search-icon".to_string()),
                ty: MatchType::HelperMatch,
                relevance: 0.6,
                urls: vec![
                    format!("https://crates.io/search?q={query}"),
                    format!("https://docs.rs/releases/search?query={query}"),
                    format!("https://lib.rs?{query}"),
                ],
                category: None,
                multiline: false,
                actions: vec![Action::CratesIo, Action::DocsRs, Action::LibRs],
            }))
            .collect();
        println!("{:#?}", matches);
        Ok(matches)
    }

    fn run(&mut self, match_id: String, action: Option<Self::Action>) -> Result<(), Self::Err> {
        open::that(match (action, match_id.strip_prefix("search ")) {
            (Some(Action::CratesIo), Some(query)) => format!("https://crates.io/search?q={query}"),
            (Some(Action::DocsRs), Some(query)) => {
                format!("https://docs.rs/releases/search?query={query}")
            }
            (Some(Action::LibRs) | None, Some(query)) => format!("https://lib.rs?{query}"),
            (Some(Action::CratesIo), None) => format!("https://crates.io/crates/{match_id}"),
            (Some(Action::DocsRs), None) => format!("https://docs.rs/{match_id}"),
            (Some(Action::LibRs) | None, None) => format!("https://lib.rs/{match_id}"),
        })?;
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut handle = Easy::new();
    handle.useragent("krunner-cratesio 0.1.0 (mailto:heipiao233@outlook.com)")?;
    let registry = Registry::new_handle(String::from("https://crates.io"), None, handle, false);
    Runner(registry).start("net.heipiao.krunner-cratesio", "/CrateRunner")?;
    Ok(())
}
