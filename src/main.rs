use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{backend::Backend, Terminal};

use toolbox::{
    app::{App, CurrentScreen, CurrentlyEditing},
    ui::ui,
};

fn main() -> eyre::Result<()> {
    // setup terminal
    toolbox::errors::install_hooks().unwrap();
    let mut terminal = toolbox::tui::init().unwrap();
    // create app and run it
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);
    // restore terminal
    toolbox::tui::restore().unwrap();

    terminal.show_cursor()?;

    if let Ok(do_print) = res {
        if do_print {
            app.print_json()?;
        }
    } else if let Err(err) = res {
        println!("{err:?}");
    }
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }
            match &mut app.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('e') => {
                        app.current_screen = CurrentScreen::Editing(CurrentlyEditing::Key);
                    }
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exiting;
                    }
                    _ => {}
                },
                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') => {
                        return Ok(true);
                    }
                    KeyCode::Char('n') | KeyCode::Char('q') => {
                        return Ok(false);
                    }
                    _ => {}
                },
                CurrentScreen::Editing(editing) if key.kind == KeyEventKind::Press => {
                    match key.code {
                        KeyCode::Enter => match editing {
                            CurrentlyEditing::Key => {
                                *editing = CurrentlyEditing::Value;
                            }
                            CurrentlyEditing::Value => {
                                app.save_key_value();
                                app.current_screen = CurrentScreen::Main;
                            }
                        },
                        KeyCode::Backspace => match editing {
                            CurrentlyEditing::Key => {
                                app.key_input.pop();
                            }
                            CurrentlyEditing::Value => {
                                app.value_input.pop();
                            }
                        },
                        KeyCode::Esc => {
                            app.current_screen = CurrentScreen::Main;
                        }
                        KeyCode::Tab => {
                            app.toggle_editing();
                        }
                        KeyCode::Char(value) => {
                            if let CurrentScreen::Editing(editing) = &app.current_screen {
                                match editing {
                                    CurrentlyEditing::Key => {
                                        app.key_input.push(value);
                                    }
                                    CurrentlyEditing::Value => {
                                        app.value_input.push(value);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}
