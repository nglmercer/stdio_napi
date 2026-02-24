//! Table rendering for terminal output.
//!
//! This module provides table rendering capabilities with support for
//! various styles, column alignment, borders, and colors.

use colored::*;
use napi_derive::napi;

/// Table border style options
#[derive(PartialEq)]
#[napi]
pub enum BorderStyle {
    /// No borders
    None,
    /// Single line borders
    Single,
    /// Double line borders
    Double,
    /// Rounded corners
    Rounded,
    /// Markdown style
    Markdown,
    /// ASCII style (for compatibility)
    Ascii,
}

/// Column alignment options
#[napi]
pub enum ColumnAlign {
    /// Left alignment (default)
    Left,
    /// Center alignment
    Center,
    /// Right alignment
    Right,
}

/// Table column configuration
#[napi(object)]
#[derive(Clone)]
pub struct TableColumn {
    /// Column header text
    pub header: String,
    /// Column width (auto if not specified)
    pub width: Option<u32>,
    /// Column alignment
    pub align: Option<String>,
    /// Whether to wrap long text
    pub wrap: Option<bool>,
    /// Custom color for header
    pub header_color: Option<String>,
    /// Custom color for cells
    pub cell_color: Option<String>,
}

/// Table row data
#[napi(object)]
#[derive(Clone)]
pub struct TableRow {
    /// Cell values for this row
    pub cells: Vec<String>,
    /// Optional row color
    pub color: Option<String>,
    /// Whether this is a header row
    pub is_header: Option<bool>,
}

/// Table configuration options
#[napi(object)]
#[derive(Clone)]
pub struct TableConfig {
    /// Table columns configuration
    pub columns: Vec<TableColumn>,
    /// Border style
    pub border_style: Option<String>,
    /// Show header row
    pub show_header: Option<bool>,
    /// Table padding
    pub padding: Option<u32>,
    /// Maximum table width
    pub max_width: Option<u32>,
    /// Compact mode (minimal spacing)
    pub compact: Option<bool>,
}

