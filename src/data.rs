/*!
Types for esports data from HLTV.


*/
use chrono::{DateTime, Utc};

/// Type of event. At this moment either LAN or online.
pub enum EventType {
    LAN,
    Online,
}

/// Best-of-X format of a match. Since there are 7 maps in the CSGO map pool,
/// the maximum is Bo7.
pub enum MatchFormat {
    Bo1,
    Bo3,
    Bo5,
    Bo7,
}

/// Basic player information.
#[derive(Default, Debug, PartialEq)]
pub struct Player {
    pub id: u64,
    pub nickname: String,
}

/// Basic information about a team.
pub struct Team {
    /// HLTV-associated ID (found in the URL of team page).
    pub id: u64,
    /// Name of the team.
    pub name: String,
}

/// Basic information about a team.
pub struct Event {
    /// HLTV-associated ID (found in the URL of the event page).
    pub id: u64,
    /// Name of the Event
    pub name: String,
}

/// Contains detailed information about an event. Corresponds to data found on [HLTV's event
/// page](https://www.hltv.org/events/6345/blast-premier-spring-final-2022).
pub struct EventDetails {
    /// HLTV-associated ID (found in the URL of the event page).
    pub id: u64,
    /// Name of the event.
    pub name: String,
    /// Date when the event starts.
    pub start_date: DateTime<Utc>,
    /// Date when the event finished.
    pub end_date: DateTime<Utc>,
    /// Price pool of the event. Can be a USD figure, or guaranteed spots in another tournament.
    pub price_pool: String,
}

/// Contains extensive information about a team.
pub struct TeamDetails {
    pub id: u64,
    pub name: String,
    pub players: [Player; 5],
}

/// Contains a summary of match data.
pub struct Match {
    pub id: u64,
    /// First team of the mach, according to HLTV's display order
    pub team1: Option<String>,
    /// Second team of the mach, according to HLTV's display order
    pub team2: Option<String>,
    /// Name of the event.
    pub event: String,
    /// Format of a match. For example, if the format is [`Bo1`][MatchFormat::Bo1],
    /// then only one map is played and the result is either a `1-0` or `0-1`.
    pub format: MatchFormat,
    /// Result of a match. If the match has not concluded, the result is [`None`].
    pub result: Option<MatchResult>,
    /// Time when an upcoming match is supposed to start. If the match is finished,
    /// this date is the finish time (according to HLTV).
    pub date: DateTime<Utc>,
    /// Number of HLTV stars given to the match. Stars are a measure of match prestige.
    /// The exact meaning is defined
    /// [here](https://www.hltv.org/forums/threads/931435/what-are-these-stars-by-the-matches#r12178822).
    pub stars: u64,
}

/// Contains detailed information about a match. Corresponds to data found on [HLTV's
/// match page](https://www.hltv.org/matches/2239492/nip-vs-virtuspro-sltv-starseries-v-finals).
pub struct MatchDetails {
    pub id: u64,
    pub team1: Option<Team>,
    pub team2: Option<Team>,
    pub event: Event,
    pub format: MatchFormat,
    pub result: Option<MatchResult>,
}

/// Refers to either the first or second team in a match, according to HLTV order.
pub enum WhichTeam {
    First,
    Second,
    None,
}

/// Represents the result of a single map. Examples are: `16-14`, `10-16`, `19-17`
pub struct MapScore {
    pub map: String,
    pub team1_rounds: u64,
    pub team2_rounds: u64,
}

/// Collection of performance metrics of a player.
pub struct Stats {
    /// Total kills.
    pub kills: u64,
    /// Total deaths.
    pub deaths: u64,
    /// Average damage per round.
    pub adr: f64,
    /// Percentage of rounds with either kill, assisst, support or trade.
    pub kast: f64,
    /// HLTV 2.0 rating.
    pub rating: f64,
}

/// This is the map score of a match. Examples are: 1-0, 2-1, 1-2, 3-0, etc.
pub struct MatchResult {
    pub winner: WhichTeam,
    /// Number of maps won by first team.
    pub team1_maps: u64,
    /// Number of maps won by first team.
    pub team2_maps: u64,
}
