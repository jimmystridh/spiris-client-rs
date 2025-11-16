use crate::app::{App, InputMode, Screen};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    // Header
    draw_header(f, chunks[0], app);

    // Main content
    match &app.screen {
        Screen::Home => draw_home(f, chunks[1], app),
        Screen::Dashboard => draw_dashboard(f, chunks[1], app),
        Screen::Auth => draw_auth(f, chunks[1], app),
        Screen::Customers => draw_customers(f, chunks[1], app),
        Screen::CustomerCreate => draw_customer_form(f, chunks[1], app),
        Screen::CustomerEdit(id) => draw_customer_edit_form(f, chunks[1], app, id),
        Screen::CustomerDetail(id) => draw_customer_detail(f, chunks[1], app, id),
        Screen::Invoices => draw_invoices(f, chunks[1], app),
        Screen::InvoiceCreate => draw_invoice_form(f, chunks[1], app),
        Screen::InvoiceDetail(id) => draw_invoice_detail(f, chunks[1], app, id),
        Screen::Articles => draw_articles(f, chunks[1], app),
        Screen::ArticleCreate => draw_article_form(f, chunks[1], app),
        Screen::ArticleDetail(id) => draw_article_detail(f, chunks[1], app, id),
        Screen::Search => draw_search(f, chunks[1], app),
        Screen::Export => draw_export(f, chunks[1], app),
        Screen::Help => draw_help(f, chunks[1]),
    }

    // Footer
    draw_footer(f, chunks[2], app);
}

fn draw_header(f: &mut Frame, area: Rect, app: &App) {
    let title = match app.client {
        Some(_) => "Spiris Bokföring och Fakturering - TUI",
        None => "Spiris Bokföring och Fakturering - TUI (Not Authenticated)",
    };

    let header = Paragraph::new(title)
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(header, area);
}

fn draw_footer(f: &mut Frame, area: Rect, app: &App) {
    let keys = match app.input_mode {
        InputMode::Normal => {
            "Tab: Next | Enter: Select | q: Quit | n: New | e: Edit | r: Refresh | s: Search | d: Dashboard | h: Help"
        }
        InputMode::Editing => "Enter: Next field | Esc: Cancel",
    };

    let footer = Paragraph::new(keys)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(footer, area);
}

fn draw_home(f: &mut Frame, area: Rect, app: &App) {
    let items = vec![
        ListItem::new("Dashboard - View statistics and quick access"),
        ListItem::new("Customers - Browse and manage customers"),
        ListItem::new("Invoices - Browse and manage invoices"),
        ListItem::new("Articles - Browse and manage products/articles"),
        ListItem::new("Search - Search across all entities"),
        ListItem::new("Export - Export data to JSON"),
        ListItem::new("Help - View keyboard shortcuts"),
    ];

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Main Menu"))
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(
        list,
        area,
        &mut ratatui::widgets::ListState::default().with_selected(Some(app.selected_customer)),
    );
}

fn draw_auth(f: &mut Frame, area: Rect, app: &App) {
    let mut text = vec![
        Line::from("OAuth2 Authentication Required"),
        Line::from(""),
        Line::from("Press Enter to start OAuth2 flow"),
        Line::from(""),
    ];

    if let Some(url) = &app.oauth_url {
        text.push(Line::from(""));
        text.push(Line::from("Open this URL in your browser:"));
        text.push(Line::from(""));
        text.push(Line::from(Span::styled(
            url.clone(),
            Style::default().fg(Color::Yellow),
        )));
        text.push(Line::from(""));
        text.push(Line::from(
            "After authorization, you'll receive a code. Use the CLI to complete the flow.",
        ));
    }

    if let Some(msg) = &app.status_message {
        text.push(Line::from(""));
        text.push(Line::from(Span::styled(
            msg.clone(),
            Style::default().fg(Color::Green),
        )));
    }

    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Authentication"))
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Center);

    f.render_widget(paragraph, area);
}