/// Renders a table to a string.
///
/// # Arguments
/// * `rows` - Table rows data
/// * `config` - Table configuration
///
/// # Returns
/// * `Result<String, napi::Error>` - The rendered table as a string
///
/// # Example
/// ```javascript
/// const { render_table } = require('stdio-napi');
/// const table = render_table([
///   { cells: ["Name", "Age", "City"] },
///   { cells: ["Alice", "30", "New York"] },
///   { cells: ["Bob", "25", "Los Angeles"] }
/// ], {
///   columns: [
///     { header: "Name", align: "left" },
///     { header: "Age", align: "center" },
///     { header: "City", align: "left" }
///   ],
///   border_style: "single",
///   show_header: true
/// });
/// console.log(table);
/// ```
#[napi]
pub fn render_table(rows: Vec<TableRow>, config: TableConfig) -> napi::Result<String> {
    if rows.is_empty() {
        return Ok(String::new());
    }

    let border_style = parse_border_style(config.border_style.as_deref());
    let padding = config.padding.unwrap_or(1) as usize;
    let show_header = config.show_header.unwrap_or(true);
    let compact = config.compact.unwrap_or(false);

    // Calculate column widths
    let num_cols = rows.iter().map(|r| r.cells.len()).max().unwrap_or(0);
    if num_cols == 0 {
        return Ok(String::new());
    }

    let mut col_widths: Vec<usize> = vec![0; num_cols];

    // Consider column config widths
    for (i, col) in config.columns.iter().enumerate() {
        if let Some(w) = col.width {
            col_widths[i] = w as usize;
        }
    }

    // Calculate max width from data
    for row in &rows {
        for (i, cell) in row.cells.iter().enumerate() {
            let cell_width = cell.lines().map(|l| l.len()).max().unwrap_or(0);
            if i < col_widths.len() {
                col_widths[i] = col_widths[i].max(cell_width);
            }
        }
    }

    // Apply max_width constraint
    if let Some(max_w) = config.max_width {
        let total_width: usize =
            col_widths.iter().sum::<usize>() + (padding * 2 + 3) * num_cols + 1;
        if total_width > max_w as usize {
            let scale = max_w as f64 / total_width as f64;
            for w in &mut col_widths {
                *w = (*w as f64 * scale) as usize;
            }
        }
    }

    let mut output = String::new();
    let border_chars = get_border_chars(&border_style);

    // Render top border
    if !compact && border_style != BorderStyle::None {
        output.push_str(&render_horizontal_border(
            &col_widths,
            &border_chars,
            padding,
            true,
        ));
        output.push('\n');
    }

    // Render rows
    for (row_idx, row) in rows.iter().enumerate() {
        let is_header_row = row.is_header.unwrap_or(false) || (show_header && row_idx == 0);

        // Render header separator after first row
        if row_idx == 1 && show_header && border_style != BorderStyle::None {
            output.push_str(&render_horizontal_border(
                &col_widths,
                &border_chars,
                padding,
                false,
            ));
            output.push('\n');
        }

        // Handle multi-line cells
        let max_lines = row
            .cells
            .iter()
            .map(|c| c.lines().count())
            .max()
            .unwrap_or(1);

        for line_idx in 0..max_lines {
            if border_style == BorderStyle::None {
                // No border mode
                for (col_idx, cell) in row.cells.iter().enumerate() {
                    let line = cell.lines().nth(line_idx).unwrap_or("");
                    let col_config = config.columns.get(col_idx);
                    let width = col_widths.get(col_idx).copied().unwrap_or(10);
                    let align = parse_align(col_config.and_then(|c| c.align.as_deref()));

                    let colored_line =
                        apply_color(line, row.color.as_deref(), is_header_row, col_config);
                    output.push_str(&format_cell(&colored_line, width, align, padding));

                    if col_idx < row.cells.len() - 1 {
                        output.push_str(&" ".repeat(padding));
                    }
                }
            } else {
                // With borders
                output.push(border_chars.vertical);

                for (col_idx, cell) in row.cells.iter().enumerate() {
                    let line = cell.lines().nth(line_idx).unwrap_or("");
                    let col_config = config.columns.get(col_idx);
                    let width = col_widths.get(col_idx).copied().unwrap_or(10);
                    let align = parse_align(col_config.and_then(|c| c.align.as_deref()));

                    let colored_line =
                        apply_color(line, row.color.as_deref(), is_header_row, col_config);
                    output.push_str(&format_cell(&colored_line, width, align, padding));
                    output.push(border_chars.vertical);
                }
            }
            output.push('\n');
        }
    }

    // Render bottom border
    if !compact && border_style != BorderStyle::None {
        output.push_str(&render_horizontal_border(
            &col_widths,
            &border_chars,
            padding,
            false,
        ));
    }

    Ok(output)
}

/// Prints a table directly to stdout.
///
/// # Arguments
/// * `rows` - Table rows data
/// * `config` - Table configuration
///
/// # Example
/// ```javascript
/// const { print_table } = require('stdio-napi');
/// print_table([
///   { cells: ["Name", "Age"] },
///   { cells: ["Alice", "30"] }
/// ], { border_style: "rounded" });
/// ```
#[napi]
pub fn print_table(rows: Vec<TableRow>, config: TableConfig) -> napi::Result<()> {
    let table = render_table(rows, config)?;
    println!("{}", table);
    Ok(())
}

