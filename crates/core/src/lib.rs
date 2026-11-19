#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unimplemented,
    clippy::todo,
    clippy::unreachable
)]

use chrono::{DateTime, Datelike, Days, Months, NaiveDate, Utc, Weekday};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

pub const MAX_TITLE_BYTES: usize = 512;
pub const MAX_NOTE_BYTES: usize = 256 * 1024;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TaskList {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Step {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum Repeat {
    #[default]
    Never,
    Daily,
    Weekdays,
    Weekly,
    Monthly,
    Yearly,
}

impl Repeat {
    pub const ALL: [Self; 6] = [
        Self::Never,
        Self::Daily,
        Self::Weekdays,
        Self::Weekly,
        Self::Monthly,
        Self::Yearly,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Never => "Never",
            Self::Daily => "Daily",
            Self::Weekdays => "Weekdays",
            Self::Weekly => "Weekly",
            Self::Monthly => "Monthly",
            Self::Yearly => "Yearly",
        }
    }

    pub fn next_due(self, previous: NaiveDate, today: NaiveDate) -> Option<NaiveDate> {
        let baseline = previous.max(today);
        match self {
            Self::Never => None,
            Self::Daily => baseline.checked_add_days(Days::new(1)),
            Self::Weekly => baseline.checked_add_days(Days::new(7)),
            Self::Monthly => baseline.checked_add_months(Months::new(1)),
            Self::Yearly => baseline.checked_add_months(Months::new(12)),
            Self::Weekdays => {
                let mut candidate = baseline;
                for _ in 0..7 {
                    candidate = candidate.checked_add_days(Days::new(1))?;
                    if !matches!(candidate.weekday(), Weekday::Sat | Weekday::Sun) {
                        return Some(candidate);
                    }
                }
                None
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Task {
    pub id: Uuid,
    pub list_id: Uuid,
    pub title: String,
    pub note: String,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub important: bool,
    pub my_day: Option<NaiveDate>,
    pub due_date: Option<NaiveDate>,
    pub reminder_at: Option<DateTime<Utc>>,
    pub repeat: Repeat,
    pub steps: Vec<Step>,
}

impl Task {
    pub fn is_completed(&self) -> bool {
        self.completed_at.is_some()
    }

    pub fn rename(&mut self, text: &str) -> Result<(), DomainError> {
        self.title = nonempty_title(text)?;
        Ok(())
    }

    pub fn set_note(&mut self, note: &str) -> Result<(), DomainError> {
        if note.len() > MAX_NOTE_BYTES {
            return Err(DomainError::TooLong);
        }
        self.note = note.to_owned();
        Ok(())
    }

    pub fn add_step(&mut self, text: &str) -> Result<Uuid, DomainError> {
        let title = nonempty_title(text)?;
        let id = Uuid::new_v4();
        self.steps.push(Step {
            id,
            title,
            completed: false,
        });
        Ok(id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Workspace {
    pub lists: Vec<TaskList>,
    pub tasks: Vec<Task>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum View {
    MyDay,
    Important,
    Planned,
    All,
    Completed,
    List(Uuid),
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DomainError {
    #[error("A title cannot be empty")]
    EmptyTitle,
    #[error("This text exceeds the allowed length")]
    TooLong,
    #[error("The selected list does not exist")]
    ListMissing,
    #[error("The selected task does not exist")]
    TaskMissing,
    #[error("Unable to calculate the next recurring date")]
    InvalidDate,
}

impl Default for Workspace {
    fn default() -> Self {
        Self {
            lists: vec![TaskList {
                id: Uuid::new_v4(),
                name: "Tasks".to_owned(),
            }],
            tasks: Vec::new(),
        }
    }
}

impl Workspace {
    pub fn inbox_id(&self) -> Option<Uuid> {
        self.lists.first().map(|list| list.id)
    }

    pub fn add_list(&mut self, name: &str) -> Result<Uuid, DomainError> {
        let name = nonempty_title(name)?;
        let id = Uuid::new_v4();
        self.lists.push(TaskList { id, name });
        Ok(id)
    }

    pub fn add_task(&mut self, list_id: Uuid, title: &str) -> Result<Uuid, DomainError> {
        if !self.lists.iter().any(|list| list.id == list_id) {
            return Err(DomainError::ListMissing);
        }
        let title = nonempty_title(title)?;
        let id = Uuid::new_v4();
        self.tasks.push(Task {
            id,
            list_id,
            title,
            note: String::new(),
            created_at: Utc::now(),
            completed_at: None,
            important: false,
            my_day: None,
            due_date: None,
            reminder_at: None,
            repeat: Repeat::Never,
            steps: Vec::new(),
        });
        Ok(id)
    }

    pub fn task_mut(&mut self, id: Uuid) -> Result<&mut Task, DomainError> {
        self.tasks
            .iter_mut()
            .find(|task| task.id == id)
            .ok_or(DomainError::TaskMissing)
    }

    pub fn remove_task(&mut self, id: Uuid) -> Result<(), DomainError> {
        let original = self.tasks.len();
        self.tasks.retain(|task| task.id != id);
        if self.tasks.len() == original {
            return Err(DomainError::TaskMissing);
        }
        Ok(())
    }

    pub fn move_task(&mut self, task_id: Uuid, list_id: Uuid) -> Result<(), DomainError> {
        if !self.lists.iter().any(|list| list.id == list_id) {
            return Err(DomainError::ListMissing);
        }
        self.task_mut(task_id)?.list_id = list_id;
        Ok(())
    }

    pub fn toggle_completed(&mut self, id: Uuid, today: NaiveDate) -> Result<(), DomainError> {
        let task = self.task_mut(id)?;
        if task.completed_at.is_some() {
            task.completed_at = None;
            return Ok(());
        }
        task.completed_at = Some(Utc::now());
        if task.repeat == Repeat::Never {
            return Ok(());
        }
        let due = task.due_date.unwrap_or(today);
        let next = task
            .repeat
            .next_due(due, today)
            .ok_or(DomainError::InvalidDate)?;
        let mut successor = task.clone();
        successor.id = Uuid::new_v4();
        successor.created_at = Utc::now();
        successor.completed_at = None;
        successor.my_day = None;
        successor.due_date = Some(next);
        successor.reminder_at = None;
        for step in &mut successor.steps {
            step.id = Uuid::new_v4();
            step.completed = false;
        }
        self.tasks.push(successor);
        Ok(())
    }

    pub fn visible_tasks(&self, view: &View, today: NaiveDate, query: &str) -> Vec<&Task> {
        let needle = query.trim().to_lowercase();
        let mut result: Vec<_> = self
            .tasks
            .iter()
            .filter(|task| {
                let in_view = match view {
                    View::MyDay => task.my_day == Some(today),
                    View::Important => task.important,
                    View::Planned => task.due_date.is_some(),
                    View::All => true,
                    View::Completed => task.is_completed(),
                    View::List(id) => task.list_id == *id,
                };
                in_view
                    && (needle.is_empty()
                        || task.title.to_lowercase().contains(&needle)
                        || task.note.to_lowercase().contains(&needle))
            })
            .collect();
        result.sort_by_key(|task| {
            (
                task.is_completed(),
                !task.important,
                task.due_date.unwrap_or(NaiveDate::MAX),
                task.created_at,
            )
        });
        result
    }
}

fn nonempty_title(text: &str) -> Result<String, DomainError> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Err(DomainError::EmptyTitle);
    }
    if trimmed.len() > MAX_TITLE_BYTES {
        return Err(DomainError::TooLong);
    }
    Ok(trimmed.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap_or(NaiveDate::MIN)
    }

    #[test]
    fn my_day_does_not_carry_to_tomorrow() {
        let today = date(2026, 11, 19);
        let mut ws = Workspace::default();
        let Some(inbox) = ws.inbox_id() else {
            panic!("default list missing")
        };
        let Ok(id) = ws.add_task(inbox, "Review") else {
            panic!("task rejected")
        };
        let Ok(task) = ws.task_mut(id) else {
            panic!("task not found")
        };
        task.my_day = Some(today);
        assert_eq!(ws.visible_tasks(&View::MyDay, today, "").len(), 1);
        assert!(
            ws.visible_tasks(&View::MyDay, date(2026, 11, 20), "")
                .is_empty()
        );
    }

    #[test]
    fn recurrence_is_not_duplicated_on_second_completion_toggle() {
        let mut ws = Workspace::default();
        let Some(inbox) = ws.inbox_id() else {
            panic!("default list missing")
        };
        let Ok(id) = ws.add_task(inbox, "Weekly") else {
            panic!("task rejected")
        };
        let Ok(task) = ws.task_mut(id) else {
            panic!("task not found")
        };
        task.repeat = Repeat::Weekly;
        task.due_date = Some(date(2026, 11, 19));
        assert!(ws.toggle_completed(id, date(2026, 11, 19)).is_ok());
        assert_eq!(ws.tasks.len(), 2);
        assert!(ws.toggle_completed(id, date(2026, 11, 19)).is_ok());
        assert_eq!(ws.tasks.len(), 2);
    }

    #[test]
    fn recurrence_handles_month_end_and_weekends() {
        assert_eq!(
            Repeat::Monthly.next_due(date(2026, 1, 31), date(2026, 1, 31)),
            Some(date(2026, 2, 28))
        );
        assert_eq!(
            Repeat::Weekdays.next_due(date(2026, 11, 20), date(2026, 11, 20)),
            Some(date(2026, 11, 23))
        );
    }

    #[test]
    fn invalid_titles_are_rejected() {
        let mut ws = Workspace::default();
        assert_eq!(ws.add_list("   "), Err(DomainError::EmptyTitle));
    }
}
