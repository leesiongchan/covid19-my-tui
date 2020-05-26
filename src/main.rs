mod ui;

use chrono::{DateTime, Utc};
use crossterm::{
    event::{read, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::{
    error::Error,
    io::{stdout, Write},
};
use tui::{backend::CrosstermBackend, Terminal};

#[derive(Deserialize)]
struct CovidApiLatest {
    confirmed: u64,
    deaths: u64,
    recovered: u64,
}

#[derive(Deserialize)]
struct CovidApiLocation {
    id: u64,
    country: String,
    country_code: String,
    country_population: u64,
    county: String,
    province: String,
    last_updated: DateTime<Utc>,
    coordinates: CovidApiCoordinates,
    latest: CovidApiLatest,
    timelines: CovidApiTimelineList,
}

#[derive(Deserialize)]
struct CovidApiTimelineItem {
    latest: u64,
    timeline: BTreeMap<DateTime<Utc>, u64>,
}

#[derive(Deserialize)]
struct CovidApiTimelineList {
    confirmed: CovidApiTimelineItem,
    deaths: CovidApiTimelineItem,
    recovered: CovidApiTimelineItem,
}

#[derive(Deserialize)]
struct CovidApiCoordinates {
    latitude: String,
    longtitude: Option<String>,
}

#[derive(Deserialize)]
pub struct CovidApiResponse {
    location: CovidApiLocation,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let api_res: CovidApiResponse =
        reqwest::get("https://coronavirus-tracker-api.herokuapp.com/v2/locations/153")
            .await?
            .json()
            .await?;

    enable_raw_mode()?;

    let mut stdout = stdout();

    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    terminal.clear()?;

    loop {
        terminal.draw(|mut f| {
            ui::draw_layout(&mut f, &api_res);
        })?;

        match read()? {
            Event::Key(event) if event.code == KeyCode::Char('q') => {
                disable_raw_mode()?;
                execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                terminal.show_cursor()?;
                break;
            }
            _ => {}
        }
    }

    Ok(())
}
