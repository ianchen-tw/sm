use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use fuzzy_matcher::clangd::ClangdMatcher;
use fuzzy_matcher::FuzzyMatcher;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Terminal,
};
use std::io;
use std::time::Duration;

mod ping;
mod ping_service;
use ping::{get_ping_info_for_latency, get_ping_legend, get_ping_failure_info, PingInfo};
use ping_service::PingManager;

#[derive(Clone)]
struct ServerInfo {
    name: String,
}



impl ServerInfo {
    fn display_name(&self, latency: Option<u32>) -> String {
        match latency {
            Some(lat) => {
                let ping_info = get_ping_info_for_latency(lat);
                format!("{} {} ({})", ping_info.emoji, self.name, ping_info.format_with_latency(lat))
            }
            None => {
                let ping_info = get_ping_failure_info();
                format!("{} {} ({})", ping_info.emoji, self.name, ping_info.description)
            }
        }
    }
    
    fn get_ping_info(&self, latency: Option<u32>) -> PingInfo {
        match latency {
            Some(lat) => get_ping_info_for_latency(lat),
            None => get_ping_failure_info(),
        }
    }
    
    fn get_ping_indicator(&self, latency: Option<u32>) -> (&str, String) {
        match latency {
            Some(lat) => {
                let ping_info = get_ping_info_for_latency(lat);
                (ping_info.emoji, ping_info.format_with_latency(lat))
            }
            None => {
                let ping_info = get_ping_failure_info();
                (ping_info.emoji, ping_info.description.to_string())
            }
        }
    }
    
    fn get_ping_color(&self, latency: Option<u32>) -> Color {
        match latency {
            Some(lat) => get_ping_info_for_latency(lat).color,
            None => get_ping_failure_info().color,
        }
    }
}

struct App {
    servers: Vec<ServerInfo>,
    filtered_servers: Vec<ServerInfo>,
    selected_index: usize,
    list_state: ListState,
    selected_server: Option<String>,
    should_quit: bool,
    input: String,
    matcher: ClangdMatcher,
    ping_manager: PingManager,
}

impl App {
    fn new() -> (App, tokio::task::JoinHandle<()>) {
        let server_names = vec![
            "Production Server 1",
            "Production Server 2", 
            "Staging Server",
            "Development Server",
            "Test Server",
            "Load Balancer",
            "Database Server",
            "Cache Server",
        ];

        let servers: Vec<ServerInfo> = server_names
            .iter()
            .map(|name| ServerInfo {
                name: name.to_string(),
            })
            .collect();

        let mut list_state = ListState::default();
        list_state.select(Some(0));

        // Create ping manager and start monitoring
        let server_names_string: Vec<String> = server_names.iter().map(|s| s.to_string()).collect();
        let (ping_manager, ping_handle) = PingManager::new(server_names_string);

        let mut app = App {
            filtered_servers: servers.clone(),
            servers,
            selected_index: 0,
            list_state,
            selected_server: None,
            should_quit: false,
            input: String::new(),
            matcher: ClangdMatcher::default(),
            ping_manager,
        };
        
        app.update_filtered_servers();
        (app, ping_handle)
    }

    fn next(&mut self) {
        if self.filtered_servers.is_empty() {
            return;
        }
        
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.filtered_servers.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
        self.selected_index = i;
    }

    fn previous(&mut self) {
        if self.filtered_servers.is_empty() {
            return;
        }
        
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.filtered_servers.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
        self.selected_index = i;
    }

    fn select_server(&mut self) {
        if self.filtered_servers.is_empty() {
            return;
        }
        
        if let Some(selected) = self.list_state.selected() {
            if selected < self.filtered_servers.len() {
                self.selected_server = Some(self.filtered_servers[selected].name.clone());
                self.should_quit = true;
            }
        }
    }

