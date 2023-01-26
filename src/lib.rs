const STATIONS: [&str; 269] = include!("../data/stations.json");
const DISTANCES: [[[u8; 2]; 269]; 269] = include!("../data/distances.json");

#[derive(Copy, Clone)]
pub struct Station {
    idx: usize,
}

#[derive(PartialEq, Eq, Hash)]
pub enum ZoneOutcome {
    Correct,
    OneAway,
    TwoAway,
    MoreThanTwo,
}

#[derive(PartialEq, Eq, Hash)]
pub struct Outcome {
    pub stops: u8,
    pub zones: ZoneOutcome,
}

impl Station {
    pub fn from(idx: usize) -> Self {
        if idx >= STATIONS.len() {
            panic!("Cannot handle station idx: {}", idx);
        }
        Self { idx }
    }

    pub fn get_name(self: &Self) -> String {
        STATIONS[self.idx].into()
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
}
