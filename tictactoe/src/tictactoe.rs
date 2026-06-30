use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

/// A player in a Tic-Tac-Toe game.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Player {
    /// The X player.
    X,
    /// The O player.
    O,
}

impl Player {
    fn other(self) -> Self {
        match self {
            Self::X => Self::O,
            Self::O => Self::X,
        }
    }
}

impl Display for Player {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::X => write!(formatter, "X"),
            Self::O => write!(formatter, "O"),
        }
    }
}

/// The final result of a completed Tic-Tac-Toe game.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GameResult {
    /// One player has three fields in a row, column, or diagonal.
    Win(Player),
    /// All fields are occupied without a winner.
    Draw,
}

impl Display for GameResult {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Win(player) => write!(formatter, "Player {player} wins."),
            Self::Draw => write!(formatter, "The game ends in a draw."),
        }
    }
}

/// A rule violation that prevents a turn from being played.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IllegalTurn {
    /// The selected field already contains a player mark.
    FieldAlreadyOccupied,
    /// The game has already ended with a win or draw.
    GameAlreadyFinished,
}

impl Display for IllegalTurn {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::FieldAlreadyOccupied => {
                write!(formatter, "The selected field is already occupied.")
            }
            Self::GameAlreadyFinished => write!(formatter, "The game is already finished."),
        }
    }
}

/// A parse error for a board position.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ParsePositionError;

impl Display for ParsePositionError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "Enter a position from A1 to C3 using an uppercase column letter."
        )
    }
}

/// A zero-based position on the 3x3 board.
///
/// Input coordinates use uppercase columns `A` through `C` and rows `1` through `3`.
/// For example, `A1` maps to `Position { x: 0, y: 0 }`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Position {
    /// The zero-based column index.
    pub x: usize,
    /// The zero-based row index.
    pub y: usize,
}

impl FromStr for Position {
    type Err = ParsePositionError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let bytes = input.as_bytes();

        if bytes.len() != 2 {
            return Err(ParsePositionError);
        }

        let x = match bytes[0] {
            b'A'..=b'C' => usize::from(bytes[0] - b'A'),
            _ => return Err(ParsePositionError),
        };
        let y = match bytes[1] {
            b'1'..=b'3' => usize::from(bytes[1] - b'1'),
            _ => return Err(ParsePositionError),
        };

        Ok(Self { x, y })
    }
}

/// The state and rules engine for a Tic-Tac-Toe game.
pub struct GameState {
    board: [[Option<Player>; 3]; 3],
    next_player: Player,
    result: Option<GameResult>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            board: [[None; 3]; 3],
            next_player: Player::X,
            result: None,
        }
    }
}

impl GameState {
    /// Returns the player whose turn is next, or `None` once the game is over.
    pub fn get_next_player(&self) -> Option<Player> {
        self.result.is_none().then_some(self.next_player)
    }

    /// Plays one turn at the given position.
    ///
    /// On success, the method returns `Ok(None)` while the game continues and
    /// `Ok(Some(result))` when the turn ends the game. Illegal turns leave the
    /// board and next player unchanged.
    pub fn make_turn(&mut self, position: Position) -> Result<Option<GameResult>, IllegalTurn> {
        if self.result.is_some() {
            return Err(IllegalTurn::GameAlreadyFinished);
        }

        if self.board[position.y][position.x].is_some() {
            return Err(IllegalTurn::FieldAlreadyOccupied);
        }

        let player = self.next_player;
        self.board[position.y][position.x] = Some(player);

        if self.has_won(player) {
            let result = GameResult::Win(player);
            self.result = Some(result);
            return Ok(Some(result));
        }

        if self.is_draw() {
            let result = GameResult::Draw;
            self.result = Some(result);
            return Ok(Some(result));
        }

        self.next_player = self.next_player.other();
        Ok(None)
    }

    fn has_won(&self, player: Player) -> bool {
        let row_win = self
            .board
            .iter()
            .any(|row| row.iter().all(|field| *field == Some(player)));
        let column_win = (0..3).any(|x| (0..3).all(|y| self.board[y][x] == Some(player)));
        let diagonal_win = (0..3).all(|index| self.board[index][index] == Some(player))
            || (0..3).all(|index| self.board[index][2 - index] == Some(player));

        row_win || column_win || diagonal_win
    }

    fn is_draw(&self) -> bool {
        self.board.iter().flatten().all(Option::is_some)
    }
}

impl Display for GameState {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        writeln!(formatter, "  A B C")?;

