use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    widgets::{Block, BorderType, List, ListItem, Paragraph, ListState},
    Frame,
};
use crate::app::AppState;
use crate::models::{CurrentPage, TodoItem};

pub fn render(frame: &mut Frame, state: &mut AppState) {
    let chunks = Layout::vertical([
        Constraint::Min(0),
        Constraint::Length(1),
    ]).split(frame.area());

    let [menu_area, content_area] = Layout::horizontal([
        Constraint::Percentage(20),
        Constraint::Percentage(80),
    ]).areas(chunks[0]);

    // On prépare les données avant de descendre dans les sous-fonctions
    let list_data = match state.current_page {
        CurrentPage::Tasks => &state.items,
        CurrentPage::Archives => &state.archives,
    };

    render_menu(frame, state.current_page, menu_area);

    // On passe uniquement ce qui est nécessaire : les items (immuable) et le state de liste (mutable)
    render_content(
        frame,
        list_data,
        &mut state.list_state,
        content_area,
        state.current_page
    );

    render_footer(frame, state.current_page, chunks[1]);
}

fn render_menu(frame: &mut Frame, current_page: CurrentPage, area: Rect) {
    let menu_items = vec![
        ListItem::new(" [1] Tâches").style(menu_style(current_page == CurrentPage::Tasks)),
        ListItem::new(" [2] Archives").style(menu_style(current_page == CurrentPage::Archives)),
    ];
    frame.render_widget(
        List::new(menu_items).block(Block::bordered().title(" Menu ")),
        area
    );
}

fn render_content(
    frame: &mut Frame,
    list_data: &[TodoItem],
    list_state: &mut ListState,
    area: Rect,
    page: CurrentPage
) {
    let title = match page {
        CurrentPage::Tasks => " Mes Tâches ",
        CurrentPage::Archives => " Archives ",
    };

    let items: Vec<ListItem> = list_data.iter().map(|item| {
        let prefix = if item.is_done { "✔ " } else { "☐ " };
        let mut style = Style::default();
        if item.is_done {
            style = style.fg(Color::DarkGray).add_modifier(Modifier::CROSSED_OUT);
        }
        ListItem::new(format!("{}{}", prefix, item.description)).style(style)
    }).collect();

    let list = List::new(items)
        .block(Block::bordered().title(title).border_type(BorderType::Rounded))
        .highlight_symbol("→ ")
        .highlight_style(Style::default().bg(Color::Indexed(236)).fg(Color::Yellow).bold());

    // Ici, le borrow checker est content car list_data et list_state sont disjoints
    frame.render_stateful_widget(list, area, list_state);
}

fn render_footer(frame: &mut Frame, current_page: CurrentPage, area: Rect) {
    let help = match current_page {
        CurrentPage::Tasks => " [Spc] Fait | [Enter] Archiver | [x] Suppr | [J/K] Déplacer | [q] Quitter ",
        CurrentPage::Archives => " [x] Suppr | [J/K] Déplacer | [q] Quitter ",
    };
    frame.render_widget(Paragraph::new(help).on_blue().white(), area);
}

fn menu_style(is_selected: bool) -> Style {
    if is_selected {
        Style::default().fg(Color::Cyan).bold()
    } else {
        Style::default()
    }
}