use std::result::Result::Ok;
use ratatui::widgets::{Block, List, ListItem, ListState};
use color_eyre::eyre::{Ok as OtherOk, Result};
use ratatui::{
    crossterm::event::{self, Event},
    widgets::{Paragraph, Widget},
    DefaultTerminal, Frame
};
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Style, Stylize};

#[derive(Debug, PartialEq)]
enum CurrentPage {
    Tasks,
    Archives,
}

impl TodoItem {
    fn new(description: &str) -> Self {
        Self {
            is_done: false,
            description: description.to_string(),
        }
    }
}

struct AppState {
    items: Vec<TodoItem>,
    archives: Vec<TodoItem>,
    list_state: ListState,
    current_page: CurrentPage,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            archives: Vec::new(),
            list_state: ListState::default(),
            current_page: CurrentPage::Tasks,
        }
    }
}
#[derive(Debug, Default)]
struct TodoItem {
    is_done: bool,
    description: String,
}

fn main() -> Result<()> {
    let mut state = AppState::default();

    let task_names = [
        "Acheter", "Réviser", "Nettoyer", "Appeler", "Coder", "Préparer", "Lire", "Organiser"
    ];
    let objects = [
        "le pain", "le projet Rust", "la cuisine", "le client", "le menu TUI", "le café", "la doc Ratatui", "le bureau"
    ];

    let items = (1..=67).map(|i| {
        let task = task_names[i % task_names.len()];
        let obj = objects[i % objects.len()];
        TodoItem::new(&format!("{} - {} {}", i, task, obj ))
    });

    state.items.extend(items);

    state.list_state.select(Some(0));

    color_eyre::install()?;

    let terminal = ratatui::init();

    let result = run(terminal, &mut state);

    ratatui::restore();

    // 6. Gestion du résultat final
    match result {
        Err(e) => {
            eprintln!("L'application a crashé : {}", e);
            Err(e)
        }
        Ok(_) => Ok(()),
    }
}

fn run(mut terminal: DefaultTerminal, app_state: &mut AppState) -> Result<()> {
    loop {
        terminal.draw(|f| render(f, app_state))?;

        if let Event::Key(key) = event::read()? {
            if key.kind != event::KeyEventKind::Press { continue; }

            match key.code {
                event::KeyCode::Esc => break,

                event::KeyCode::Char('1') => {
                    app_state.current_page = CurrentPage::Tasks;
                    app_state.list_state.select(Some(0));
                }
                event::KeyCode::Char('2') => {
                    app_state.current_page = CurrentPage::Archives;
                    app_state.list_state.select(Some(0));
                }

                event::KeyCode::Up => app_state.list_state.select_previous(),
                event::KeyCode::Down => app_state.list_state.select_next(),

                event::KeyCode::Right => {
                    if app_state.current_page == CurrentPage::Tasks {
                        if let Some(index) = app_state.list_state.selected() {
                            let item = app_state.items.remove(index);
                            app_state.archives.push(item);
                        }
                    }
                }
                _ => {}
            }
        }
    }
    Ok({})
}

fn render(frame: &mut Frame, app_state: &mut AppState) {
    let [menu_area, content_area] = Layout::horizontal([
        Constraint::Percentage(20),
        Constraint::Percentage(80),
    ]).areas(frame.area());

    let menu_items = vec![
        ListItem::new(" [1] Tâches").style(if app_state.current_page == CurrentPage::Tasks { Style::default().fg(Color::Yellow).bold() } else { Style::default() }),
        ListItem::new(" [2] Archives").style(if app_state.current_page == CurrentPage::Archives { Style::default().fg(Color::Yellow).bold() } else { Style::default() }),
        ListItem::new(""),
        ListItem::new(" [Esc] Quitter"),
    ];
    frame.render_widget(List::new(menu_items).block(Block::bordered().title(" Menu ")), menu_area);

    let title = match app_state.current_page {
        CurrentPage::Tasks => " Mes Tâches ",
        CurrentPage::Archives => " Archives ",
    };

    let list_block = Block::bordered().title(title).border_type(ratatui::widgets::BorderType::Rounded);
    let list_inner_area = list_block.inner(content_area);
    frame.render_widget(list_block, content_area);

    let display_items = match app_state.current_page {
        CurrentPage::Tasks => &app_state.items,
        CurrentPage::Archives => &app_state.archives,
    };

    let list = List::new(display_items.iter().map(|x| ListItem::from(x.description.clone())))
        .highlight_symbol("> ")
        .highlight_style(Style::default().fg(Color::Green).bold());

    frame.render_stateful_widget(list, list_inner_area, &mut app_state.list_state);
}