/// Creates a simple table from headers and rows.
///
/// # Arguments
/// * `headers` - Column headers
/// * `rows` - Data rows (array of arrays)
/// * `border_style` - Border style name
///
/// # Returns
/// * `Result<String, napi::Error>` - The rendered table as a string
///
/// # Example
/// ```javascript
/// const { create_table } = require('stdio-napi');
/// const table = create_table(
///   ["Name", "Age", "City"],
///   [["Alice", "30", "NYC"], ["Bob", "25", "LA"]],
///   "single"
/// );
/// console.log(table);
/// ```
#[napi]
pub fn create_table(
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    border_style: Option<String>,
) -> napi::Result<String> {
    let mut table_rows: Vec<TableRow> = Vec::new();

    // Add header row
    table_rows.push(TableRow {
        cells: headers.clone(),
        color: None,
        is_header: Some(true),
    });

    // Add data rows
    for row in rows {
        table_rows.push(TableRow {
            cells: row,
            color: None,
            is_header: Some(false),
        });
    }

    // Create columns from headers
    let columns: Vec<TableColumn> = headers
        .iter()
        .map(|h| TableColumn {
            header: h.clone(),
            width: None,
            align: None,
            wrap: None,
            header_color: None,
            cell_color: None,
        })
        .collect();

    let config = TableConfig {
        columns,
        border_style,
        show_header: Some(true),
        padding: Some(1),
        max_width: None,
        compact: Some(false),
    };

    render_table(table_rows, config)
}

/// Renders a simple key-value table.
///
/// # Arguments
/// * `data` - Key-value pairs
/// * `border_style` - Border style name
///
/// # Returns
/// * `Result<String, napi::Error>` - The rendered table as a string
///
/// # Example
/// ```javascript
/// const { render_key_value_table } = require('stdio-napi');
/// const table = render_key_value_table({
///   "Name": "Alice",
///   "Age": "30",
///   "City": "New York"
/// }, "rounded");
/// console.log(table);
/// ```
#[napi]
pub fn render_key_value_table(
    data: std::collections::HashMap<String, String>,
    border_style: Option<String>,
) -> napi::Result<String> {
    let mut rows: Vec<TableRow> = Vec::new();

    for (key, value) in data {
        rows.push(TableRow {
            cells: vec![key, value],
            color: None,
            is_header: Some(false),
        });
    }

    let columns = vec![
        TableColumn {
            header: "Key".to_string(),
            width: None,
            align: Some("left".to_string()),
            wrap: None,
            header_color: Some("cyan".to_string()),
            cell_color: Some("yellow".to_string()),
        },
        TableColumn {
            header: "Value".to_string(),
            width: None,
            align: Some("left".to_string()),
            wrap: None,
            header_color: Some("cyan".to_string()),
            cell_color: None,
        },
    ];

    let config = TableConfig {
        columns,
        border_style,
        show_header: Some(false),
        padding: Some(1),
        max_width: None,
        compact: Some(false),
    };

    render_table(rows, config)
}

/// Renders a CSV-formatted table.
///
/// # Arguments
/// * `csv_data` - CSV string data
/// * `delimiter` - Column delimiter (default: comma)
/// * `has_header` - Whether first row is a header
///
/// # Returns
/// * `Result<String, napi::Error>` - The rendered table as a string
///
/// # Example
/// ```javascript
/// const { render_csv_table } = require('stdio-napi');
/// const table = render_csv_table("Name,Age,City\nAlice,30,NYC\nBob,25,LA", ",", true);
/// console.log(table);
/// ```
#[napi]
pub fn render_csv_table(
    csv_data: String,
    delimiter: Option<String>,
    has_header: Option<bool>,
) -> napi::Result<String> {
    let delim = delimiter.unwrap_or_else(|| ",".to_string());
    let header = has_header.unwrap_or(true);

    let lines: Vec<&str> = csv_data.lines().collect();
    if lines.is_empty() {
        return Ok(String::new());
    }

    let mut rows: Vec<TableRow> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        let cells: Vec<String> = line.split(&delim).map(|s| s.trim().to_string()).collect();
        rows.push(TableRow {
            cells,
            color: None,
            is_header: Some(header && i == 0),
        });
    }

    // Create columns from first row if header
    let columns: Vec<TableColumn> = if header && !rows.is_empty() {
        rows[0]
            .cells
            .iter()
            .map(|c| TableColumn {
                header: c.clone(),
                width: None,
                align: None,
                wrap: None,
                header_color: None,
                cell_color: None,
            })
            .collect()
    } else {
        rows.first()
            .map(|r| {
                r.cells
                    .iter()
                    .map(|_| TableColumn {
                        header: String::new(),
                        width: None,
                        align: None,
                        wrap: None,
                        header_color: None,
                        cell_color: None,
                    })
                    .collect()
            })
            .unwrap_or_default()
    };

    let config = TableConfig {
        columns,
        border_style: Some("single".to_string()),
        show_header: Some(header),
        padding: Some(1),
        max_width: None,
        compact: Some(false),
    };

    render_table(rows, config)
}

