use chrono::{DateTime, Local, LocalResult, NaiveDate, NaiveDateTime, TimeZone, Utc};
use eframe::egui::{self, RichText};
use todocraft_core::{Repeat, View, Workspace};
use todocraft_storage::Store;
use uuid::Uuid;

use crate::theme::{self, ThemeMode, Tokens};

pub struct ToDoCraftApp {
    store: Option<Store>,
    workspace: Workspace,
    view: View,
    selected: Option<Uuid>,
    search: String,
    new_list: String,
    new_task: String,
    edit_title: String,
    edit_note: String,
    edit_due: String,
    edit_reminder: String,
    new_step: String,
    confirm_delete: bool,
    status: Option<String>,
    tokens: Tokens,
    theme_mode: ThemeMode,
}

impl ToDoCraftApp {
    pub fn new(cc: &eframe::CreationContext<'_>, store: Result<Store, String>) -> Self {
        let theme_mode = ThemeMode::Light;
        let tokens = theme::apply(&cc.egui_ctx, theme_mode);
        let (store, workspace, status) = match store {
            Ok(storage) => match storage.load() {
                Ok(workspace) => (Some(storage), workspace, None),
                Err(err) => (
                    None,
                    Workspace::default(),
                    Some(format!(
                        "Unable to read ToDoCraft data: {err}. Original data has not been changed."
                    )),
                ),
            },
            Err(err) => (
                None,
                Workspace::default(),
                Some(format!("Unable to open ToDoCraft storage: {err}")),
            ),
        };
        let view = workspace.inbox_id().map(View::List).unwrap_or(View::All);
        Self {
            store,
            workspace,
            view,
            selected: None,
            search: String::new(),
            new_list: String::new(),
            new_task: String::new(),
            edit_title: String::new(),
            edit_note: String::new(),
            edit_due: String::new(),
            edit_reminder: String::new(),
            new_step: String::new(),
            confirm_delete: false,
            status,
            tokens,
            theme_mode,
        }
    }

    fn today() -> NaiveDate {
        Local::now().date_naive()
    }

    fn persist(&mut self) {
        if let Some(store) = self.store.as_mut() {
            match store.save(&self.workspace) {
                Ok(()) => self.status = None,
                Err(err) => {
                    self.status = Some(format!(
                        "Changes could not be saved: {err}. Keep this window open."
                    ));
                }
            }
        }
    }

    fn select_task(&mut self, id: Uuid) {
        if let Some(task) = self.workspace.tasks.iter().find(|task| task.id == id) {
            self.selected = Some(id);
            self.edit_title = task.title.clone();
            self.edit_note = task.note.clone();
            self.edit_due = task
                .due_date
                .map(|date| date.to_string())
                .unwrap_or_default();
            self.edit_reminder = task
                .reminder_at
                .map(|timestamp| {
                    timestamp
                        .with_timezone(&Local)
                        .format("%Y-%m-%d %H:%M")
                        .to_string()
                })
                .unwrap_or_default();
            self.new_step.clear();
            self.confirm_delete = false;
        }
    }

    fn view_name(&self) -> String {
        match &self.view {
            View::MyDay => "My Day".to_owned(),
            View::Important => "Important".to_owned(),
            View::Planned => "Planned".to_owned(),
            View::All => "All tasks".to_owned(),
            View::Completed => "Completed".to_owned(),
            View::List(id) => self
                .workspace
                .lists
                .iter()
                .find(|list| list.id == *id)
                .map(|list| list.name.clone())
                .unwrap_or_else(|| "Tasks".to_owned()),
        }
    }

    fn target_list(&self) -> Option<Uuid> {
        match &self.view {
            View::List(id) => Some(*id),
            _ => self.workspace.inbox_id(),
        }
    }

    fn add_task(&mut self) {
        let Some(list_id) = self.target_list() else {
            self.status = Some("No task list is available".to_owned());
            return;
        };
        match self.workspace.add_task(list_id, &self.new_task) {
            Ok(id) => {
                if matches!(self.view, View::MyDay)
                    && let Ok(task) = self.workspace.task_mut(id)
                {
                    task.my_day = Some(Self::today());
                }
                if matches!(self.view, View::Planned)
                    && let Ok(task) = self.workspace.task_mut(id)
                {
                    task.due_date = Some(Self::today());
                }
                self.new_task.clear();
                self.persist();
                self.select_task(id);
            }
            Err(err) => self.status = Some(err.to_string()),
        }
    }

