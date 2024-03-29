use sea_orm::DatabaseConnection;
use std::time::Duration;
use tui::widgets::TableState;

#[derive(Clone)]
pub enum AppState {
    Init,
    Initialized {
        duration: Duration,
        counter_sleep: u32,
        counter_tick: u64,
        db: DatabaseConnection,
        books: Vec<i32>,
        books_tablestate: TableState,
    },
}

impl AppState {
    pub fn initialized(db: DatabaseConnection) -> Self {
        let duration = Duration::from_secs(1);
        let counter_sleep = 0;
        let counter_tick = 0;

        Self::Initialized {
            duration,
            counter_sleep,
            counter_tick,
            db: db,
            books: Vec::new(),
            books_tablestate: TableState::default(),
        }
    }

    pub fn is_initialized(&self) -> bool {
        matches!(self, &Self::Initialized { .. })
    }

    pub fn incr_sleep(&mut self) {
        if let Self::Initialized { counter_sleep, .. } = self {
            *counter_sleep += 1;
        }
    }

    pub fn incr_tick(&mut self) {
        if let Self::Initialized { counter_tick, .. } = self {
            *counter_tick += 1;
        }
    }

    pub fn count_sleep(&self) -> Option<u32> {
        if let Self::Initialized { counter_sleep, .. } = self {
            Some(*counter_sleep)
        } else {
            None
        }
    }

    pub fn count_tick(&self) -> Option<u64> {
        if let Self::Initialized { counter_tick, .. } = self {
            Some(*counter_tick)
        } else {
            None
        }
    }

    pub fn duration(&self) -> Option<&Duration> {
        if let Self::Initialized { duration, .. } = self {
            Some(duration)
        } else {
            None
        }
    }

    pub fn increment_delay(&mut self) {
        if let Self::Initialized { duration, .. } = self {
            // Set the duration, note that the duration is in 1s..10s
            let secs = (duration.as_secs() + 1).clamp(1, 10);
            *duration = Duration::from_secs(secs);
        }
    }

    pub fn decrement_delay(&mut self) {
        if let Self::Initialized { duration, .. } = self {
            // Set the duration, note that the duration is in 1s..10s
            let secs = (duration.as_secs() - 1).clamp(1, 10);
            *duration = Duration::from_secs(secs);
        }
    }

    pub fn books(&self) -> Option<&Vec<i32>> {
        if let Self::Initialized { books, .. } = self {
            Some(books)
        } else {
            None
        }
    }

    pub fn next_book(&mut self) {
        if let Self::Initialized {
            books_tablestate,
            books,
            ..
        } = self
        {
            let mut new_pos = books_tablestate.selected().unwrap_or(0) as i32 + 1;
            if new_pos >= books.len() as i32 {
                new_pos = 0;
            }
            books_tablestate.select(Some(new_pos as usize));
        }
    }

    pub fn previous_book(&mut self) {
        if let Self::Initialized {
            books_tablestate,
            books,
            ..
        } = self
        {
            let mut new_pos = books_tablestate.selected().unwrap_or(0) as i32 - 1;
            if new_pos < 0 {
                new_pos = books.len() as i32 - 1;
            }
            books_tablestate.select(Some(new_pos as usize));
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::Init
    }
}