    fn update_filtered_servers(&mut self) {
        if self.input.is_empty() {
            self.filtered_servers = self.servers.clone();
        } else {
            // Use fuzzy matching to find servers
            let mut matches: Vec<(ServerInfo, i64)> = self.servers
                .iter()
                .filter_map(|server| {
                    self.matcher
                        .fuzzy_match(&server.name, &self.input)
                        .map(|score| (server.clone(), score))
                })
                .collect();
            
            // Sort by score (higher is better)
            matches.sort_by(|a, b| b.1.cmp(&a.1));
            
            // Extract just the server info
            self.filtered_servers = matches.into_iter().map(|(server, _)| server).collect();
        }
        
        // Reset selection to first item when filter changes
        if !self.filtered_servers.is_empty() {
            self.list_state.select(Some(0));
            self.selected_index = 0;
        } else {
            self.list_state.select(None);
            self.selected_index = 0;
        }
    }



    fn add_char(&mut self, c: char) {
        self.input.push(c);
        self.update_filtered_servers();
    }

    fn delete_char(&mut self) {
        self.input.pop();
        self.update_filtered_servers();
    }

    fn clear_input(&mut self) {
        self.input.clear();
        self.update_filtered_servers();
    }

    fn update_ping_latencies(&mut self) {
        self.ping_manager.update_latencies();
    }
}

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run it
    let (mut app, _ping_handle) = App::new();
    let res = run_app(&mut terminal, &mut app).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    } else if let Some(selected_server) = app.selected_server {
        println!("Selected server: {}", selected_server);
    }

    Ok(())
}

async fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        // Update ping latencies
        app.update_ping_latencies();
        
        terminal.draw(|f| ui(f, app))?;

        if app.should_quit {
            return Ok(());
        }

        // Check for events with a timeout for periodic updates
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            return Ok(());
                        }
                        KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            app.clear_input();
                        }
                        KeyCode::Down => app.next(),
                        KeyCode::Up => app.previous(),
                        KeyCode::Enter => app.select_server(),
                        KeyCode::Char(c) => app.add_char(c),
                        KeyCode::Backspace => app.delete_char(),
                        _ => {}
                    }
                }
            }
        }
        // If no events or timeout occurred, continue loop to update ping latencies
    }
}

fn ui(f: &mut ratatui::Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(4),
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("Server Selection")
        .style(Style::default().fg(Color::Cyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White)),
        );
    f.render_widget(title, chunks[0]);

    // Input field
    let input_style = Style::default().fg(Color::Yellow);
    
    let input_block = Block::default()
        .borders(Borders::ALL)
        .title("Fuzzy Search (Type to search, Ctrl+U to clear)")
        .style(input_style);
    
    let input = Paragraph::new(app.input.as_str())
        .style(input_style)
        .block(input_block);
    
    f.render_widget(input, chunks[1]);
    
    // Set cursor position
    f.set_cursor_position((
        chunks[1].x + app.input.len() as u16 + 1,
        chunks[1].y + 1,
    ));

    // Server list
    let items: Vec<ListItem> = app
        .filtered_servers
        .iter()
        .enumerate()
        .map(|(i, server)| {
            let latency = app.ping_manager.get_latency(&server.name);
            let display_text = server.display_name(latency);
            let ping_color = server.get_ping_color(latency);
            
            let content = if i == app.selected_index {
                Line::from(vec![
                    Span::styled(
                        "► ",
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        display_text,
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                    ),
                ])
            } else {
                Line::from(vec![
                    Span::styled(
                        "  ",
                        Style::default().fg(Color::White),
                    ),
                    Span::styled(
                        display_text,
                        Style::default().fg(Color::White),
                    ),
                ])
            };
            ListItem::new(content)
        })
        .collect();

    let list_title = format!(
        "Available Servers (showing {} of {})",
        app.filtered_servers.len(),
        app.servers.len()
    );
    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(list_title)
                .style(Style::default().fg(Color::White)),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        );

    f.render_stateful_widget(list, chunks[2], &mut app.list_state);

    // Instructions
    let ping_legend = get_ping_legend();
    let instructions = format!(
        "↑/↓: Navigate | Enter: Select | Type: Fuzzy Search | Ctrl+U: Clear | Ctrl+C: Quit\n{}",
        ping_legend
    );
    
    let instructions_widget = Paragraph::new(instructions)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White)),
        );
    f.render_widget(instructions_widget, chunks[3]);


}