// Helper functions

fn parse_border_style(style: Option<&str>) -> BorderStyle {
    match style {
        Some("none") => BorderStyle::None,
        Some("single") => BorderStyle::Single,
        Some("double") => BorderStyle::Double,
        Some("rounded") => BorderStyle::Rounded,
        Some("markdown") => BorderStyle::Markdown,
        Some("ascii") => BorderStyle::Ascii,
        _ => BorderStyle::Single,
    }
}

fn parse_align(align: Option<&str>) -> ColumnAlign {
    match align {
        Some("left") => ColumnAlign::Left,
        Some("center") => ColumnAlign::Center,
        Some("right") => ColumnAlign::Right,
        _ => ColumnAlign::Left,
    }
}

struct BorderChars {
    top_left: char,
    top_right: char,
    bottom_left: char,
    bottom_right: char,
    horizontal: char,
    vertical: char,
    #[allow(dead_code)]
    left_tee: char,
    #[allow(dead_code)]
    right_tee: char,
    top_tee: char,
    bottom_tee: char,
    #[allow(dead_code)]
    cross: char,
}

fn get_border_chars(style: &BorderStyle) -> BorderChars {
    match style {
        BorderStyle::None => BorderChars {
            top_left: ' ',
            top_right: ' ',
            bottom_left: ' ',
            bottom_right: ' ',
            horizontal: ' ',
            vertical: ' ',
            left_tee: ' ',
            right_tee: ' ',
            top_tee: ' ',
            bottom_tee: ' ',
            cross: ' ',
        },
        BorderStyle::Single => BorderChars {
            top_left: '┌',
            top_right: '┐',
            bottom_left: '└',
            bottom_right: '┘',
            horizontal: '─',
            vertical: '│',
            left_tee: '├',
            right_tee: '┤',
            top_tee: '┬',
            bottom_tee: '┴',
            cross: '┼',
        },
        BorderStyle::Double => BorderChars {
            top_left: '╔',
            top_right: '╗',
            bottom_left: '╚',
            bottom_right: '╝',
            horizontal: '═',
            vertical: '║',
            left_tee: '╠',
            right_tee: '╣',
            top_tee: '╦',
            bottom_tee: '╩',
            cross: '╬',
        },
        BorderStyle::Rounded => BorderChars {
            top_left: '╭',
            top_right: '╮',
            bottom_left: '╰',
            bottom_right: '╯',
            horizontal: '─',
            vertical: '│',
            left_tee: '├',
            right_tee: '┤',
            top_tee: '┬',
            bottom_tee: '┴',
            cross: '┼',
        },
        BorderStyle::Markdown => BorderChars {
            top_left: '|',
            top_right: '|',
            bottom_left: '|',
            bottom_right: '|',
            horizontal: '-',
            vertical: '|',
            left_tee: '|',
            right_tee: '|',
            top_tee: '|',
            bottom_tee: '|',
            cross: '|',
        },
        BorderStyle::Ascii => BorderChars {
            top_left: '+',
            top_right: '+',
            bottom_left: '+',
            bottom_right: '+',
            horizontal: '-',
            vertical: '|',
            left_tee: '+',
            right_tee: '+',
            top_tee: '+',
            bottom_tee: '+',
            cross: '+',
        },
    }
}