fn draw_customers(f: &mut Frame, area: Rect, app: &App) {
    if app.loading {
        let loading = Paragraph::new("Loading customers...")
            .block(Block::default().borders(Borders::ALL).title("Customers"))
            .alignment(Alignment::Center);
        f.render_widget(loading, area);
        return;
    }

    if app.customers.is_empty() {
        let empty = Paragraph::new("No customers found. Press 'n' to create a new customer.")
            .block(Block::default().borders(Borders::ALL).title("Customers"))
            .alignment(Alignment::Center);
        f.render_widget(empty, area);
        return;
    }

    let items: Vec<ListItem> = app
        .customers
        .iter()
        .map(|c| {
            let name = c.name.as_deref().unwrap_or("N/A");
            let email = c.email.as_deref().unwrap_or("N/A");
            let customer_number = c
                .customer_number
                .as_ref()
                .map(|n| n.to_string())
                .unwrap_or_else(|| "N/A".to_string());

            ListItem::new(format!("[{}] {} - {}", customer_number, name, email))
        })
        .collect();

    let title = format!(
        "Customers (Page {} | ↑↓: select, ←→: page, Enter: view)",
        app.current_page
    );

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title),
        )
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(
        list,
        area,
        &mut ratatui::widgets::ListState::default().with_selected(Some(app.selected_customer)),
    );
}

fn draw_customer_form(f: &mut Frame, area: Rect, app: &App) {
    let fields = vec!["Name", "Email", "Phone", "Website (optional)"];
    let current_field = app.input_field;

    let mut text = vec![Line::from("Create New Customer"), Line::from("")];

    for (i, field) in fields.iter().enumerate() {
        let value = app.form_data.get(i).map(|s| s.as_str()).unwrap_or("");
        let line = if i == current_field && app.input_mode == InputMode::Editing {
            Line::from(vec![
                Span::styled(format!("{}: ", field), Style::default().fg(Color::Yellow)),
                Span::raw(&app.input),
                Span::styled("_", Style::default().add_modifier(Modifier::SLOW_BLINK)),
            ])
        } else {
            Line::from(format!("{}: {}", field, value))
        };
        text.push(line);
    }

    if let Some(err) = &app.error_message {
        text.push(Line::from(""));
        text.push(Line::from(Span::styled(
            err.clone(),
            Style::default().fg(Color::Red),
        )));
    }

    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Create Customer"),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(paragraph, area);
}

fn draw_customer_detail(f: &mut Frame, area: Rect, app: &App, id: &str) {
    let customer = app.customers.iter().find(|c| c.id.as_deref() == Some(id));

    let text = if let Some(c) = customer {
        vec![
            Line::from(format!(
                "ID: {}",
                c.id.as_deref().unwrap_or("N/A")
            )),
            Line::from(format!(
                "Customer Number: {}",
                c.customer_number
                    .as_ref()
                    .map(|n| n.to_string())
                    .unwrap_or_else(|| "N/A".to_string())
            )),
            Line::from(format!("Name: {}", c.name.as_deref().unwrap_or("N/A"))),
            Line::from(format!("Email: {}", c.email.as_deref().unwrap_or("N/A"))),
            Line::from(format!("Phone: {}", c.phone.as_deref().unwrap_or("N/A"))),
            Line::from(format!(
                "Website: {}",
                c.website.as_deref().unwrap_or("N/A")
            )),
            Line::from(format!(
                "Active: {}",
                c.is_active.map(|a| a.to_string()).unwrap_or_else(|| "N/A".to_string())
            )),
        ]
    } else {
        vec![Line::from("Customer not found")]
    };

    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Customer Detail (Press 'e' to edit, ESC to go back)"),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(paragraph, area);
}

fn draw_invoices(f: &mut Frame, area: Rect, app: &App) {
    if app.loading {
        let loading = Paragraph::new("Loading invoices...")
            .block(Block::default().borders(Borders::ALL).title("Invoices"))
            .alignment(Alignment::Center);
        f.render_widget(loading, area);
        return;
    }

    if app.invoices.is_empty() {
        let empty = Paragraph::new("No invoices found. Press 'n' to create a new invoice.")
            .block(Block::default().borders(Borders::ALL).title("Invoices"))
            .alignment(Alignment::Center);
        f.render_widget(empty, area);
        return;
    }

    let items: Vec<ListItem> = app
        .invoices
        .iter()
        .map(|inv| {
            let number = inv
                .invoice_number
                .as_ref()
                .map(|n| n.to_string())
                .unwrap_or_else(|| "N/A".to_string());
            let total = inv
                .total_amount_including_vat
                .map(|t| format!("{:.2}", t))
                .unwrap_or_else(|| "N/A".to_string());
            let customer_id = inv.customer_id.as_deref().unwrap_or("N/A");

            ListItem::new(format!("[{}] Customer: {} - {} SEK", number, customer_id, total))
        })
        .collect();

    let title = format!(
        "Invoices (Page {} | ↑↓: select, ←→: page, Enter: view)",
        app.current_page
    );

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title),
        )
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(
        list,
        area,
        &mut ratatui::widgets::ListState::default().with_selected(Some(app.selected_invoice)),
    );
}

