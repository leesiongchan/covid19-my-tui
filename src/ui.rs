use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{BarChart, Block, Borders, Paragraph, Text},
    Frame,
};

pub fn draw_layout<B>(f: &mut Frame<B>, api_res: &crate::CovidApiResponse)
where
    B: Backend,
{
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(4),
                Constraint::Percentage(100),
            ]
            .as_ref(),
        )
        .split(Rect::new(size.x, size.y, size.width, 25));

    draw_title_block(f, chunks[0]);
    draw_overview_block(f, chunks[1], api_res);
    draw_chart_block(f, chunks[2], api_res);
}

fn draw_overview_block<B>(f: &mut Frame<B>, layout_chunk: Rect, api_res: &crate::CovidApiResponse)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Ratio(1, 4),
                Constraint::Ratio(1, 4),
                Constraint::Ratio(1, 4),
                Constraint::Ratio(1, 4),
            ]
            .as_ref(),
        )
        .split(layout_chunk);

    draw_confirmed_cases_block(f, chunks[0], api_res);
    draw_active_cases_block(f, chunks[1], api_res);
    draw_recovered_cases_block(f, chunks[2], api_res);
    draw_fatal_cases_block(f, chunks[3], api_res);
}

fn draw_chart_block<B>(f: &mut Frame<B>, layout_chunk: Rect, api_res: &crate::CovidApiResponse)
where
    B: Backend,
{
    draw_confirmed_cases_bar_chart(f, layout_chunk, api_res);
}

fn draw_title_block<B>(f: &mut Frame<B>, layout_chunk: Rect)
where
    B: Backend,
{
    let block = Block::default().borders(Borders::ALL);
    let lines = [Text::raw("Malaysia COVID-19 Tracker")];
    let title = Paragraph::new(lines.iter())
        .alignment(Alignment::Center)
        .block(block);

    f.render_widget(title, layout_chunk);
}

fn draw_confirmed_cases_bar_chart<B>(
    f: &mut Frame<B>,
    layout_chunk: Rect,
    api_res: &crate::CovidApiResponse,
) where
    B: Backend,
{
    let confirmed_dates = &api_res
        .location
        .timelines
        .confirmed
        .timeline
        .keys()
        .map(|dt| dt.format("%m/%d").to_string())
        .collect::<Vec<_>>();
    let confirmed_cases = &api_res
        .location
        .timelines
        .confirmed
        .timeline
        .values()
        .map(|c| *c)
        .collect::<Vec<_>>();
    let confirmed_data = confirmed_dates
        .iter()
        .map(|d| d.as_str())
        .zip(confirmed_cases.iter().map(|c| *c))
        .rev()
        .take(22)
        .rev()
        .collect::<Vec<_>>();

    let block = Block::default().title("Last 30 days").borders(Borders::ALL);
    let bar_chart = BarChart::default()
        .block(block)
        .bar_width(6)
        .style(Style::default().fg(Color::LightCyan))
        .data(confirmed_data.as_slice())
        .max(api_res.location.latest.confirmed + 1000);

    f.render_widget(bar_chart, layout_chunk);
}

fn draw_confirmed_cases_block<B>(
    f: &mut Frame<B>,
    layout_chunk: Rect,
    api_res: &crate::CovidApiResponse,
) where
    B: Backend,
{
    let block = Block::default().borders(Borders::ALL);
    let lines = [
        Text::raw("Confirmed Cases\n"),
        Text::styled(
            format!("{}", api_res.location.latest.confirmed),
            Style::default().fg(Color::LightRed),
        ),
    ];
    let content = Paragraph::new(lines.iter())
        .alignment(Alignment::Center)
        .block(block);

    f.render_widget(content, layout_chunk);
}

fn draw_active_cases_block<B>(
    f: &mut Frame<B>,
    layout_chunk: Rect,
    api_res: &crate::CovidApiResponse,
) where
    B: Backend,
{
    let block = Block::default().borders(Borders::ALL);
    let lines = [
        Text::raw("Active Cases\n"),
        Text::styled(
            format!(
                "{}",
                api_res.location.latest.confirmed - api_res.location.latest.recovered
            ),
            Style::default().fg(Color::LightYellow),
        ),
    ];
    let content = Paragraph::new(lines.iter())
        .alignment(Alignment::Center)
        .block(block);

    f.render_widget(content, layout_chunk);
}

fn draw_recovered_cases_block<B>(
    f: &mut Frame<B>,
    layout_chunk: Rect,
    api_res: &crate::CovidApiResponse,
) where
    B: Backend,
{
    let block = Block::default().borders(Borders::ALL);
    let lines = [
        Text::raw("Recovered Cases\n"),
        Text::styled(
            format!("{}", api_res.location.latest.recovered),
            Style::default().fg(Color::LightGreen),
        ),
    ];
    let content = Paragraph::new(lines.iter())
        .alignment(Alignment::Center)
        .block(block);

    f.render_widget(content, layout_chunk);
}

fn draw_fatal_cases_block<B>(
    f: &mut Frame<B>,
    layout_chunk: Rect,
    api_res: &crate::CovidApiResponse,
) where
    B: Backend,
{
    let block = Block::default().borders(Borders::ALL);
    let lines = [
        Text::raw("Fatal Cases\n"),
        Text::styled(
            format!("{}", api_res.location.latest.deaths),
            Style::default().fg(Color::DarkGray),
        ),
    ];
    let content = Paragraph::new(lines.iter())
        .alignment(Alignment::Center)
        .block(block);

    f.render_widget(content, layout_chunk);
}
