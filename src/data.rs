const STATIONS: [&str; 269] = include!("../data/stations.json");
const DISTANCES: [[[u8; 2]; 269]; 269] = include!("../data/distances.json");

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Station {
    idx: usize,
}

impl std::fmt::Display for Station {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", STATIONS[self.idx])
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ZoneOutcome {
    Correct,
    OneAway,
    TwoAway,
    MoreThanTwo,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
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

impl Station {
    fn from(idx: usize) -> Self {
        if idx >= STATIONS.len() {
            panic!("Cannot handle station idx: {}", idx);
        }
        Self { idx }
    }

    pub fn get_outcome(self: &Self, other: &Self) -> Outcome {
        let distance = DISTANCES[self.idx][other.idx];
        let stops = distance[0];
        let zones = match distance[1] {
            0 => ZoneOutcome::Correct,
            1 => ZoneOutcome::OneAway,
            2 => ZoneOutcome::TwoAway,
            _ => ZoneOutcome::MoreThanTwo,
        };

        Outcome { stops, zones }
    }

    pub fn all_stations() -> Vec<Self> {
        (0..STATIONS.len()).map(Self::from).collect()
    }
}
