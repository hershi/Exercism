use std::collections::HashMap;
use std::cmp::Ordering;
use std::fmt;

pub fn tally(input : &String) -> String {
    // Get the per-team results and sort them by score, then by name
    let mut team_results = tally_internal(input).into_iter().collect::<Vec<(&str, TeamTally)>>();
    team_results.as_mut_slice().sort_by(compare_by_score_then_by_name);

    // Tranform each result object to a string
    let team_results = team_results.iter().map(|x| {
        format!("{:31}|{}", x.0, x.1)
    });

    // Initialize the result with the header, then ad all the result strings
    let mut result = format!("{:31}|{:>3} |{:>3} |{:>3} |{:>3} |{:>3}", "Team", "MP", "W", "D", "L", "P");
    for r in team_results { result.push_str("\n"); result.push_str(&r); }
    result
}

fn compare_by_score_then_by_name(team1 : &(&str, TeamTally), team2 : &(&str, TeamTally)) -> Ordering {
    match team2.1.get_points().cmp(&team1.1.get_points()) {
        Ordering::Equal => team1.0.cmp(&team2.0),
        x => x ,
    }
}

fn tally_internal(input : &String) -> HashMap<&str, TeamTally> {
    let mut accumulator = HashMap::new();

    // Split the input into lines, 
    // Parse each line and ignore the ones that fail parsing
    // For each parsed line, update the win/loss/draw count for the teams
    for result in input.split('\n')
                       .filter_map(|line| MatchResult::from_str(line).ok()) {
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

// The TeamTally objects represent the aggregate results for a single team - 
// how many wins, losses, and draws it had, as well as the score and matches-played (inferred
// based on the wins/losses/draws).
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

// MatchResult instances describe the result of a single match. They are used
// to parse the textual input into a structured form.
enum MatchResult<'a> {
    NonDraw {winning_team : &'a str, losing_team : &'a str},
    Draw {team1 : &'a str, team2 : &'a str}
}

impl <'a> MatchResult<'a> {
    fn from_str<'b>(s : &'b str) -> Result<MatchResult<'b>, &'static str> {
        let splits = s.split(';').collect::<Vec<&'b str>>();
        if splits.len() != 3 {
            return Err("Wrong # of parts");
        }

        match splits[2] {
            "win" => Ok(MatchResult::NonDraw{winning_team : splits[0], losing_team : splits[1]}),
            "loss" => Ok(MatchResult::NonDraw{winning_team : splits[1], losing_team : splits[0]}),
            "draw" => Ok(MatchResult::Draw{team1 : splits[0], team2 : splits[1]}),
            _ => Err("Invalid match outcome"),
        }
    }
}