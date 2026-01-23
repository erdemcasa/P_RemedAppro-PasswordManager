use crate::models::{CurrentPage, TodoItem};
use ratatui::widgets::ListState;

pub struct AppState {
    pub items: Vec<TodoItem>,
    pub archives: Vec<TodoItem>,
    pub list_state: ListState,
    pub current_page: CurrentPage,
    pub running: bool,
}

impl Default for AppState {
    fn default() -> Self {
        let mut state = Self {
            items: Vec::new(),
            archives: Vec::new(),
            list_state: ListState::default(),
            current_page: CurrentPage::Tasks,
            running: true,
        };
        state.list_state.select(Some(0));
        state
    }
}

impl AppState {
    pub fn next(&mut self) {
        self.list_state.select_next();
    }

    pub fn previous(&mut self) {
        self.list_state.select_previous();
    }

    pub fn toggle_status(&mut self) {
        if let Some(i) = self.list_state.selected() {
            if self.current_page == CurrentPage::Tasks {
                if let Some(item) = self.items.get_mut(i) {
                    item.is_done = !item.is_done;
                }
            }
        }
    }

    pub fn archive_current(&mut self) {
        if self.current_page != CurrentPage::Tasks { return; }

        if let Some(i) = self.list_state.selected() {
            if i < self.items.len() {
                let item = self.items.remove(i);
                self.archives.push(item);
                self.adjust_selection(i, self.items.len());
            }
        }
    }

    pub fn delete_current(&mut self) {
        if let Some(i) = self.list_state.selected() {
            // On délègue à une méthode qui gère l'emprunt proprement
            let len = {
                let list = match self.current_page {
                    CurrentPage::Tasks => &mut self.items,
                    CurrentPage::Archives => &mut self.archives,
                };
                if i < list.len() {
                    list.remove(i);
                }
                list.len()
            };
            self.adjust_selection(i, len);
        }
    }
    fn adjust_selection(&mut self, removed_index: usize, new_len: usize) {
        if new_len == 0 {
            self.list_state.select(None);
        } else if removed_index >= new_len {
            self.list_state.select(Some(new_len - 1));
        } else {
            self.list_state.select(Some(removed_index));
        }
    }

    pub fn move_up(&mut self) {
        let i = self.list_state.selected().unwrap_or(0);
        let list = self.get_active_list_mut();
        if i > 0 && !list.is_empty() {
            list.swap(i, i - 1);
            self.list_state.select(Some(i - 1));
        }
    }

    pub fn move_down(&mut self) {
        let i = self.list_state.selected().unwrap_or(0);
        let list = self.get_active_list_mut();
        if i < list.len() - 1 {
            list.swap(i, i + 1);
            self.list_state.select(Some(i + 1));
        }
    }

    fn get_active_list_mut(&mut self) -> &mut Vec<TodoItem> {
        match self.current_page {
            CurrentPage::Tasks => &mut self.items,
            CurrentPage::Archives => &mut self.archives,
        }
    }

}