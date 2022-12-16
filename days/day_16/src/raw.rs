use std::str::FromStr;

use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Valve {
    pub ident: String,
    pub flow_rate: u32,
    pub connections: Vec<String>,
}

static RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"Valve ([A-Z]{2}) has flow rate=([0-9]*); tunnels? leads? to valves? ([A-Z, ]*)"#)
        .unwrap()
});

impl FromStr for Valve {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = RE.captures(s).unwrap();

        let ident = captures.get(1).unwrap().as_str();
        let flow_rate = captures.get(2).unwrap().as_str().parse().unwrap();
        let connections = captures.get(3).unwrap().as_str();

        Ok(Valve {
            ident: ident.into(),
            flow_rate,
            connections: connections.split(", ").map(Into::into).collect(),
        })
    }
}