fn draw_invoice_form(f: &mut Frame, area: Rect, app: &App) {
    let fields = vec!["Customer ID", "Description/Remarks", "Amount (SEK)"];
    let current_field = app.input_field;

    let mut text = vec![Line::from("Create New Invoice"), Line::from("")];

    for (i, field) in fields.iter().enumerate() {
        let value = app.form_data.get(i).map(|s| s.as_str()).unwrap_or("");
        let line = if i == current_field && app.input_mode == InputMode::Editing {
            Line::from(vec![
                Span::styled(format!("{}: ", field), Style::default().fg(Color::Yellow)),
                Span::raw(&app.input),
                Span::styled("_", Style::default().add_modifier(Modifier::SLOW_BLINK)),
            ])
        } else {
            Line::from(format!("{}: {}", field, value))
        };
        text.push(line);
    }

    if let Some(err) = &app.error_message {
        text.push(Line::from(""));
        text.push(Line::from(Span::styled(
            err.clone(),
            Style::default().fg(Color::Red),
        )));
    }

    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Create Invoice"),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(paragraph, area);
}

fn draw_invoice_detail(f: &mut Frame, area: Rect, app: &App, id: &str) {
    let invoice = app.invoices.iter().find(|inv| inv.id.as_deref() == Some(id));

    let text = if let Some(inv) = invoice {
        vec![
            Line::from(format!(
                "Invoice Number: {}",
                inv.invoice_number
                    .as_ref()
                    .map(|n| n.to_string())
                    .unwrap_or_else(|| "N/A".to_string())
            )),
            Line::from(format!(
                "Customer ID: {}",
                inv.customer_id.as_deref().unwrap_or("N/A")
            )),
            Line::from(format!(
                "Date: {}",
                inv.invoice_date
                    .map(|d| d.format("%Y-%m-%d").to_string())
                    .unwrap_or_else(|| "N/A".to_string())
            )),
            Line::from(format!(
                "Total Amount: {} SEK",
                inv.total_amount
                    .map(|t| format!("{:.2}", t))
                    .unwrap_or_else(|| "N/A".to_string())
            )),
            Line::from(format!(
                "VAT Amount: {} SEK",
                inv.total_vat_amount
                    .map(|t| format!("{:.2}", t))
                    .unwrap_or_else(|| "N/A".to_string())
            )),
            Line::from(format!(
                "Total Including VAT: {} SEK",
                inv.total_amount_including_vat
                    .map(|t| format!("{:.2}", t))
                    .unwrap_or_else(|| "N/A".to_string())
            )),
            Line::from(format!(
                "Remarks: {}",
                inv.remarks.as_deref().unwrap_or("N/A")
            )),
        ]
    } else {
        vec![Line::from("Invoice not found")]
    };

    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Invoice Detail (ESC to go back)"),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(paragraph, area);
}