fn render_horizontal_border(
    col_widths: &[usize],
    chars: &BorderChars,
    padding: usize,
    is_top: bool,
) -> String {
    let mut result = String::new();

    let corner = if is_top {
        chars.top_left
    } else {
        chars.bottom_left
    };
    let tee = if is_top {
        chars.top_tee
    } else {
        chars.bottom_tee
    };
    let end_corner = if is_top {
        chars.top_right
    } else {
        chars.bottom_right
    };

    result.push(corner);

    for (i, &width) in col_widths.iter().enumerate() {
        let total_width = width + padding * 2;
        result.push_str(&chars.horizontal.to_string().repeat(total_width));

        if i < col_widths.len() - 1 {
            result.push(tee);
        }
    }

    result.push(end_corner);
    result
}

fn format_cell(content: &str, width: usize, align: ColumnAlign, padding: usize) -> String {
    // Strip ANSI escape codes to get the actual visible text length
    let visible_text = strip_ansi_escapes(content);
    let visible_width = visible_text.len();
    let available = width;

    let (left_pad, right_pad) = match align {
        ColumnAlign::Left => (0, available.saturating_sub(visible_width)),
        ColumnAlign::Right => (available.saturating_sub(visible_width), 0),
        ColumnAlign::Center => {
            let diff = available.saturating_sub(visible_width);
            (diff / 2, diff - diff / 2)
        }
    };

    let cell_padding = " ".repeat(padding);
    let left = " ".repeat(left_pad);
    let right = " ".repeat(right_pad);

    // Truncate if the visible content is too long
    let display_content = if visible_width > width {
        // Find where to truncate considering ANSI codes
        let mut truncated = String::new();
        let mut visible_count = 0;
        let mut chars = content.chars().peekable();
        
        while let Some(c) = chars.next() {
            if c == '\x1b' {
                // Handle ANSI escape sequence
                truncated.push(c);
                while let Some(&next_c) = chars.peek() {
                    truncated.push(chars.next().unwrap());
                    if next_c.is_ascii_alphabetic() {
                        break;
                    }
                }
            } else {
                if visible_count >= width - 3 {
                    truncated.push_str("...");
                    break;
                }
                truncated.push(c);
                visible_count += 1;
            }
        }
        truncated
    } else {
        content.to_string()
    };

    format!(
        "{}{}{}{}{}",
        cell_padding, left, display_content, right, cell_padding
    )
}

fn strip_ansi_escapes(text: &str) -> String {
    let mut result = String::new();
    let mut chars = text.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            // Skip ANSI escape sequence
            while let Some(&next_c) = chars.peek() {
                chars.next();
                if next_c.is_ascii_alphabetic() {
                    break;
                }
            }
        } else {
            result.push(c);
        }
    }
    
    result
}

fn apply_color(
    text: &str,
    row_color: Option<&str>,
    is_header: bool,
    col_config: Option<&TableColumn>,
) -> String {
    if is_header {
        return text.white().bold().to_string();
    }

    if let Some(color) = row_color {
        return apply_color_str(text, color);
    }

    if let Some(config) = col_config {
        if let Some(ref color) = config.cell_color {
            return apply_color_str(text, color);
        }
    }

    text.to_string()
}

