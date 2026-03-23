//! Board view widget — specs grouped by status.

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use leanspec_core::SpecInfo;

use super::app::{App, FocusPane, PrimaryView};
use super::theme;

pub fn render(area: Rect, buf: &mut Buffer, app: &App) {
    let is_focused = app.focus == FocusPane::Left && app.primary_view == PrimaryView::Board;
    let border_style = if is_focused {
        Style::default().fg(ratatui::style::Color::Cyan)
    } else {
        Style::default().fg(ratatui::style::Color::DarkGray)
    };

    let block = Block::default()
        .title(" Board ")
        .borders(Borders::ALL)
        .border_style(border_style);
    let inner = block.inner(area);
    block.render(area, buf);

    let mut lines: Vec<Line> = Vec::new();

    for (gi, group) in app.board_groups.iter().enumerate() {
        // Group header
        let header_style = theme::status_style(&group.status).add_modifier(Modifier::BOLD);
        let symbol = theme::status_symbol(&group.status);
        lines.push(Line::from(vec![
            Span::styled(format!(" {} {} ", symbol, group.label), header_style),
            Span::styled(format!("({})", group.indices.len()), theme::dimmed_style()),
        ]));

        // Items in group
        for (ii, &spec_idx) in group.indices.iter().enumerate() {
            let spec = &app.specs[spec_idx];
            let is_current = gi == app.board_group_idx && ii == app.board_item_idx;

            let style = if is_current && is_focused {
                theme::selected_style()
            } else if is_current {
                theme::inactive_selected_style()
            } else {
                Style::default()
            };

            let pri = theme::priority_symbol(spec.frontmatter.priority.as_ref());
            let dep_count = app
                .dep_graph
                .get_complete_graph(&spec.path)
                .map_or(0, |g| g.depends_on.len());
            let line = format_spec_line(pri, spec, dep_count);
            lines.push(Line::styled(line, style));
        }

        // Blank line between groups
        lines.push(Line::from(""));
    }

    if lines.is_empty() {
        lines.push(Line::styled("  No specs found", theme::dimmed_style()));
    }

    // Compute scroll offset to keep the selected item visible
    let scroll = compute_scroll(app, inner.height as usize);
    let paragraph = Paragraph::new(lines).scroll((scroll as u16, 0));
    paragraph.render(inner, buf);
}

/// Return the scroll offset (in lines) needed to keep the selected board item visible.
fn compute_scroll(app: &App, visible_height: usize) -> usize {
    let mut selected_row: usize = 0;
    let mut found = false;
    'outer: for (gi, group) in app.board_groups.iter().enumerate() {
        selected_row += 1; // group header
        for ii in 0..group.indices.len() {
            if gi == app.board_group_idx && ii == app.board_item_idx {
                found = true;
                break 'outer;
            }
            selected_row += 1;
        }
        selected_row += 1; // blank line
    }
    if !found || visible_height == 0 {
        return 0;
    }
    selected_row.saturating_sub(visible_height.saturating_sub(1))
}

fn format_spec_line(priority: &str, spec: &SpecInfo, dep_count: usize) -> String {
    let title = if spec.title.chars().count() > 36 {
        let truncated: String = spec.title.chars().take(33).collect();
        format!("{}...", truncated)
    } else {
        spec.title.clone()
    };
    let dep_str = if dep_count > 0 {
        format!(" deps:{}", dep_count)
    } else {
        String::new()
    };
    format!("  {} {} {}{}", priority, spec.path, title, dep_str)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    use super::super::app::App;

    fn buffer_text(buf: &ratatui::buffer::Buffer) -> String {
        buf.content().iter().map(|c| c.symbol()).collect()
    }

    #[test]
    fn test_board_renders_group_headers() {
        let mut app = App::empty_for_test();
        app.board_groups = vec![super::super::app::BoardGroup {
            status: leanspec_core::SpecStatus::Draft,
            label: "Draft".to_string(),
            indices: vec![],
        }];
        let backend = TestBackend::new(60, 20);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|frame| {
                render(frame.area(), frame.buffer_mut(), &app);
            })
            .unwrap();

        let buf_str = buffer_text(terminal.backend().buffer());
        assert!(buf_str.contains("Board"));
        assert!(buf_str.contains("Draft"));
    }
}