fn draw_help(f: &mut Frame, area: Rect) {
    let text = vec![
        Line::from(Span::styled(
            "Keyboard Shortcuts",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("Navigation:"),
        Line::from("  Tab/Shift+Tab  - Switch between screens"),
        Line::from("  ↑/↓            - Navigate lists"),
        Line::from("  Enter          - Select/confirm"),
        Line::from("  ESC            - Go back/cancel"),
        Line::from("  q              - Quit (from main screens)"),
        Line::from(""),
        Line::from("Actions:"),
        Line::from("  n              - Create new (customer/invoice/article)"),
        Line::from("  e              - Edit selected item"),
        Line::from("  r              - Refresh current view"),
        Line::from("  d              - Go to Dashboard"),
        Line::from("  s              - Search"),
        Line::from("  h or ?         - Show this help"),
        Line::from(""),
        Line::from("Screens:"),
        Line::from("  Home           - Main menu"),
        Line::from("  Dashboard      - Statistics and quick access"),
        Line::from("  Customers      - View and manage customers"),
        Line::from("  Invoices       - View and manage invoices"),
        Line::from("  Articles       - View and manage articles/products"),
        Line::from("  Search         - Search across all entities"),
        Line::from("  Export         - Export data to JSON files"),
        Line::from("  Help           - This screen"),
        Line::from(""),
        Line::from(Span::styled(
            "Press ESC to return to the previous screen",
            Style::default().fg(Color::Yellow),
        )),
    ];

    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Help"))
        .wrap(Wrap { trim: false });

    f.render_widget(paragraph, area);
}
fn draw_dashboard(f: &mut Frame, area: Rect, app: &App) {
    let items = vec![
        ListItem::new(format!("Customers: {}", app.stats_total_customers)),
        ListItem::new(format!("Invoices: {}", app.stats_total_invoices)),
        ListItem::new(format!("Articles: {}", app.stats_total_articles)),
        ListItem::new("Export All Data"),
    ];

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Dashboard - Statistics & Quick Actions"),
        )
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(
        list,
        area,
        &mut ratatui::widgets::ListState::default().with_selected(Some(app.selected_customer)),
    );
}

fn draw_articles(f: &mut Frame, area: Rect, app: &App) {
    if app.loading {
        let loading = Paragraph::new("Loading articles...")
            .block(Block::default().borders(Borders::ALL).title("Articles"))
            .alignment(Alignment::Center);
        f.render_widget(loading, area);
        return;
    }

    if app.articles.is_empty() {
        let empty = Paragraph::new("No articles found. Press 'n' to create a new article.")
            .block(Block::default().borders(Borders::ALL).title("Articles"))
            .alignment(Alignment::Center);
        f.render_widget(empty, area);
        return;
    }

    let items: Vec<ListItem> = app
        .articles
        .iter()
        .map(|article| {
            let name = article.name.as_deref().unwrap_or("N/A");
            let price = article
                .sales_price
                .map(|p| format!("{:.2} SEK", p))
                .unwrap_or_else(|| "N/A".to_string());
            let article_number = article
                .article_number
                .as_ref()
                .map(|n| n.to_string())
                .unwrap_or_else(|| "N/A".to_string());

            ListItem::new(format!("[{}] {} - {}", article_number, name, price))
        })
        .collect();

    let title = format!(
        "Articles (Page {} | ↑↓: select, ←→: page, Enter: view)",
        app.current_page
    );

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title),
        )
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(
        list,
        area,
        &mut ratatui::widgets::ListState::default().with_selected(Some(app.selected_article)),
    );
}

fn draw_article_detail(f: &mut Frame, area: Rect, app: &App, id: &str) {
    let article = app.articles.iter().find(|a| a.id.as_deref() == Some(id));

    let text = if let Some(art) = article {
        vec![
            Line::from(format!("ID: {}", art.id.as_deref().unwrap_or("N/A"))),
            Line::from(format!(
                "Article Number: {}",
                art.article_number
                    .as_ref()
                    .map(|n| n.to_string())
                    .unwrap_or_else(|| "N/A".to_string())
            )),
            Line::from(format!("Name: {}", art.name.as_deref().unwrap_or("N/A"))),
            Line::from(format!("Unit: {}", art.unit.as_deref().unwrap_or("N/A"))),
            Line::from(format!(
                "Sales Price: {} SEK",
                art.sales_price
                    .map(|p| format!("{:.2}", p))
                    .unwrap_or_else(|| "N/A".to_string())
            )),
            Line::from(format!(
                "Purchase Price: {} SEK",
                art.purchase_price
                    .map(|p| format!("{:.2}", p))
                    .unwrap_or_else(|| "N/A".to_string())
            )),
            Line::from(format!(
                "Active: {}",
                art.is_active.map(|a| a.to_string()).unwrap_or_else(|| "N/A".to_string())
            )),
        ]
    } else {
        vec![Line::from("Article not found")]
    };

    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Article Detail (ESC to go back)"),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(paragraph, area);
}