fn apply_color_str(text: &str, color: &str) -> String {
    match color.to_lowercase().as_str() {
        "red" => text.red().to_string(),
        "green" => text.green().to_string(),
        "yellow" => text.yellow().to_string(),
        "blue" => text.blue().to_string(),
        "magenta" => text.magenta().to_string(),
        "cyan" => text.cyan().to_string(),
        "white" => text.white().to_string(),
        "black" => text.black().to_string(),
        "bright_red" | "redbright" => text.bright_red().to_string(),
        "bright_green" | "greenbright" => text.bright_green().to_string(),
        "bright_yellow" | "yellowbright" => text.bright_yellow().to_string(),
        "bright_blue" | "bluebright" => text.bright_blue().to_string(),
        "bright_magenta" | "magentabright" => text.bright_magenta().to_string(),
        "bright_cyan" | "cyanbright" => text.bright_cyan().to_string(),
        "bright_white" | "whitebright" => text.bright_white().to_string(),
        _ => text.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_border_style() {
        assert!(matches!(
            parse_border_style(Some("none")),
            BorderStyle::None
        ));
        assert!(matches!(
            parse_border_style(Some("single")),
            BorderStyle::Single
        ));
        assert!(matches!(
            parse_border_style(Some("double")),
            BorderStyle::Double
        ));
        assert!(matches!(
            parse_border_style(Some("rounded")),
            BorderStyle::Rounded
        ));
        assert!(matches!(
            parse_border_style(Some("markdown")),
            BorderStyle::Markdown
        ));
        assert!(matches!(
            parse_border_style(Some("ascii")),
            BorderStyle::Ascii
        ));
        assert!(matches!(parse_border_style(None), BorderStyle::Single));
    }

    #[test]
    fn test_parse_align() {
        assert!(matches!(parse_align(Some("left")), ColumnAlign::Left));
        assert!(matches!(parse_align(Some("center")), ColumnAlign::Center));
        assert!(matches!(parse_align(Some("right")), ColumnAlign::Right));
        assert!(matches!(parse_align(None), ColumnAlign::Left));
    }

    #[test]
    fn test_create_table_basic() {
        let table = create_table(
            vec!["Name".to_string(), "Age".to_string()],
            vec![vec!["Alice".to_string(), "30".to_string()]],
            Some("single".to_string()),
        )
        .unwrap();

        println!("Table output: '{}'", table);
        println!("Table bytes: {:?}", table.as_bytes());
        
        assert!(table.contains("Name"));
        assert!(table.contains("Age"));
        assert!(table.contains("Alice"));
        assert!(table.contains("30"));
    }

    #[test]
    fn test_render_table_empty() {
        let table = render_table(
            vec![],
            TableConfig {
                columns: vec![],
                border_style: None,
                show_header: None,
                padding: None,
                max_width: None,
                compact: None,
            },
        )
        .unwrap();

        assert!(table.is_empty());
    }

    #[test]
    fn test_format_cell_left() {
        let result = format_cell("test", 10, ColumnAlign::Left, 1);
        assert!(result.contains("test"));
    }

    #[test]
    fn test_format_cell_right() {
        let result = format_cell("test", 10, ColumnAlign::Right, 1);
        assert!(result.contains("test"));
    }

    #[test]
    fn test_format_cell_center() {
        let result = format_cell("test", 10, ColumnAlign::Center, 1);
        assert!(result.contains("test"));
    }

    #[test]
    fn test_format_cell_truncate() {
        let result = format_cell("very long text here", 10, ColumnAlign::Left, 0);
        assert!(result.contains("..."));
    }

    #[test]
    fn test_apply_color_str() {
        assert!(apply_color_str("test", "red").contains("test"));
        assert!(apply_color_str("test", "green").contains("test"));
        assert!(apply_color_str("test", "unknown").contains("test"));
    }

    #[test]
    fn test_render_key_value_table() {
        let mut data = std::collections::HashMap::new();
        data.insert("Name".to_string(), "Alice".to_string());
        data.insert("Age".to_string(), "30".to_string());

        let table = render_key_value_table(data, Some("single".to_string())).unwrap();
        assert!(table.contains("Name"));
        assert!(table.contains("Alice"));
    }

    #[test]
    fn test_render_csv_table() {
        let csv = "Name,Age,City\nAlice,30,NYC\nBob,25,LA".to_string();
        let table = render_csv_table(csv, None, Some(true)).unwrap();

        assert!(table.contains("Name"));
        assert!(table.contains("Alice"));
        assert!(table.contains("Bob"));
    }

    #[test]
    fn test_render_csv_table_custom_delimiter() {
        let csv = "Name;Age;City\nAlice;30;NYC".to_string();
        let table = render_csv_table(csv, Some(";".to_string()), Some(true)).unwrap();

        assert!(table.contains("Name"));
        assert!(table.contains("Alice"));
    }
}
