use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, ClearType},
};
use std::io;

use std::process::Command;

fn main() {
    let output = Command::new("terraform")
        .arg("workspace")
        .arg("list")
        .output()
        .expect("failed to run terraform workspace list");

    if !output.status.success() {
        eprintln!(
            "terraform workspace list failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
        std::process::exit(1);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let workspaces: Vec<String> = stdout
        .lines()
        .map(|l| l.trim_start_matches(['*', ' ']).trim().to_string())
        .filter(|l| !l.is_empty())
        .collect();

    let current = stdout
        .lines()
        .find(|l| l.trim_start().starts_with('*'))
        .map(|l| l.trim_start_matches(['*', ' ']).trim().to_string());

    let query = std::env::args().nth(1);

    let filtered: Vec<String> = match &query {
        Some(q) => {
            let q_lower = q.to_lowercase();
            if let Some(exact) = workspaces.iter().find(|w| w.to_lowercase() == q_lower) {
                vec![exact.clone()]
            } else {
                workspaces
                    .iter()
                    .filter(|w| w.to_lowercase().contains(&q_lower))
                    .cloned()
                    .collect()
            }
        }
        None => workspaces.clone(),
    };

    if filtered.is_empty() {
        eprintln!("No workspaces match \"{}\".", query.unwrap());
        std::process::exit(1);
    }

    if filtered.len() == 1 {
        std::fs::write(".terraform/environment", &filtered[0])
            .expect("failed to write .terraform/environment");
        return;
    }

    let initial = current
        .as_ref()
        .and_then(|c| filtered.iter().position(|w| w == c))
        .unwrap_or(0);

    let selected = match run_selector(&filtered, initial) {
        Some(i) => i,
        None => {
            println!("Cancelled.");
            return;
        }
    };

    let workspace = &filtered[selected];
    std::fs::write(".terraform/environment", workspace)
        .expect("failed to write .terraform/environment");
}

fn run_selector(workspaces: &[String], initial: usize) -> Option<usize> {
    let mut stdout = io::stdout();
    let mut selected = initial;

    terminal::enable_raw_mode().expect("failed to enable raw mode");

    let result = (|| {
        for (i, ws) in workspaces.iter().enumerate() {
            if i == selected {
                execute!(
                    stdout,
                    SetForegroundColor(Color::Green),
                    Print(format!("> {}\r\n", ws)),
                    ResetColor
                )
                .unwrap();
            } else {
                execute!(stdout, Print(format!("  {}\r\n", ws))).unwrap();
            }
        }

        loop {
            let event = event::read().expect("failed to read event");
            match event {
                Event::Key(KeyEvent { code, .. }) => match code {
                    KeyCode::Up | KeyCode::Char('k') => {
                        selected = selected.saturating_sub(1);
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        if selected < workspaces.len() - 1 {
                            selected += 1;
                        }
                    }
                    KeyCode::Enter => {
                        return Some(selected);
                    }
                    KeyCode::Esc | KeyCode::Char('q') => {
                        return None;
                    }
                    _ => continue,
                },
                _ => continue,
            }

            execute!(
                stdout,
                cursor::MoveUp(workspaces.len() as u16),
                terminal::Clear(ClearType::FromCursorDown)
            )
            .unwrap();

            for (i, ws) in workspaces.iter().enumerate() {
                if i == selected {
                    execute!(
                        stdout,
                        SetForegroundColor(Color::Green),
                        Print(format!("> {}\r\n", ws)),
                        ResetColor
                    )
                    .unwrap();
                } else {
                    execute!(stdout, Print(format!("  {}\r\n", ws))).unwrap();
                }
            }
        }
    })();

    terminal::disable_raw_mode().expect("failed to disable raw mode");
    result
}
