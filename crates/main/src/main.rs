use std::{
  env,
  error::Error,
  time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use argvus_game_snake_core::{
  BOARD_HEIGHT, BOARD_WIDTH, Direction, RankingStore, ScoreEntry, SnakeGame, StepOutcome,
};
use argvus_i18n::I18n;
use argvus_theme::Theme;
use argvus_tui::{
  chrome::{self, Header, MessageKind},
  terminal::{TerminalGuard, install_panic_hook},
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
  Frame,
  layout::{Constraint, Layout, Rect},
  style::{Modifier, Style},
  text::{Line, Span},
  widgets::{Block, Borders, Clear, Paragraph},
};

const MIN_TERMINAL_WIDTH: u16 = 60;
const MIN_TERMINAL_HEIGHT: u16 = 18;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Screen {
  Ready,
  Running,
  Paused,
  GameOver,
}

struct App {
  i18n: I18n,
  game: SnakeGame,
  ranking_store: RankingStore,
  ranking: Vec<ScoreEntry>,
  player: String,
  screen: Screen,
  started_at: Instant,
  last_tick: Instant,
  final_duration_seconds: u64,
  status_message: Option<String>,
  status_kind: MessageKind,
  should_quit: bool,
}

impl App {
  fn new(i18n: I18n) -> Result<Self, Box<dyn Error>> {
    let ranking_store = RankingStore::from_environment()?;
    let (ranking, status_message, status_kind) = match ranking_store.load() {
      Ok(ranking) => (ranking, None, MessageKind::Info),
      Err(error) => (
        Vec::new(),
        Some(format!("{}: {error}", i18n.tr("ranking_load_failed"))),
        MessageKind::Error,
      ),
    };
    let now = Instant::now();
    Ok(Self {
      i18n,
      game: SnakeGame::new(),
      ranking_store,
      ranking,
      player: env::var("USER").unwrap_or_else(|_| "player".to_owned()),
      screen: Screen::Ready,
      started_at: now,
      last_tick: now,
      final_duration_seconds: 0,
      status_message,
      status_kind,
      should_quit: false,
    })
  }

  fn run(&mut self, terminal: &mut TerminalGuard) -> Result<(), Box<dyn Error>> {
    let theme = Theme::load();
    while !self.should_quit {
      terminal
        .terminal_mut()
        .draw(|frame| self.render(frame, &theme))?;
      self.process_events()?;
      self.advance_if_due();
    }
    Ok(())
  }

  fn process_events(&mut self) -> Result<(), Box<dyn Error>> {
    let timeout = if self.screen == Screen::Running {
      Duration::from_millis(self.game.tick_millis().min(30))
    } else {
      Duration::from_millis(100)
    };
    if event::poll(timeout)?
      && let Event::Key(key_event) = event::read()?
    {
      self.handle_key(key_event);
    }
    Ok(())
  }

  fn handle_key(&mut self, key_event: KeyEvent) {
    if key_event.kind != KeyEventKind::Press {
      return;
    }
    if matches!(key_event.code, KeyCode::Esc) {
      self.should_quit = true;
      return;
    }

    match self.screen {
      Screen::Ready => match key_event.code {
        KeyCode::Enter => self.start_game(Direction::Right),
        KeyCode::Char('q') => self.should_quit = true,
        code => {
          if let Some(direction) = direction_from_key(code) {
            self.start_game(direction);
          }
        }
      },
      Screen::Running => match key_event.code {
        KeyCode::Char('q') => self.should_quit = true,
        KeyCode::Char('p') | KeyCode::Char(' ') => self.screen = Screen::Paused,
        code => {
          if let Some(direction) = direction_from_key(code) {
            self.game.turn(direction);
          }
        }
      },
      Screen::Paused => match key_event.code {
        KeyCode::Char('q') => self.should_quit = true,
        KeyCode::Char('p') | KeyCode::Char(' ') => {
          self.screen = Screen::Running;
          self.last_tick = Instant::now();
        }
        _ => {}
      },
      Screen::GameOver => match key_event.code {
        KeyCode::Char('q') => self.should_quit = true,
        KeyCode::Char('r') => self.start_game(Direction::Right),
        _ => {}
      },
    }
  }

  fn start_game(&mut self, direction: Direction) {
    self.game = SnakeGame::with_direction(direction);
    let now = Instant::now();
    self.started_at = now;
    self.last_tick = now;
    self.final_duration_seconds = 0;
    self.status_message = None;
    self.status_kind = MessageKind::Info;
    self.screen = Screen::Running;
  }

  fn advance_if_due(&mut self) {
    if self.screen != Screen::Running
      || self.last_tick.elapsed() < Duration::from_millis(self.game.tick_millis())
    {
      return;
    }
    self.last_tick = Instant::now();
    match self.game.advance() {
      StepOutcome::Moved | StepOutcome::Ate => {}
      StepOutcome::Died | StepOutcome::Won => self.finish_game(),
    }
  }

  fn finish_game(&mut self) {
    self.final_duration_seconds = self.started_at.elapsed().as_secs();
    let previous_best = self.ranking.first().map(|entry| entry.score).unwrap_or(0);
    if self.game.score() > 0 {
      let played_at_epoch_seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0);
      let entry = ScoreEntry {
        player: self.player.clone(),
        score: self.game.score(),
        foods_eaten: self.game.foods_eaten(),
        length: self.game.length() as u16,
        duration_seconds: self.final_duration_seconds,
        played_at_epoch_seconds,
      };
      match self.ranking_store.record(entry) {
        Ok(ranking) => {
          self.ranking = ranking;
          self.status_message = Some(if self.game.score() > previous_best {
            self.i18n.tr("new_record")
          } else {
            self.i18n.tr("score_saved")
          });
          self.status_kind = MessageKind::Success;
        }
        Err(error) => {
          self.status_message = Some(format!("{}: {error}", self.i18n.tr("ranking_save_failed")));
          self.status_kind = MessageKind::Error;
        }
      }
    }
    self.screen = Screen::GameOver;
  }

  fn render(&self, frame: &mut Frame, theme: &Theme) {
    let area = frame.area();
    if area.width < MIN_TERMINAL_WIDTH || area.height < MIN_TERMINAL_HEIGHT {
      chrome::draw_too_small(
        frame,
        area,
        theme,
        &self.i18n.tr("terminal_too_small"),
        &format!(
          "{}: {}x{}",
          self.i18n.tr("minimum"),
          MIN_TERMINAL_WIDTH,
          MIN_TERMINAL_HEIGHT
        ),
        &format!(
          "{}: {}x{}",
          self.i18n.tr("current"),
          area.width,
          area.height
        ),
      );
      return;
    }

    let title = self.i18n.tr("title");
    let header_area = Layout::vertical([
      Constraint::Length(1),
      Constraint::Min(1),
      Constraint::Length(2),
    ])
    .split(area);
    chrome::draw_header(
      frame,
      header_area[0],
      theme,
      Header {
        title: &title,
        version: None,
        version_label: "",
      },
    );

    match self.screen {
      Screen::Ready => self.render_ready(frame, header_area[1], theme),
      Screen::Running | Screen::Paused => self.render_board(frame, header_area[1], theme),
      Screen::GameOver => self.render_game_over(frame, header_area[1], theme),
    }

    let hints = self.footer_hints();
    let status = self
      .status_message
      .as_deref()
      .map(|message| (message, self.status_kind));
    chrome::draw_footer(frame, header_area[2], theme, status, &hints);
  }

  fn render_ready(&self, frame: &mut Frame, area: Rect, theme: &Theme) {
    let mut lines = vec![
      Line::from(Span::styled(
        self.i18n.tr("ready").to_uppercase(),
        Style::default()
          .fg(theme.accent)
          .add_modifier(Modifier::BOLD),
      )),
      Line::from(""),
      Line::from(self.i18n.tr("ready_description")),
      Line::from(""),
      Line::from(self.i18n.tr("ranking")),
    ];
    lines.extend(self.ranking_lines(theme));
    lines.push(Line::from(""));
    lines.push(Line::from(self.i18n.tr("press_start")));
    let popup = chrome::centered(
      area,
      area.width.saturating_sub(4).min(64),
      area.height.min(18),
    );
    frame.render_widget(Clear, popup);
    frame.render_widget(
      Paragraph::new(lines)
        .alignment(ratatui::layout::Alignment::Center)
        .block(
          Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.border_active))
            .title(format!(" {} ", self.i18n.tr("game_menu"))),
        )
        .style(Style::default().fg(theme.foreground).bg(theme.background)),
      popup,
    );
  }

  fn render_board(&self, frame: &mut Frame, area: Rect, theme: &Theme) {
    let rows = Layout::vertical([Constraint::Length(1), Constraint::Min(1)]).split(area);
    let hud = format!(
      "{}: {}   {}: {}   {}: {}   {}: {}ms",
      self.i18n.tr("score"),
      self.game.score(),
      self.i18n.tr("length"),
      self.game.length(),
      self.i18n.tr("level"),
      self.game.speed_level(),
      self.i18n.tr("speed"),
      self.game.tick_millis(),
    );
    frame.render_widget(
      Paragraph::new(hud)
        .alignment(ratatui::layout::Alignment::Center)
        .style(Style::default().fg(theme.foreground)),
      rows[0],
    );

    let board_width = BOARD_WIDTH * 2 + 2;
    let board_height = BOARD_HEIGHT + 2;
    let board_area = chrome::centered(rows[1], board_width, board_height);
    let block = Block::default()
      .borders(Borders::ALL)
      .border_style(Style::default().fg(theme.border_active))
      .title(format!(" {} ", self.i18n.tr("board")))
      .style(Style::default().bg(theme.background));
    let inner = block.inner(board_area);
    frame.render_widget(block, board_area);

    let lines = (0..BOARD_HEIGHT)
      .map(|y| {
        let spans = (0..BOARD_WIDTH).map(|x| {
          let position = argvus_game_snake_core::Position::new(x, y);
          let glyph = if self.game.food() == Some(position) {
            "▒▒"
          } else if self.game.head() == position {
            "▓▓"
          } else if self.game.body().contains(&position) {
            "██"
          } else {
            "  "
          };
          let color = if self.game.food() == Some(position) {
            theme.warning
          } else if self.game.head() == position {
            theme.selected_background
          } else if self.game.body().contains(&position) {
            theme.accent
          } else {
            theme.background
          };
          Span::styled(glyph, Style::default().fg(color).bg(theme.background))
        });
        Line::from(spans.collect::<Vec<_>>())
      })
      .collect::<Vec<_>>();
    frame.render_widget(Paragraph::new(lines), inner);

    if self.screen == Screen::Paused {
      let overlay = chrome::centered(board_area, 30, 5);
      frame.render_widget(Clear, overlay);
      frame.render_widget(
        Paragraph::new(self.i18n.tr("paused"))
          .alignment(ratatui::layout::Alignment::Center)
          .block(
            Block::default()
              .borders(Borders::ALL)
              .border_style(Style::default().fg(theme.warning)),
          )
          .style(Style::default().fg(theme.foreground).bg(theme.background)),
        overlay,
      );
    }
  }

  fn render_game_over(&self, frame: &mut Frame, area: Rect, theme: &Theme) {
    let mut lines = vec![
      Line::from(Span::styled(
        self.i18n.tr("game_over").to_uppercase(),
        Style::default()
          .fg(theme.error)
          .add_modifier(Modifier::BOLD),
      )),
      Line::from(""),
      Line::from(format!("{}: {}", self.i18n.tr("score"), self.game.score())),
      Line::from(format!(
        "{}: {}",
        self.i18n.tr("length"),
        self.game.length()
      )),
      Line::from(format!(
        "{}: {}s",
        self.i18n.tr("duration"),
        self.final_duration_seconds
      )),
      Line::from(""),
      Line::from(self.i18n.tr("ranking")),
    ];
    lines.extend(self.ranking_lines(theme));
    lines.push(Line::from(""));
    lines.push(Line::from(self.i18n.tr("restart_or_quit")));
    let popup = chrome::centered(
      area,
      area.width.saturating_sub(4).min(64),
      area.height.min(18),
    );
    frame.render_widget(Clear, popup);
    frame.render_widget(
      Paragraph::new(lines)
        .alignment(ratatui::layout::Alignment::Center)
        .block(
          Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.border_active))
            .title(format!(" {} ", self.i18n.tr("game_over"))),
        )
        .style(Style::default().fg(theme.foreground).bg(theme.background)),
      popup,
    );
  }

  fn ranking_lines(&self, theme: &Theme) -> Vec<Line<'static>> {
    if self.ranking.is_empty() {
      return vec![Line::from(Span::styled(
        self.i18n.tr("no_scores"),
        Style::default().fg(theme.muted),
      ))];
    }
    self
      .ranking
      .iter()
      .enumerate()
      .map(|(index, entry)| {
        Line::from(format!(
          "{:>2}. {:<16} {:>5} {}",
          index + 1,
          truncate_player(&entry.player),
          entry.score,
          self.i18n.tr("points_short")
        ))
      })
      .collect()
  }

  fn footer_hints(&self) -> String {
    match self.screen {
      Screen::Ready => format!(
        "{}  {}  {}",
        self.i18n.tr("start_hint"),
        self.i18n.tr("movement_hint"),
        self.i18n.tr("quit_hint")
      ),
      Screen::Running => format!(
        "{}  {}  {}",
        self.i18n.tr("movement_hint"),
        self.i18n.tr("pause_hint"),
        self.i18n.tr("quit_hint")
      ),
      Screen::Paused => format!(
        "{}  {}",
        self.i18n.tr("resume_hint"),
        self.i18n.tr("quit_hint")
      ),
      Screen::GameOver => format!(
        "{}  {}",
        self.i18n.tr("restart_hint"),
        self.i18n.tr("quit_hint")
      ),
    }
  }
}

fn direction_from_key(key_code: KeyCode) -> Option<Direction> {
  match key_code {
    KeyCode::Char('h') => Some(Direction::Left),
    KeyCode::Char('j') => Some(Direction::Down),
    KeyCode::Char('k') => Some(Direction::Up),
    KeyCode::Char('l') => Some(Direction::Right),
    _ => None,
  }
}

fn truncate_player(player: &str) -> String {
  player.chars().take(16).collect()
}

fn main() -> Result<(), Box<dyn Error>> {
  install_panic_hook();
  let i18n = I18n::new("game-snake")?;
  let mut app = App::new(i18n)?;
  let mut terminal = TerminalGuard::new()?;
  app.run(&mut terminal)
}