        for (row_index, row) in self.board.iter().enumerate() {
            write!(formatter, "{}", row_index + 1)?;

            for field in row {
                match field {
                    Some(player) => write!(formatter, " {player}")?,
                    None => write!(formatter, " .")?,
                }
            }

            if row_index < 2 {
                writeln!(formatter)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn position(input: &str) -> Position {
        input.parse().expect("test positions must be valid")
    }

    #[test]
    fn parses_valid_positions() {
        assert_eq!("A1".parse::<Position>(), Ok(Position { x: 0, y: 0 }));
        assert_eq!("B2".parse::<Position>(), Ok(Position { x: 1, y: 1 }));
        assert_eq!("C3".parse::<Position>(), Ok(Position { x: 2, y: 2 }));
    }

    #[test]
    fn rejects_invalid_positions() {
        for input in ["a1", "A0", "A4", "D1", "AA", "A", "A11", ""] {
            assert_eq!(input.parse::<Position>(), Err(ParsePositionError));
        }
    }

    #[test]
    fn default_state_has_empty_board_and_x_starts() {
        let game = GameState::default();

        assert_eq!(game.board, [[None; 3]; 3]);
        assert_eq!(game.get_next_player(), Some(Player::X));
    }

    #[test]
    fn successful_turn_marks_field_and_switches_player() {
        let mut game = GameState::default();

        assert_eq!(game.make_turn(position("B2")), Ok(None));

        assert_eq!(game.board[1][1], Some(Player::X));
        assert_eq!(game.get_next_player(), Some(Player::O));
    }

    #[test]
    fn occupied_field_is_rejected_without_switching_player() {
        let mut game = GameState::default();

        assert_eq!(game.make_turn(position("A1")), Ok(None));
        assert_eq!(
            game.make_turn(position("A1")),
            Err(IllegalTurn::FieldAlreadyOccupied)
        );

        assert_eq!(game.board[0][0], Some(Player::X));
        assert_eq!(game.get_next_player(), Some(Player::O));
    }

    #[test]
    fn detects_row_win() {
        let mut game = GameState::default();

        assert_eq!(game.make_turn(position("A1")), Ok(None));
        assert_eq!(game.make_turn(position("A2")), Ok(None));
        assert_eq!(game.make_turn(position("B1")), Ok(None));
        assert_eq!(game.make_turn(position("B2")), Ok(None));

        assert_eq!(
            game.make_turn(position("C1")),
            Ok(Some(GameResult::Win(Player::X)))
        );
        assert_eq!(game.get_next_player(), None);
    }

    #[test]
    fn detects_column_win() {
        let mut game = GameState::default();

        assert_eq!(game.make_turn(position("A1")), Ok(None));
        assert_eq!(game.make_turn(position("B1")), Ok(None));
        assert_eq!(game.make_turn(position("A2")), Ok(None));
        assert_eq!(game.make_turn(position("B2")), Ok(None));

        assert_eq!(
            game.make_turn(position("A3")),
            Ok(Some(GameResult::Win(Player::X)))
        );
    }

    #[test]
    fn detects_diagonal_win() {
        let mut game = GameState::default();

        assert_eq!(game.make_turn(position("A1")), Ok(None));
        assert_eq!(game.make_turn(position("B1")), Ok(None));
        assert_eq!(game.make_turn(position("B2")), Ok(None));
        assert_eq!(game.make_turn(position("C1")), Ok(None));

        assert_eq!(
            game.make_turn(position("C3")),
            Ok(Some(GameResult::Win(Player::X)))
        );
    }

    #[test]
    fn detects_draw() {
        let mut game = GameState::default();

        for input in ["A1", "B1", "C1", "C2", "A2", "A3", "B2", "C3"] {
            assert_eq!(game.make_turn(position(input)), Ok(None));
        }

        assert_eq!(game.make_turn(position("B3")), Ok(Some(GameResult::Draw)));
        assert_eq!(game.get_next_player(), None);
    }

    #[test]
    fn finished_game_rejects_additional_turns() {
        let mut game = GameState::default();

        assert_eq!(game.make_turn(position("A1")), Ok(None));
        assert_eq!(game.make_turn(position("A2")), Ok(None));
        assert_eq!(game.make_turn(position("B1")), Ok(None));
        assert_eq!(game.make_turn(position("B2")), Ok(None));
        assert_eq!(
            game.make_turn(position("C1")),
            Ok(Some(GameResult::Win(Player::X)))
        );

        assert_eq!(
            game.make_turn(position("C3")),
            Err(IllegalTurn::GameAlreadyFinished)
        );
    }

    #[test]
    fn display_renders_board_with_coordinates() {
        let mut game = GameState::default();

        assert_eq!(game.make_turn(position("A1")), Ok(None));
        assert_eq!(game.make_turn(position("B2")), Ok(None));

        assert_eq!(format!("{game}"), "  A B C\n1 X . .\n2 . O .\n3 . . .");
    }
}
