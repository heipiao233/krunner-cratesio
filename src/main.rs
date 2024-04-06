use std::fmt::Display;

use crates_io::{Crate, Registry};
use curl::easy::Easy;
use krunner::{Match, MatchIcon, MatchType, RunnerExt};

#[derive(krunner::Action, Debug, Clone)]
enum Action {
    #[action(id = "cratesio", title = "Show in crates.io", icon = "cratesio")]
    CratesIo,
    #[action(id = "docsrs", title = "Show in docs.rs", icon = "docsrs")]
    DocsRs,
    #[action(id = "librs", title = "Show in lib.rs", icon = "librs")]
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
	let ty = if from.name == query {
		MatchType::ExactMatch
	} else {
		MatchType::PossibleMatch
	};
    Match {
        id: from.name.clone(),
        title: from.name.clone(),
        subtitle: from.description.clone(),
        icon: MatchIcon::ByName("cratesio".to_string()),
        ty,
        relevance: 0.5,
        urls: [
			format!("https://crates.io/crate/{}", from.name),
			format!("https://lib.rs/{}", from.name),
			format!("https://docs.rs/{}", from.name),
		].to_vec(),
        category: None,
        multiline: false,
        actions: [Action::CratesIo, Action::DocsRs, Action::LibRs].to_vec(),
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
		println!("{}", query);
        let matches = self.0
            .search(query.as_str(), 10)?
            .0;
		println!("{:?}", matches.iter().map(|i|i.name.clone()).collect::<Vec<String>>());
		let matches = matches
            .iter()
            .map(|i| crate_to_match(i, query.clone()))
            .collect();

        Ok(matches)
    }

    fn run(&mut self, match_id: String, action: Option<Self::Action>) -> Result<(), Self::Err> {
        match action {
			Some(Action::CratesIo) => open::that(format!("https://crates.io/crates/{match_id}"))?,
			Some(Action::DocsRs) => open::that(format!("https://docs.rs/{match_id}"))?,
			Some(Action::LibRs) | None => open::that(format!("https://lib.rs/{match_id}"))?,
		}
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