fn draw_article_form(f: &mut Frame, area: Rect, app: &App) {
    let fields = vec!["Name", "Sales Price (SEK)"];
    let current_field = app.input_field;

    let mut text = vec![Line::from("Create New Article"), Line::from("")];

    for (i, field) in fields.iter().enumerate() {
        let value = app.form_data.get(i).map(|s| s.as_str()).unwrap_or("");
        let line = if i == current_field && app.input_mode == InputMode::Editing {
            Line::from(vec![
                Span::styled(format!("{}: ", field), Style::default().fg(Color::Yellow)),
                Span::raw(&app.input),
                Span::styled("_", Style::default().add_modifier(Modifier::SLOW_BLINK)),
            ])
        } else {
            Line::from(format!("{}: {}", field, value))
        };
        text.push(line);
    }

    if let Some(err) = &app.error_message {
        text.push(Line::from(""));
        text.push(Line::from(Span::styled(
            err.clone(),
            Style::default().fg(Color::Red),
        )));
    }

    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Create Article"),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(paragraph, area);
}

fn draw_customer_edit_form(f: &mut Frame, area: Rect, app: &App, _id: &str) {
    let fields = vec!["Name", "Email", "Phone", "Website (optional)"];
    let current_field = app.input_field;

    let mut text = vec![Line::from("Edit Customer"), Line::from("")];

    for (i, field) in fields.iter().enumerate() {
        let value = app.form_data.get(i).map(|s| s.as_str()).unwrap_or("");
        let line = if i == current_field && app.input_mode == InputMode::Editing {
            Line::from(vec![
                Span::styled(format!("{}: ", field), Style::default().fg(Color::Yellow)),
                Span::raw(&app.input),
                Span::styled("_", Style::default().add_modifier(Modifier::SLOW_BLINK)),
            ])
        } else {
            Line::from(format!("{}: {}", field, value))
        };
        text.push(line);
    }

    if let Some(err) = &app.error_message {
        text.push(Line::from(""));
        text.push(Line::from(Span::styled(
            err.clone(),
            Style::default().fg(Color::Red),
        )));
    }

    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Edit Customer"))
        .wrap(Wrap { trim: false });

    f.render_widget(paragraph, area);
}

fn draw_search(f: &mut Frame, area: Rect, app: &App) {
    let mut text = vec![
        Line::from("Search Across Customers and Invoices"),
        Line::from(""),
    ];

    // Show input field
    if app.search_input_mode {
        text.push(Line::from(vec![
            Span::styled("Query: ", Style::default().fg(Color::Yellow)),
            Span::raw(&app.input),
            Span::styled("_", Style::default().add_modifier(Modifier::SLOW_BLINK)),
        ]));
    } else {
        text.push(Line::from(format!("Query: {}", app.search_query)));
    }

    text.push(Line::from(""));
    text.push(Line::from(format!(
        "Results: {} customers, {} invoices",
        app.search_results_customers.len(),
        app.search_results_invoices.len()
    )));
    text.push(Line::from(""));

    if app.loading {
        text.push(Line::from("Searching..."));
    } else if app.search_input_mode {
        text.push(Line::from("Press Enter to search, ESC to stop typing"));
    } else {
        text.push(Line::from("Type to enter search query, Enter to search"));
    }

    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Search"))
        .wrap(Wrap { trim: false })
        .alignment(Alignment::Center);

    f.render_widget(paragraph, area);
}

fn draw_export(f: &mut Frame, area: Rect, app: &App) {
    let mut text = vec![
        Line::from("Export Data"),
        Line::from(""),
        Line::from(format!(
            "Ready to export {} customers",
            app.customers.len()
        )),
        Line::from(format!(
            "Ready to export {} invoices",
            app.invoices.len()
        )),
        Line::from(format!(
            "Ready to export {} articles",
            app.articles.len()
        )),
        Line::from(""),
        Line::from("Press Enter to export all data to JSON files"),
        Line::from(""),
    ];

    if let Some(msg) = &app.status_message {
        text.push(Line::from(""));
        text.push(Line::from(Span::styled(
            msg.clone(),
            Style::default().fg(Color::Green),
        )));
    }

    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Export"))
        .wrap(Wrap { trim: false })
        .alignment(Alignment::Center);

    f.render_widget(paragraph, area);
}
