use crate::window::GameWindow;
const MAX_ENERGY: i16 = i16::MAX;

const MIN_ENERGY: i16 = 1;

const DefaultWindowsSavePath: &str = r"%homepath%\AppData\LocalLow";
const DefaultLinuxSavePath: &str = r"%homepath%\AppData\LocalLow";

const EasyHighestGameValue: i8 = 4;
const EasyLowestGameValue: i8 = 1;
const EasySpecialValueIndex: i8 = 6;


pub enum LevelDifficulty {
    Easy,
    Normal,
    Hard,
}

pub struct GameLevel {
    window: GameWindow,
    energy: i16,
}

impl GameLevel {
    pub fn new(difficulty: LevelDifficulty) -> Self {

    }
}