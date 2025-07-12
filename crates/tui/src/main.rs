use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use fuzzy_matcher::clangd::ClangdMatcher;
use fuzzy_matcher::FuzzyMatcher;
use rand::Rng;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Terminal,
};
use std::io;

mod ping;
use ping::{get_ping_info_for_latency, get_ping_legend, PingInfo};

#[derive(Clone)]
struct ServerInfo {
    name: String,
    latency: u32,
}



impl ServerInfo {
    fn display_name(&self) -> String {
        let ping_info = self.get_ping_info();
        format!("{} {} {}", self.name, ping_info.emoji, ping_info.format_with_latency(self.latency))
    }
    
    fn get_ping_info(&self) -> PingInfo {
        get_ping_info_for_latency(self.latency)
    }
    
    fn get_ping_indicator(&self) -> (&str, String) {
        let ping_info = self.get_ping_info();
        (ping_info.emoji, ping_info.format_with_latency(self.latency))
    }
    
    fn get_ping_color(&self) -> Color {
        self.get_ping_info().color
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
}

impl App {
    fn new() -> App {
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

        let mut rng = rand::thread_rng();
        let servers: Vec<ServerInfo> = server_names
            .into_iter()
            .map(|name| ServerInfo {
                name: name.to_string(),
                latency: rng.gen_range(10..=500), // Random latency between 10ms and 500ms
            })
            .collect();

        let mut list_state = ListState::default();
        list_state.select(Some(0));

        let mut app = App {
            filtered_servers: servers.clone(),
            servers,
            selected_index: 0,
            list_state,
            selected_server: None,
            should_quit: false,
            input: String::new(),
            matcher: ClangdMatcher::default(),
        };
        
        app.update_filtered_servers();
        app
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


}

fn main() -> Result<(), io::Error> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run it
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

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

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if app.should_quit {
            return Ok(());
        }

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
            let (ping_emoji, ping_text) = server.get_ping_indicator();
            let ping_color = server.get_ping_color();
            
            let content = if i == app.selected_index {
                Line::from(vec![
                    Span::styled(
                        "► ",
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        &server.name,
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        " ",
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        ping_emoji,
                        Style::default().fg(ping_color).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!(" {}", ping_text),
                        Style::default().fg(ping_color).add_modifier(Modifier::BOLD),
                    ),
                ])
            } else {
                Line::from(vec![
                    Span::styled(
                        "  ",
                        Style::default().fg(Color::White),
                    ),
                    Span::styled(
                        &server.name,
                        Style::default().fg(Color::White),
                    ),
                    Span::styled(
                        " ",
                        Style::default().fg(Color::White),
                    ),
                    Span::styled(
                        ping_emoji,
                        Style::default().fg(ping_color),
                    ),
                    Span::styled(
                        format!(" {}", ping_text),
                        Style::default().fg(ping_color),
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


