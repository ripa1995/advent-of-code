use std::str::FromStr;

enum Options {
    R,
    P,
    S,
}

impl FromStr for Options {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::R),
            "B" | "Y" => Ok(Self::P),
            "C" | "Z" => Ok(Self::S),
            _ => Err(String::from("Invalid input.")),
        }
    }
}

impl Options {
    pub fn value(&self) -> u8 {
        match self {
            Options::R => 1,
            Options::P => 2,
            Options::S => 3,
        }
    }

    pub fn beat(&self, other: &Self) -> MatchResult {
        match self {
            Options::R => {
                match other {
                    Options::R => MatchResult::D,
                    Options::P => MatchResult::L,
                    Options::S => MatchResult::W,
                }
            },
            Options::P => {
                match other {
                    Options::R => MatchResult::W,
                    Options::P => MatchResult::D,
                    Options::S => MatchResult::L,
                }
            },
            Options::S => {
                match other {
                    Options::R => MatchResult::L,
                    Options::P => MatchResult::W,
                    Options::S => MatchResult::D,
                }
            },
        }
    }

    pub fn counterparty_required_for(&self, other: &MatchResult) -> Self {
        match self {
            Options::R => {
                match other {
                    MatchResult::W => Options::P,
                    MatchResult::L => Options::S,
                    MatchResult::D => Options::R,
                }
            },
            Options::P => {
                match other {
                    MatchResult::W => Options::S,
                    MatchResult::L => Options::R,
                    MatchResult::D => Options::P,
                }
            },
            Options::S => {
                match other {
                    MatchResult::W => Options::R,
                    MatchResult::L => Options::P,
                    MatchResult::D => Options::S,
                }
            },
        }
    }
}

pub enum MatchResult {
    W,
    L,
    D
}

impl FromStr for MatchResult {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::L),
            "Y" => Ok(Self::D),
            "Z" => Ok(Self::W),
            _ => Err(String::from("Invalid input.")),
        }
    }
}

impl MatchResult {
    pub fn value(&self) -> u8 {
        match self {
            MatchResult::W => 6,
            MatchResult::L => 0,
            MatchResult::D => 3,
        }
    }
}

pub struct RPSMatch {
    enemy: Options,
    player: Options,
}

impl FromStr for RPSMatch {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(' ').collect();
        let enemy = if let Some(enemy) = split.first() {
            Options::from_str(enemy)
        } else {
            return Err(String::from("Invalid input."));
        };

        let player = if let Some(s) = split.get(1) {
            Options::from_str(s)
        } else {
            return Err(String::from("Invalid input."));
        };

        
        if let Ok(player) = player {
            if let Ok(enemy) = enemy {
                return Ok(Self {
                    enemy,
                    player
                })
            }
        } 
        Err(String::from("Invalid input."))
    }
}

impl RPSMatch {
    pub fn eval_match(&self) -> u8 {
        self.player.beat(&self.enemy).value() + self.player.value()
    }
}

pub struct RPSMatchV2 {
    enemy: Options,
    result: MatchResult,
}

impl FromStr for RPSMatchV2 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(' ').collect();
        let enemy = if let Some(enemy) = split.first() {
            Options::from_str(enemy)
        } else {
            return Err(String::from("Invalid input."));
        };

        let res = if let Some(s) = split.get(1) {
            MatchResult::from_str(s)
        } else {
            return Err(String::from("Invalid input."));
        };

        if let Ok(result) = res {
            if let Ok(enemy) = enemy {
                return Ok(Self {
                    enemy,
                    result
                })
            }
        } 
        Err(String::from("Invalid input."))
    }
}

impl RPSMatchV2 {
    pub fn eval_match(&self) -> u8 {
        self.result.value() + self.enemy.counterparty_required_for(&self.result).value()
    }
}