    fn apply_details(&mut self, id: Uuid) -> Result<(), String> {
        let date = parse_date(&self.edit_due)?;
        let reminder = parse_reminder(&self.edit_reminder)?;
        let task = self.workspace.task_mut(id).map_err(|err| err.to_string())?;
        task.rename(&self.edit_title)
            .map_err(|err| err.to_string())?;
        task.set_note(&self.edit_note)
            .map_err(|err| err.to_string())?;
        task.due_date = date;
        task.reminder_at = reminder;
        self.persist();
        Ok(())
    }

    fn side_navigation(&mut self, ui: &mut egui::Ui) {
        ui.add_space(10.0);
        ui.label(
            RichText::new("Workspace")
                .size(12.0)
                .color(self.tokens.muted_text),
        );
        ui.add_space(14.0);

        for (view, icon, label) in [
            (View::MyDay, "☀", "My Day"),
            (View::Important, "☆", "Important"),
            (View::Planned, "▦", "Planned"),
            (View::All, "☷", "All tasks"),
            (View::Completed, "✓", "Completed"),
        ] {
            let selected = self.view == view;
            let button = egui::Button::new(
                RichText::new(format!("{icon}     {label}"))
                    .color(if selected {
                        self.tokens.accent
                    } else {
                        self.tokens.text
                    })
                    .size(14.0)
                    .strong(),
            )
            .fill(if selected {
                self.tokens.accent_tint
            } else {
                self.tokens.sidebar_surface
            })
            .stroke(egui::Stroke::NONE)
            .corner_radius(8);
            if ui.add_sized([ui.available_width(), 38.0], button).clicked() {
                self.view = view;
            }
        }

        ui.add_space(20.0);
        ui.label(
            RichText::new("Your lists")
                .size(12.0)
                .strong()
                .color(self.tokens.muted_text),
        );
        ui.add_space(8.0);
        let lists: Vec<_> = self
            .workspace
            .lists
            .iter()
            .map(|list| (list.id, list.name.clone()))
            .collect();
        for (id, name) in lists {
            let selected = self.view == View::List(id);
            let button =
                egui::Button::new(RichText::new(format!("☷     {name}")).size(14.0).color(
                    if selected {
                        self.tokens.accent
                    } else {
                        self.tokens.text
                    },
                ))
                .fill(if selected {
                    self.tokens.accent_tint
                } else {
                    self.tokens.sidebar_surface
                })
                .stroke(egui::Stroke::NONE)
                .corner_radius(8);
            if ui.add_sized([ui.available_width(), 36.0], button).clicked() {
                self.view = View::List(id);
            }
        }
        ui.add_space(12.0);
        ui.horizontal(|ui| {
            let response = ui.add(
                egui::TextEdit::singleline(&mut self.new_list)
                    .hint_text("New list")
                    .desired_width((ui.available_width() - 45.0).max(60.0)),
            );
            if ui.button("+").on_hover_text("Create list").clicked()
                || (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))
            {
                match self.workspace.add_list(&self.new_list) {
                    Ok(id) => {
                        self.new_list.clear();
                        self.view = View::List(id);
                        self.persist();
                    }
                    Err(err) => self.status = Some(err.to_string()),
                }
            }
        });
    }

    fn central_view(&mut self, ui: &mut egui::Ui) {
        ui.add_space(10.0);
        let today = Self::today();
        if matches!(self.view, View::MyDay) {
            egui::Frame::new()
                .fill(self.tokens.accent_tint)
                .corner_radius(10)
                .inner_margin(egui::Margin::symmetric(18, 14))
                .show(ui, |ui| {
                    ui.heading(
                        RichText::new("☀  My Day")
                            .size(25.0)
                            .strong()
                            .color(self.tokens.accent),
                    );
                    ui.label(
                        RichText::new(today.format("%A, %d %B").to_string())
                            .color(self.tokens.muted_text),
                    );
                });
        } else {
            ui.heading(RichText::new(self.view_name()).size(25.0).strong());
        }
        ui.add_space(14.0);
        let count = self
            .workspace
            .visible_tasks(&self.view, today, &self.search)
            .len();
        let noun = if count == 1 { "task" } else { "tasks" };
        ui.label(
            RichText::new(format!("{count} {noun}"))
                .size(12.0)
                .color(self.tokens.muted_text),
        );
        ui.add_space(6.0);

        let rows: Vec<_> = self
            .workspace
            .visible_tasks(&self.view, today, &self.search)
            .into_iter()
            .map(|task| {
                (
                    task.id,
                    task.title.clone(),
                    task.is_completed(),
                    task.important,
                    task.due_date,
                )
            })
            .collect();

        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                if rows.is_empty() {
                    ui.add_space(56.0);
                    ui.vertical_centered(|ui| {
                        ui.label(RichText::new("✓").size(28.0).color(self.tokens.accent));
                        ui.add_space(8.0);
                        ui.label(RichText::new("You're all caught up").size(16.0).strong());
                        ui.label(
                            RichText::new("Add a task below when something comes to mind.")
                                .color(self.tokens.muted_text),
                        );
                    });
                }
                for (id, title, completed, important, due) in rows {
                    let selected = self.selected == Some(id);
                    let row_width = (ui.available_width() - 24.0).max(240.0);
                    egui::Frame::new()
                        .fill(if selected {
                            self.tokens.accent_tint
                        } else {
                            self.tokens.surface
                        })
                        .stroke(egui::Stroke::new(
                            1.0,
                            if selected {
                                self.tokens.accent
                            } else {
                                self.tokens.border
                            },
                        ))
                        .corner_radius(8)
                        .inner_margin(egui::Margin::symmetric(12, 6))
                        .show(ui, |ui| {
                            ui.set_min_width(row_width);
                            ui.horizontal(|ui| {
                                let complete_label = if completed { "✓" } else { "○" };
                                if ui
                                    .add_sized(
                                        [30.0, 32.0],
                                        egui::Button::new(
                                            RichText::new(complete_label)
                                                .size(20.0)
                                                .color(self.tokens.accent),
                                        )
                                        .frame(false),
                                    )
                                    .on_hover_text("Toggle completed")
                                    .clicked()
                                {
                                    match self.workspace.toggle_completed(id, today) {
                                        Ok(()) => self.persist(),
                                        Err(err) => self.status = Some(err.to_string()),
                                    }
                                }

                                let text = if completed {
                                    RichText::new(title)
                                        .size(14.0)
                                        .strikethrough()
                                        .color(self.tokens.muted_text)
                                } else {
                                    RichText::new(title).size(14.0).color(self.tokens.text)
                                };
                                let title_width = (ui.available_width() - 117.0).max(90.0);
                                if ui
                                    .add_sized(
                                        [title_width, 32.0],
                                        egui::Button::new(text).frame(false),
                                    )
                                    .clicked()
                                {
                                    self.select_task(id);
                                }
                                if let Some(date) = due {
                                    ui.label(
                                        RichText::new(date.format("%d %b").to_string())
                                            .size(12.0)
                                            .color(self.tokens.muted_text),
                                    );
                                }
                                if ui
                                    .add_sized(
                                        [28.0, 30.0],
                                        egui::Button::new(
                                            RichText::new(if important { "★" } else { "☆" })
                                                .size(19.0)
                                                .color(if important {
                                                    self.tokens.important
                                                } else {
                                                    self.tokens.muted_text
                                                }),
                                        )
                                        .frame(false),
                                    )
                                    .on_hover_text("Toggle important")
                                    .clicked()
                                    && let Ok(task) = self.workspace.task_mut(id)
                                {
                                    task.important = !task.important;
                                    self.persist();
                                }
                            });
                        });
                    ui.add_space(5.0);
                }
            });
    }

    fn add_task_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.add_space(8.0);
            let response = ui.add(
                egui::TextEdit::singleline(&mut self.new_task)
                    .hint_text("Add a task")
                    .desired_width((ui.available_width() - 106.0).max(100.0)),
            );
            let add = egui::Button::new(
                RichText::new("+  Add")
                    .color(self.tokens.on_accent)
                    .strong(),
            )
            .fill(self.tokens.accent)
            .corner_radius(7);
            if ui.add_sized([82.0, 34.0], add).clicked()
                || (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))
            {
                self.add_task();
            }
        });
    }

    fn inspector(&mut self, ui: &mut egui::Ui) {
        let Some(id) = self.selected else {
            ui.label("Choose a task to see its details.");
            return;
        };
        let Some(task) = self
            .workspace
            .tasks
            .iter()
            .find(|task| task.id == id)
            .cloned()
        else {
            self.selected = None;
            return;
        };
        ui.horizontal(|ui| {
            ui.heading("Task details");
            if ui.button("✕").on_hover_text("Close details").clicked() {
                self.selected = None;
            }
        });
        ui.separator();
        ui.label("Title");
        ui.text_edit_singleline(&mut self.edit_title);
        ui.add_space(8.0);
        ui.label("Notes");
        ui.add(
            egui::TextEdit::multiline(&mut self.edit_note)
                .hint_text("Add a note")
                .desired_rows(4),
        );
        ui.add_space(8.0);
        ui.label("Due date (YYYY-MM-DD)");
        ui.text_edit_singleline(&mut self.edit_due);
        ui.label("Reminder (local YYYY-MM-DD HH:MM)");
        ui.text_edit_singleline(&mut self.edit_reminder);
        ui.add_space(8.0);
        if ui.button("Save details").clicked()
            && let Err(err) = self.apply_details(id)
        {
            self.status = Some(err);
        }
        ui.separator();
        let today = Self::today();
        if ui
            .button(if task.my_day == Some(today) {
                "Remove from My Day"
            } else {
                "Add to My Day"
            })
            .clicked()
            && let Ok(value) = self.workspace.task_mut(id)
        {
            value.my_day = if value.my_day == Some(today) {
                None
            } else {
                Some(today)
            };
            self.persist();
        }

        let mut repeat = task.repeat;
        egui::ComboBox::from_id_salt(("repeat", id))
            .selected_text(repeat.label())
            .show_ui(ui, |ui| {
                for choice in Repeat::ALL {
                    ui.selectable_value(&mut repeat, choice, choice.label());
                }
            });
        if repeat != task.repeat
            && let Ok(value) = self.workspace.task_mut(id)
        {
            value.repeat = repeat;
            self.persist();
        }

        ui.label("Move to list");
        let mut list_id = task.list_id;
        let list_name = self
            .workspace
            .lists
            .iter()
            .find(|list| list.id == list_id)
            .map(|list| list.name.as_str())
            .unwrap_or("Tasks")
            .to_owned();
        egui::ComboBox::from_id_salt(("move", id))
            .selected_text(list_name)
            .show_ui(ui, |ui| {
                for list in &self.workspace.lists {
                    ui.selectable_value(&mut list_id, list.id, &list.name);
                }
            });
        if list_id != task.list_id {
            match self.workspace.move_task(id, list_id) {
                Ok(()) => self.persist(),
                Err(err) => self.status = Some(err.to_string()),
            }
        }

        ui.add_space(10.0);
        ui.heading("Steps");
        for step in task.steps {
            ui.horizontal(|ui| {
                if ui.button(if step.completed { "☑" } else { "☐" }).clicked()
                    && let Ok(item) = self.workspace.task_mut(id)
                    && let Some(value) = item.steps.iter_mut().find(|value| value.id == step.id)
                {
                    value.completed = !value.completed;
                    self.persist();
                }
                let text = if step.completed {
                    RichText::new(step.title).strikethrough()
                } else {
                    RichText::new(step.title)
                };
                ui.label(text);
                if ui.small_button("✕").on_hover_text("Delete step").clicked()
                    && let Ok(item) = self.workspace.task_mut(id)
                {
                    item.steps.retain(|value| value.id != step.id);
                    self.persist();
                }
            });
        }
        ui.horizontal(|ui| {
            ui.add(
                egui::TextEdit::singleline(&mut self.new_step)
                    .hint_text("New step")
                    .desired_width(ui.available_width() - 45.0),
            );
            if ui.button("+").on_hover_text("Add step").clicked()
                && let Ok(item) = self.workspace.task_mut(id)
            {
                match item.add_step(&self.new_step) {
                    Ok(_) => {
                        self.new_step.clear();
                        self.persist();
                    }
                    Err(err) => self.status = Some(err.to_string()),
                }
            }
        });
        ui.separator();
        if !self.confirm_delete {
            if ui.button("Delete task…").clicked() {
                self.confirm_delete = true;
            }
        } else {
            ui.colored_label(self.tokens.important, "Delete this task permanently?");
            ui.horizontal(|ui| {
                if ui.button("Cancel").clicked() {
                    self.confirm_delete = false;
                }
                if ui.button("Confirm delete").clicked() {
                    match self.workspace.remove_task(id) {
                        Ok(()) => {
                            self.selected = None;
                            self.persist();
                        }
                        Err(err) => self.status = Some(err.to_string()),
                    }
                    self.confirm_delete = false;
                }
            });
        }
    }
}

