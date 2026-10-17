static STATIONS: [&str; 269] = include!("../data/stations.json");
static DISTANCES: [[[u8; 2]; 269]; 269] = include!("../data/distances.json");

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Station {
    idx: usize,
}

impl std::fmt::Display for Station {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ZoneOutcome {
    Correct,
    OneAway,
    TwoAway,
    MoreThanTwo,
}

impl ZoneOutcome {
    fn from_zones(zones: u8) -> Self {
        match zones {
            0 => Self::Correct,
            1 => Self::OneAway,
            2 => Self::TwoAway,
            _ => Self::MoreThanTwo,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Outcome {
    pub stops: u8,
    pub zones: ZoneOutcome,
}

impl std::fmt::Display for Outcome {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let Self { stops, zones } = self;
        let zones = match zones {
            ZoneOutcome::Correct => "0",
            ZoneOutcome::OneAway => "1",
            ZoneOutcome::TwoAway => "2",
            ZoneOutcome::MoreThanTwo => ">2",
        };
        write!(f, "{} stops, {} zones", stops, zones)
    }
}

/// Parses an outcome as stops then zones, e.g. `3 1` or `7 >2`.
impl std::str::FromStr for Outcome {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let error = || format!("expected stops then zones, e.g. \"3 1\" or \"7 >2\", got {s:?}");
        let [stops, zones] = s.split_whitespace().collect::<Vec<_>>()[..] else {
            return Err(error());
        };
        let stops = stops.parse().map_err(|_| error())?;
        let zones = match zones.strip_prefix('>') {
            Some("2") => ZoneOutcome::MoreThanTwo,
            Some(_) => return Err(error()),
            None => ZoneOutcome::from_zones(zones.parse().map_err(|_| error())?),
        };
        Ok(Self { stops, zones })
    }
}

impl Station {
    pub fn name(&self) -> &'static str {
        STATIONS[self.idx]
    }

    /// Finds a station by its name, ignoring case.
    pub fn find(name: &str) -> Option<Self> {
        Self::all().find(|station| station.name().eq_ignore_ascii_case(name.trim()))
    }

    pub fn get_outcome(&self, other: &Self) -> Outcome {
        let [stops, zones] = DISTANCES[self.idx][other.idx];
        Outcome {
            stops,
            zones: ZoneOutcome::from_zones(zones),
        }
    }

    pub fn all() -> impl Iterator<Item = Self> {
        (0..STATIONS.len()).map(|idx| Self { idx })
    }
}

const WORDS: usize = STATIONS.len().div_ceil(64);

/// A set of stations stored as a bitset, cheap to copy, compare and hash.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct StationSet([u64; WORDS]);

impl StationSet {
    pub fn all() -> Self {
        Station::all().collect()
    }

    pub fn insert(&mut self, station: Station) {
        self.0[station.idx / 64] |= 1 << (station.idx % 64);
    }

    pub fn remove(&mut self, station: Station) {
        self.0[station.idx / 64] &= !(1 << (station.idx % 64));
    }

    pub fn contains(&self, station: Station) -> bool {
        self.0[station.idx / 64] & (1 << (station.idx % 64)) != 0
    }

    pub fn len(&self) -> usize {
        self.0.iter().map(|word| word.count_ones() as usize).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.0.iter().all(|&word| word == 0)
    }

    pub fn first(&self) -> Option<Station> {
        self.iter().next()
    }

    pub fn iter(&self) -> impl Iterator<Item = Station> + '_ {
        self.0.iter().enumerate().flat_map(|(i, &word)| {
            let mut word = word;
            std::iter::from_fn(move || {
                if word == 0 {
                    return None;
                }
                let bit = word.trailing_zeros() as usize;
                word &= word - 1;
                Some(Station { idx: i * 64 + bit })
            })
        })
    }
}

impl FromIterator<Station> for StationSet {
    fn from_iter<I: IntoIterator<Item = Station>>(iter: I) -> Self {
        let mut set = Self::default();
        for station in iter {
            set.insert(station);
        }
        set
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distances_are_consistent() {
        for a in Station::all() {
            for b in Station::all() {
                let outcome = a.get_outcome(&b);
                assert!(outcome == b.get_outcome(&a), "{a} and {b} not symmetric");
                assert_eq!(outcome.stops == 0, a == b, "{a} to {b} is {outcome}");
                if a == b {
                    assert_eq!(outcome.zones, ZoneOutcome::Correct);
                }
            }
        }
    }

    #[test]
    fn station_names_are_unique() {
        for station in Station::all() {
            assert_eq!(Station::find(station.name()), Some(station));
        }
        assert_eq!(
            Station::find(" bank - monument "),
            Station::find("Bank - Monument")
        );
        assert_eq!(Station::find("Nowhere"), None);
    }

    #[test]
    fn station_set() {
        let all = StationSet::all();
        assert_eq!(all.len(), STATIONS.len());
        assert!(all.iter().eq(Station::all()));

        let mut set = StationSet::default();
        assert!(set.is_empty());
        let last = Station::all().last().unwrap();
        set.insert(last);
        assert!(set.contains(last) && set.len() == 1 && set.first() == Some(last));
        set.remove(last);
        assert!(set.is_empty());
    }

    #[test]
    fn parse_outcome() {
        let outcome = |stops, zones| Ok(Outcome { stops, zones });
        assert_eq!("3 1".parse(), outcome(3, ZoneOutcome::OneAway));
        assert_eq!(" 0  0 ".parse(), outcome(0, ZoneOutcome::Correct));
        assert_eq!("7 >2".parse(), outcome(7, ZoneOutcome::MoreThanTwo));
        assert_eq!("7 5".parse(), outcome(7, ZoneOutcome::MoreThanTwo));
        for bad in ["", "3", "3 1 1", "x 1", "3 >1", "-1 0"] {
            assert!(bad.parse::<Outcome>().is_err(), "{bad:?} parsed");
        }
    }
}
