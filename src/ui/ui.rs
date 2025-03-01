//! Terminal UI implementation

use crate::app::App;
use crate::Result as AppResult;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use std::io;

/// Setup the terminal
pub fn setup_terminal() -> AppResult<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

/// Restore the terminal
pub fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> AppResult<()> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

/// Run the application
pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> AppResult<()> {
    // Initial update
    app.update_filters()?;

    loop {
        terminal.draw(|f| ui(f, &app))?;

        if app.should_quit() {
            break;
        }

        if let Event::Key(key) = event::read()? {
            app.handle_key_event(key)?;
        }
    }

    Ok(())
}

/// Render the UI
pub fn ui(f: &mut Frame, app: &App) {
    // Create constraints for all possible filters (up to max_dimensions)
    let mut constraints = Vec::with_capacity(app.max_dimensions() + 2); // +2 for results and status line
    
    // Add constraints for each filter dimension
    for _ in 0..app.max_dimensions() {
        constraints.push(Constraint::Length(3));
    }
    
    // Add constraints for results and status line
    constraints.push(Constraint::Min(1));     // Results
    constraints.push(Constraint::Length(1));  // Status line
    
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(constraints)
        .split(f.size());

    // Render each filter dimension
    for dim in 0..app.max_dimensions() {
        let is_active = dim == app.current_dimension();
        let has_content = !app.filter(dim).is_empty();
        
        // Skip rendering dimensions beyond the current one if they're empty
        if dim > app.current_dimension() && !has_content {
            continue;
        }
        
        // Create dimension name
        let dimension_name = match dim {
            0 => "1st".to_string(),
            1 => "2nd".to_string(),
            2 => "3rd".to_string(),
            _ => format!("{}th", dim + 1),
        };
        
        // Create filter title
        let filter_title = if is_active {
            format!("Filter ({} dimension) [active]", dimension_name)
        } else {
            format!("Filter ({} dimension)", dimension_name)
        };

        // Set filter style
        let filter_style = if is_active || has_content {
            Style::default()
        } else {
            Style::default().fg(Color::DarkGray)
        };

        // Create filter block with title
        let filter_block = Block::default()
            .borders(Borders::ALL)
            .title(filter_title);

        // Create and render filter paragraph
        let filter_text = app.filter(dim);
        let filter = Paragraph::new(filter_text)
            .style(filter_style)
            .block(filter_block);
            
        f.render_widget(filter, chunks[dim]);
    }

    // Results (positioned after all filter dimensions)
    let results_idx = app.max_dimensions();
    let results = app
        .filtered_entries()
        .iter()
        .enumerate()
        .map(|(i, entry)| {
            let content = format!("{}", entry.command);
            let style = if i == app.selected_index() {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(content).style(style)
        })
        .collect::<Vec<_>>();

    let results_count = format!("{} results", results.len());
    let results_list = List::new(results)
        .block(Block::default().borders(Borders::ALL).title(results_count));
    f.render_widget(results_list, chunks[results_idx]);

    // Status line (positioned after results)
    let status_idx = app.max_dimensions() + 1;
    let can_add_dimension = app.current_dimension() < app.max_dimensions() - 1 && 
                           !app.filter(app.current_dimension()).is_empty();
    
    let status = if can_add_dimension {
        format!("Press Ctrl+R to add a {}th dimension filter | Up/Down to navigate | Enter to select | Esc to cancel", 
                app.current_dimension() + 2)
    } else {
        "Up/Down to navigate | Enter to select | Esc to cancel".to_string()
    };

    let status_line = Paragraph::new(status)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default());
    f.render_widget(status_line, chunks[status_idx]);

    // Set cursor position at the end of the current filter
    let current_dim = app.current_dimension();
    f.set_cursor(
        chunks[current_dim].x + app.filter(current_dim).len() as u16 + 1,
        chunks[current_dim].y + 1,
    );
}
