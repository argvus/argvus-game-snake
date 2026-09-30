//! Deterministic Snake rules and local ranking persistence.

use std::{
  collections::VecDeque,
  env, fs,
  io::{self, Write},
  path::{Path, PathBuf},
  time::{SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const BOARD_WIDTH: u16 = 24;
pub const BOARD_HEIGHT: u16 = 12;
pub const POINTS_PER_FOOD: u32 = 10;
pub const INITIAL_TICK_MILLIS: u64 = 180;
pub const FASTEST_TICK_MILLIS: u64 = 60;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
  pub x: u16,
  pub y: u16,
}

impl Position {
  pub const fn new(x: u16, y: u16) -> Self {
    Self { x, y }
  }

  fn step(self, direction: Direction) -> Option<Self> {
    match direction {
      Direction::Up => self.y.checked_sub(1).map(|y| Self::new(self.x, y)),
      Direction::Down => self.y.checked_add(1).map(|y| Self::new(self.x, y)),
      Direction::Left => self.x.checked_sub(1).map(|x| Self::new(x, self.y)),
      Direction::Right => self.x.checked_add(1).map(|x| Self::new(x, self.y)),
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
  Up,
  Down,
  Left,
  Right,
}

impl Direction {
  pub fn is_opposite(self, other: Self) -> bool {
    matches!(
      (self, other),
      (Self::Up, Self::Down)
        | (Self::Down, Self::Up)
        | (Self::Left, Self::Right)
        | (Self::Right, Self::Left)
    )
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepOutcome {
  Moved,
  Ate,
  Died,
  Won,
}

#[derive(Debug, Clone)]
pub struct SnakeGame {
  body: VecDeque<Position>,
  direction: Direction,
  food: Option<Position>,
  score: u32,
  foods_eaten: u32,
  random_state: u64,
}

impl SnakeGame {
  pub fn new() -> Self {
    let seed = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .map(|duration| duration.as_nanos() as u64)
      .unwrap_or(1);
    Self::with_seed(seed)
  }

  pub fn with_seed(seed: u64) -> Self {
    Self::with_seed_and_direction(seed, Direction::Right)
  }

  pub fn with_direction(direction: Direction) -> Self {
    let seed = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .map(|duration| duration.as_nanos() as u64)
      .unwrap_or(1);
    Self::with_seed_and_direction(seed, direction)
  }

  pub fn with_seed_and_direction(seed: u64, direction: Direction) -> Self {
    let center = Position::new(BOARD_WIDTH / 2, BOARD_HEIGHT / 2);
    let mut body = VecDeque::new();
    body.push_front(center);
    match direction {
      Direction::Up => {
        body.push_back(Position::new(center.x, center.y + 1));
        body.push_back(Position::new(center.x, center.y + 2));
      }
      Direction::Down => {
        body.push_back(Position::new(center.x, center.y - 1));
        body.push_back(Position::new(center.x, center.y - 2));
      }
      Direction::Left => {
        body.push_back(Position::new(center.x + 1, center.y));
        body.push_back(Position::new(center.x + 2, center.y));
      }
      Direction::Right => {
        body.push_back(Position::new(center.x - 1, center.y));
        body.push_back(Position::new(center.x - 2, center.y));
      }
    }

    let mut game = Self {
      body,
      direction,
      food: None,
      score: 0,
      foods_eaten: 0,
      random_state: seed,
    };
    game.spawn_food();
    game
  }

  pub fn turn(&mut self, direction: Direction) -> bool {
    if self.direction.is_opposite(direction) {
      return false;
    }
    self.direction = direction;
    true
  }

  pub fn advance(&mut self) -> StepOutcome {
    let Some(next_head) = self.head().step(self.direction) else {
      return StepOutcome::Died;
    };
    if next_head.x >= BOARD_WIDTH || next_head.y >= BOARD_HEIGHT {
      return StepOutcome::Died;
    }

    let is_eating = self.food == Some(next_head);
    let occupied_segments = if is_eating {
      self.body.len()
    } else {
      self.body.len().saturating_sub(1)
    };
    if self
      .body
      .iter()
      .take(occupied_segments)
      .any(|&part| part == next_head)
    {
      return StepOutcome::Died;
    }

    self.body.push_front(next_head);
    if !is_eating {
      self.body.pop_back();
      return StepOutcome::Moved;
    }

    self.score += POINTS_PER_FOOD;
    self.foods_eaten += 1;
    self.spawn_food();
    if self.food.is_none() {
      StepOutcome::Won
    } else {
      StepOutcome::Ate
    }
  }

  pub fn body(&self) -> &VecDeque<Position> {
    &self.body
  }

  pub fn head(&self) -> Position {
    *self.body.front().expect("SnakeGame always has a head")
  }

  pub fn food(&self) -> Option<Position> {
    self.food
  }

  pub fn direction(&self) -> Direction {
    self.direction
  }

  pub fn score(&self) -> u32 {
    self.score
  }

  pub fn foods_eaten(&self) -> u32 {
    self.foods_eaten
  }

  pub fn length(&self) -> usize {
    self.body.len()
  }

  pub fn speed_level(&self) -> u32 {
    self.foods_eaten / 5 + 1
  }

  pub fn tick_millis(&self) -> u64 {
    let reduction = (self.speed_level().saturating_sub(1) as u64 * 10)
      .min(INITIAL_TICK_MILLIS - FASTEST_TICK_MILLIS);
    INITIAL_TICK_MILLIS - reduction
  }

  fn next_random(&mut self) -> u64 {
    self.random_state = self
      .random_state
      .wrapping_mul(6364136223846793005)
      .wrapping_add(1442695040888963407);
    self.random_state
  }

  fn spawn_food(&mut self) {
    let mut free_positions = Vec::new();
    for y in 0..BOARD_HEIGHT {
      for x in 0..BOARD_WIDTH {
        let position = Position::new(x, y);
        if !self.body.contains(&position) {
          free_positions.push(position);
        }
      }
    }

    self.food = if free_positions.is_empty() {
      None
    } else {
      let index = (self.next_random() as usize) % free_positions.len();
      Some(free_positions[index])
    };
  }
}

impl Default for SnakeGame {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ScoreEntry {
  pub player: String,
  pub score: u32,
  pub foods_eaten: u32,
  pub length: u16,
  pub duration_seconds: u64,
  pub played_at_epoch_seconds: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct RankingDocument {
  schema_version: u32,
  entries: Vec<ScoreEntry>,
}

#[derive(Debug, Error)]
pub enum RankingError {
  #[error("could not determine the ARGVUS configuration directory")]
  MissingConfigHome,
  #[error("could not access ranking file {path}: {source}")]
  Io { path: PathBuf, source: io::Error },
  #[error("could not parse ranking file {path}: {source}")]
  Json {
    path: PathBuf,
    source: serde_json::Error,
  },
  #[error("unsupported ranking schema version {version}")]
  UnsupportedSchema { version: u32 },
}

#[derive(Debug, Clone)]
pub struct RankingStore {
  directory: PathBuf,
}

impl RankingStore {
  pub fn from_environment() -> Result<Self, RankingError> {
    let config_home = env::var_os("ARGVUS_CONFIG_HOME")
      .or_else(|| env::var_os("XDG_CONFIG_HOME"))
      .map(PathBuf::from)
      .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".config")))
      .ok_or(RankingError::MissingConfigHome)?;
    Ok(Self::new(config_home.join("argvus/games/snake")))
  }

  pub fn new(directory: impl Into<PathBuf>) -> Self {
    Self {
      directory: directory.into(),
    }
  }

  pub fn directory(&self) -> &Path {
    &self.directory
  }

  pub fn ranking_path(&self) -> PathBuf {
    self.directory.join("ranking.json")
  }

  pub fn load(&self) -> Result<Vec<ScoreEntry>, RankingError> {
    let path = self.ranking_path();
    if !path.exists() {
      return Ok(Vec::new());
    }
    let contents = fs::read_to_string(&path).map_err(|source| RankingError::Io {
      path: path.clone(),
      source,
    })?;
    let document: RankingDocument =
      serde_json::from_str(&contents).map_err(|source| RankingError::Json {
        path: path.clone(),
        source,
      })?;
    if document.schema_version != 1 {
      return Err(RankingError::UnsupportedSchema {
        version: document.schema_version,
      });
    }
    Ok(sorted_entries(document.entries))
  }

  pub fn record(&self, entry: ScoreEntry) -> Result<Vec<ScoreEntry>, RankingError> {
    let mut entries = self.load()?;
    entries.push(entry);
    let entries = sorted_entries(entries);
    fs::create_dir_all(&self.directory).map_err(|source| RankingError::Io {
      path: self.directory.clone(),
      source,
    })?;

    let path = self.ranking_path();
    let temporary_path = self
      .directory
      .join(format!("ranking.json.tmp.{}", std::process::id()));
    let document = RankingDocument {
      schema_version: 1,
      entries: entries.clone(),
    };
    let serialized = serde_json::to_vec_pretty(&document).expect("ranking is serializable");
    let write_result = (|| {
      let mut file = fs::File::create(&temporary_path).map_err(|source| RankingError::Io {
        path: temporary_path.clone(),
        source,
      })?;
      file
        .write_all(&serialized)
        .and_then(|_| file.sync_all())
        .map_err(|source| RankingError::Io {
          path: temporary_path.clone(),
          source,
        })?;
      fs::rename(&temporary_path, &path).map_err(|source| RankingError::Io {
        path: path.clone(),
        source,
      })
    })();
    if write_result.is_err() {
      let _ = fs::remove_file(&temporary_path);
    }
    write_result.map(|_| entries)
  }
}

fn sorted_entries(mut entries: Vec<ScoreEntry>) -> Vec<ScoreEntry> {
  entries.sort_by(|left, right| {
    right
      .score
      .cmp(&left.score)
      .then(
        left
          .played_at_epoch_seconds
          .cmp(&right.played_at_epoch_seconds),
      )
      .then(left.player.cmp(&right.player))
  });
  entries.truncate(10);
  entries
}

#[cfg(test)]
mod tests {
  use super::*;

  fn score_entry(score: u32, played_at_epoch_seconds: u64) -> ScoreEntry {
    ScoreEntry {
      player: "tester".into(),
      score,
      foods_eaten: score / POINTS_PER_FOOD,
      length: 3 + (score / POINTS_PER_FOOD) as u16,
      duration_seconds: 10,
      played_at_epoch_seconds,
    }
  }

  #[test]
  fn game_starts_with_a_three_segment_snake_and_food() {
    let game = SnakeGame::with_seed(1);
    assert_eq!(game.length(), 3);
    assert_eq!(game.score(), 0);
    assert!(game.food().is_some());
  }

  #[test]
  fn opposite_direction_is_rejected() {
    let mut game = SnakeGame::with_seed(1);
    assert!(!game.turn(Direction::Left));
    assert_eq!(game.direction(), Direction::Right);
  }

  #[test]
  fn initial_direction_orients_the_body_behind_the_head() {
    let mut game = SnakeGame::with_seed_and_direction(1, Direction::Left);
    assert_eq!(game.direction(), Direction::Left);
    assert_eq!(game.advance(), StepOutcome::Moved);
    assert_eq!(
      game.head(),
      Position::new(BOARD_WIDTH / 2 - 1, BOARD_HEIGHT / 2)
    );
  }

  #[test]
  fn movement_keeps_length_until_food_is_eaten() {
    let mut game = SnakeGame::with_seed(1);
    assert_eq!(game.advance(), StepOutcome::Moved);
    assert_eq!(game.length(), 3);
  }

  #[test]
  fn speed_increases_every_five_foods() {
    let mut game = SnakeGame::with_seed(1);
    assert_eq!(game.tick_millis(), INITIAL_TICK_MILLIS);
    game.foods_eaten = 5;
    assert_eq!(game.speed_level(), 2);
    assert_eq!(game.tick_millis(), INITIAL_TICK_MILLIS - 10);
    game.foods_eaten = 100;
    assert_eq!(game.tick_millis(), FASTEST_TICK_MILLIS);
  }

  #[test]
  fn ranking_is_sorted_and_limited_to_ten_entries() {
    let unique = format!(
      "argvus-game-snake-ranking-{}-{}",
      std::process::id(),
      SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos()
    );
    let directory = env::temp_dir().join(unique);
    let store = RankingStore::new(&directory);
    for score in 0..12 {
      store.record(score_entry(score, score as u64)).unwrap();
    }
    let entries = store.load().unwrap();
    assert_eq!(entries.len(), 10);
    assert_eq!(entries.first().unwrap().score, 11);
    assert_eq!(entries.last().unwrap().score, 2);
    fs::remove_dir_all(directory).unwrap();
  }

  #[test]
  fn missing_ranking_starts_empty() {
    let directory = env::temp_dir().join(format!("argvus-game-snake-empty-{}", std::process::id()));
    let store = RankingStore::new(&directory);
    assert!(store.load().unwrap().is_empty());
  }
}
