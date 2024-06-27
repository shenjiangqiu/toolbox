use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, List, ListDirection, ListState};
use strum::{Display, EnumIter, IntoEnumIterator};
#[derive(Debug, EnumIter, Display)]
enum AllApps {
    TorrentsOpt,
    FileOpt,
}

fn main() -> eyre::Result<()> {
    let mut terminal = toolbox::tui::init().unwrap();
    toolbox::errors::install_hooks().unwrap();

    // run app here
    let app = App::default();
    let mut app_state = AppState::default();
    loop {
        terminal.draw(|frame| {
            frame.render_stateful_widget(&app, frame.size(), &mut app_state);
        })?;
        if !handle_events(&app, &mut app_state) {
            break;
        }
    }

    // need to restore the terminal

    toolbox::tui::restore().unwrap();
    Ok(())
}
#[derive(Debug)]
struct App {
    all_apps: Vec<AllApps>,
}
#[derive(Default)]
struct AppState {
    list_state: ListState,
}
impl Default for App {
    fn default() -> Self {
        Self {
            all_apps: AllApps::iter().collect(),
        }
    }
}

impl StatefulWidget for &App {
    type State = AppState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let list = List::new(self.all_apps.iter().map(ToString::to_string))
            .block(
                Block::bordered()
                    .title("List")
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true)
            .direction(ListDirection::TopToBottom);
        StatefulWidget::render(list, area, buf, &mut state.list_state);
    }
}

/// return is_continue
fn handle_events(app: &App, app_state: &mut AppState) -> bool {
    let _ = app;
    let event = event::read();

    if let Ok(Event::Key(KeyEvent {
        code,
        modifiers: _,
        kind: KeyEventKind::Release,
        state: _,
    })) = event
    {
        match code {
            KeyCode::Char(code) => match code {
                'W' | 'w' => {
                    app_state.list_state.select_previous();
                }
                'S' | 's' => {
                    app_state.list_state.select_next();
                }
                'Q' | 'q' => {
                    return false;
                }
                _ => {}
            },
            KeyCode::Up => {
                app_state.list_state.select_previous();
            }
            KeyCode::Down => {
                app_state.list_state.select_next();
            }
            KeyCode::Esc => {
                *app_state.list_state.selected_mut() = None;
            }
            _ => {}
        }
    }
    true
}