impl eframe::App for ToDoCraftApp {
    fn ui(&mut self, root: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if self.store.is_none() {
            egui::CentralPanel::default().show(root, |ui| {
                ui.heading("ToDoCraft needs attention");
                if let Some(message) = &self.status {
                    ui.colored_label(self.tokens.important, message);
                }
                ui.label(
                    "No tasks have been loaded or overwritten. Resolve the storage problem and relaunch.",
                );
            });
            return;
        }
        egui::Panel::top("toolbar")
            .frame(
                egui::Frame::new()
                    .fill(self.tokens.surface)
                    .inner_margin(egui::Margin::symmetric(18, 10))
                    .stroke(egui::Stroke::new(1.0, self.tokens.border)),
            )
            .show(root, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("◉").color(self.tokens.accent).size(20.0));
                    ui.label(
                        RichText::new("ToDoCraft")
                            .strong()
                            .size(17.0)
                            .color(self.tokens.text),
                    );
                    ui.add_space(24.0);
                    ui.add(
                        egui::TextEdit::singleline(&mut self.search)
                            .hint_text("⌕  Search tasks")
                            .desired_width(300.0),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button(self.theme_mode.switch_label()).clicked() {
                            self.theme_mode = self.theme_mode.toggle();
                            self.tokens = theme::apply(ui.ctx(), self.theme_mode);
                        }
                    });
                });
            });
        egui::Panel::left("navigation")
            .default_size(224.0)
            .min_size(180.0)
            .frame(
                egui::Frame::new()
                    .fill(self.tokens.sidebar_surface)
                    .inner_margin(egui::Margin::symmetric(14, 10)),
            )
            .show(root, |ui| self.side_navigation(ui));
        if self.selected.is_some() {
            egui::Panel::right("inspector")
                .default_size(310.0)
                .min_size(260.0)
                .resizable(true)
                .frame(
                    egui::Frame::new()
                        .fill(self.tokens.surface)
                        .inner_margin(egui::Margin::symmetric(16, 16)),
                )
                .show(root, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| self.inspector(ui));
                });
        }
        egui::Panel::bottom("composer")
            .frame(
                egui::Frame::new()
                    .fill(self.tokens.surface)
                    .stroke(egui::Stroke::new(1.0, self.tokens.border))
                    .inner_margin(egui::Margin::symmetric(18, 10)),
            )
            .show(root, |ui| self.add_task_bar(ui));
        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(self.tokens.background)
                    .inner_margin(egui::Margin::symmetric(20, 16)),
            )
            .show(root, |ui| {
                if let Some(message) = &self.status {
                    ui.colored_label(self.tokens.important, message);
                    ui.add_space(8.0);
                }
                self.central_view(ui);
            });
    }
}

fn parse_date(text: &str) -> Result<Option<NaiveDate>, String> {
    if text.trim().is_empty() {
        return Ok(None);
    }
    NaiveDate::parse_from_str(text.trim(), "%Y-%m-%d")
        .map(Some)
        .map_err(|_| "Enter a valid due date as YYYY-MM-DD".to_owned())
}

fn parse_reminder(text: &str) -> Result<Option<DateTime<Utc>>, String> {
    if text.trim().is_empty() {
        return Ok(None);
    }
    let naive = NaiveDateTime::parse_from_str(text.trim(), "%Y-%m-%d %H:%M")
        .map_err(|_| "Enter a valid reminder as YYYY-MM-DD HH:MM".to_owned())?;
    match Local.from_local_datetime(&naive) {
        LocalResult::Single(datetime) => Ok(Some(datetime.with_timezone(&Utc))),
        _ => Err("That reminder time is ambiguous or does not exist in your time zone".to_owned()),
    }
}
