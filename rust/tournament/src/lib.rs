use std::str::FromStr;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::fmt;

pub fn tally(input : &String) -> String {
    let mut team_results = tally_internal(input).into_iter().collect::<Vec<(String, TeamTally)>>();

    team_results.as_mut_slice().sort_by(compare_results);

    let team_results = team_results.into_iter().map(|x| {
        format!("{:31}|{}", x.0.to_string(), x.1)
    });

    let mut result = format!("{:31}|{:>3} |{:>3} |{:>3} |{:>3} |{:>3}", "Team", "MP", "W", "D", "L", "P");
    for r in team_results { result.push_str("\n"); result.push_str(&r); }
    result
}

fn tally_internal(input : &String) -> HashMap<String, TeamTally> {
    let mut accumulator = HashMap::new();
    for result in input.split('\n')
                       .filter_map(|line| line.parse().ok()) {
        match result {
            MatchResult::Draw{team1, team2} => {
                accumulator.entry(team1).or_insert(TeamTally::new()).draw += 1;                
                accumulator.entry(team2).or_insert(TeamTally::new()).draw += 1;                
            },
            MatchResult::NonDraw {winning_team, losing_team} => {
                accumulator.entry(winning_team).or_insert(TeamTally::new()).won += 1;                
                accumulator.entry(losing_team).or_insert(TeamTally::new()).lost += 1;                
            }
        }
    }

    accumulator
}

fn compare_results(team1 : &(String, TeamTally), team2 : &(String, TeamTally)) -> Ordering {
    let first_order = team2.1.get_points().cmp(&team1.1.get_points());
    if first_order != Ordering::Equal { first_order } else { team1.0.cmp(&team2.0) }
}

struct TeamTally {
    won : u32,
    draw : u32,
    lost : u32
}

impl TeamTally {
    fn new() -> TeamTally {
        TeamTally {won : 0, draw : 0, lost : 0}
    }

    fn get_matches_played(&self) -> u32 {
        self.won + self.draw + self.lost
    }
    
    fn get_points(&self) -> u32 {
        self.won * 3 + self.draw
    }
}

impl fmt::Display for TeamTally {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:>3} |{:>3} |{:>3} |{:>3} |{:>3}", 
            self.get_matches_played(), self.won, self.draw, self.lost, self.get_points())
    }
}

enum MatchResult {
    NonDraw {winning_team : String, losing_team : String},
    Draw {team1 : String, team2 : String}
}

impl FromStr for MatchResult {
    type Err = &'static str;

    fn from_str(s : &str) -> Result<MatchResult, &'static str> {
        let splits = s.split(';').collect::<Vec<&str>>();
        if splits.len() != 3 {
            return Err("Wrong # of parts");
        }

        if splits[2] == "win" {
            Ok(MatchResult::NonDraw{winning_team : splits[0].to_string(), losing_team : splits[1].to_string()})
        }
        else if splits[2] == "loss" {
            Ok(MatchResult::NonDraw{winning_team : splits[1].to_string(), losing_team : splits[0].to_string()})
        }
        else if splits[2] == "draw" {
            Ok(MatchResult::Draw{team1 : splits[0].to_string(), team2 : splits[1].to_string()})
        }
        else {
            Err("Invalid match outcome")
        }
    }
}



/*
Lifetimes issue:

#[macro_use]
extern crate lazy_static;

use std::str::FromStr;

pub fn tally(input : &String) -> String {
    "".to_string()
}

enum MatchResult<'a> {
    NonDraw {winning_team : &'a str, losing_team : &'a str},
    Draw {team1 : &'a str, team2 : &'a str}
}

impl <'a> FromStr for MatchResult<'a> {
    type Err = &'static str;

    fn from_str(s : &'a str) -> Result<MatchResult<'a>, &'static str> {
        let mut splits = Vec::new();
        for x in s.split(';') { splits.push(x); }
        if splits.len() != 3 {
            return Err("Wrong # of parts");
        }

        match splits[2] {
            "win" => Ok(MatchResult::NonDraw{winning_team : splits[0], losing_team : splits[1]}),
            "lose" => Ok(MatchResult::NonDraw{winning_team : splits[1], losing_team : splits[0]}),
            "draw" => Ok(MatchResult::Draw{team1 : splits[0], team2 : splits[1]}),
            _ => Err("Invalid match outcome")
        }
    }
}
*/