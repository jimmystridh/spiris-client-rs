//! Context-aware help system.
//!
//! Provides screen-specific help content and keyboard shortcuts.

use crate::app::Screen;

/// Get help content for a specific screen
pub fn get_screen_help(screen: &Screen) -> ScreenHelp {
    match screen {
        Screen::Home => ScreenHelp {
            title: "Home Screen",
            description: "Main landing screen showing navigation options",
            shortcuts: vec![
                ("Tab/Shift+Tab", "Navigate between screens"),
                ("d", "Go to Dashboard"),
                ("c", "Go to Customers"),
                ("i", "Go to Invoices"),
                ("a", "Go to Articles"),
                ("s or /", "Open Search"),
                ("h or ?", "Show this Help"),
                ("q", "Quit application"),
            ],
            tips: vec![
                "Use Tab to quickly cycle through all screens",
                "Press 'r' on any list screen to refresh data",
                "All keyboard shortcuts are case-insensitive",
            ],
        },
        Screen::Dashboard => ScreenHelp {
            title: "Dashboard",
            description: "Overview of key statistics and recent activity",
            shortcuts: vec![
                ("r", "Refresh statistics"),
                ("c", "Go to Customers"),
                ("i", "Go to Invoices"),
                ("a", "Go to Articles"),
                ("Esc", "Back to Home"),
            ],
            tips: vec![
                "Dashboard auto-refreshes if configured in settings",
                "Statistics are calculated from loaded data",
                "Recent activity shows last 7 and 30 days",
            ],
        },
        Screen::Customers => ScreenHelp {
            title: "Customers List",
            description: "View and manage all customers",
            shortcuts: vec![
                ("↑/↓", "Navigate list"),
                ("←/→", "Previous/Next page"),
                ("Enter", "View customer details"),
                ("n", "Create new customer"),
                ("o", "Cycle sort options"),
                ("r", "Refresh customer list"),
                ("b", "Toggle batch selection mode"),
                ("f", "Toggle filter panel"),
                ("Space", "Select/deselect (in batch mode)"),
            ],
            tips: vec![
                "Batch mode allows multi-select for bulk operations",
                "Filters persist within the current session",
                "Sort by Name, Email, or Customer Number",
            ],
        },
        Screen::Invoices => ScreenHelp {
            title: "Invoices List",
            description: "View and manage all invoices",
            shortcuts: vec![
                ("↑/↓", "Navigate list"),
                ("←/→", "Previous/Next page"),
                ("Enter", "View invoice details"),
                ("n", "Create new invoice"),
                ("o", "Cycle sort options"),
                ("r", "Refresh invoice list"),
                ("b", "Toggle batch selection mode"),
            ],
            tips: vec![
                "Invoices can be sorted by number, date, or amount",
                "Use batch mode to select multiple invoices",
                "Export functionality available from Export screen",
            ],
        },
        Screen::Articles => ScreenHelp {
            title: "Articles List",
            description: "View and manage all articles/products",
            shortcuts: vec![
                ("↑/↓", "Navigate list"),
                ("←/→", "Previous/Next page"),
                ("Enter", "View article details"),
                ("n", "Create new article"),
                ("o", "Cycle sort options"),
                ("r", "Refresh article list"),
                ("b", "Toggle batch selection mode"),
                ("f", "Toggle filter panel"),
            ],
            tips: vec![
                "Articles represent products or services you sell",
                "Set sales price and purchase price for margin tracking",
                "Inactive articles are hidden by default in forms",
            ],
        },
        Screen::Search => ScreenHelp {
            title: "Search",
            description: "Search across customers and invoices",
            shortcuts: vec![
                ("Type", "Enter search query"),
                ("Enter", "Execute search"),
                ("m", "Cycle search mode (All/Customers/Invoices)"),
                ("Esc", "Clear search / Go back"),
            ],
            tips: vec![
                "Search is case-insensitive",
                "Results update as you type",
                "Use 'm' to search only customers or invoices",
            ],
        },
        Screen::Export => ScreenHelp {
            title: "Export Data",
            description: "Export data to CSV or JSON format",
            shortcuts: vec![
                ("↑/↓", "Select export format"),
                ("Enter", "Execute export"),
                ("Esc", "Cancel and go back"),
            ],
            tips: vec![
                "Default format can be set in config file",
                "Exports include all loaded data",
                "Files are timestamped automatically",
                "Export directory configurable in settings",
            ],
        },
        Screen::CustomerDetail(_) => ScreenHelp {
            title: "Customer Details",
            description: "View detailed information for a customer",
            shortcuts: vec![
                ("e", "Edit customer"),
                ("x", "Delete customer (with confirmation)"),
                ("Esc", "Back to customers list"),
            ],
            tips: vec![
                "Delete requires confirmation to prevent accidents",
                "Changes sync with the API immediately",
            ],
        },
        Screen::InvoiceDetail(_) => ScreenHelp {
            title: "Invoice Details",
            description: "View detailed information for an invoice",
            shortcuts: vec![
                ("e", "Edit invoice"),
                ("x", "Delete invoice (with confirmation)"),
                ("Esc", "Back to invoices list"),
            ],
            tips: vec![
                "Invoice rows are displayed with full details",
                "Total amounts include VAT calculations",
            ],
        },
        Screen::ArticleDetail(_) => ScreenHelp {
            title: "Article Details",
            description: "View detailed information for an article",
            shortcuts: vec![
                ("e", "Edit article"),
                ("x", "Delete article (with confirmation)"),
                ("Esc", "Back to articles list"),
            ],
            tips: vec![
                "Active status controls visibility in forms",
                "Price changes apply to future transactions only",
            ],
        },
        Screen::CustomerCreate | Screen::CustomerEdit(_) => ScreenHelp {
            title: "Customer Form",
            description: "Create or edit customer information",
            shortcuts: vec![
                ("Tab", "Next field"),
                ("Shift+Tab", "Previous field"),
                ("Enter", "Submit form"),
                ("Esc", "Cancel and go back"),
            ],
            tips: vec![
                "Email validation is performed automatically",
                "Website field is optional",
                "All changes require form submission",
            ],
        },
        Screen::InvoiceCreate | Screen::InvoiceEdit(_) => ScreenHelp {
            title: "Invoice Form",
            description: "Create or edit invoice",
            shortcuts: vec![
                ("Tab", "Next field"),
                ("Shift+Tab", "Previous field"),
                ("Enter", "Submit form"),
                ("Esc", "Cancel and go back"),
            ],
            tips: vec![
                "Customer ID must match an existing customer",
                "Amount validation ensures positive values",
                "Remarks field is optional",
            ],
        },
        Screen::ArticleCreate | Screen::ArticleEdit(_) => ScreenHelp {
            title: "Article Form",
            description: "Create or edit article/product",
            shortcuts: vec![
                ("Tab", "Next field"),
                ("Shift+Tab", "Previous field"),
                ("Enter", "Submit form"),
                ("Esc", "Cancel and go back"),
            ],
            tips: vec![
                "Name is required",
                "Price must be a positive number",
                "Use descriptive names for better organization",
            ],
        },
        Screen::Help => ScreenHelp {
            title: "Help & Keyboard Shortcuts",
            description: "Comprehensive help and shortcut reference",
            shortcuts: vec![
                ("Esc", "Close help"),
                ("↑/↓", "Scroll help text"),
            ],
            tips: vec![
                "Context-specific help available on each screen",
                "Press 'h' or '?' from any screen for help",
                "Configuration file: ~/.config/spiris-tui/config.toml",
            ],
        },
        Screen::Auth => ScreenHelp {
            title: "Authentication",
            description: "OAuth2 authentication flow",
            shortcuts: vec![
                ("Enter", "Start OAuth flow"),
                ("Esc", "Cancel"),
            ],
            tips: vec![
                "Requires SPIRIS_CLIENT_ID environment variable",
                "Token is saved locally for future sessions",
                "Open the provided URL in your browser",
            ],
        },
    }
}

/// Help content for a specific screen
pub struct ScreenHelp {
    pub title: &'static str,
    pub description: &'static str,
    pub shortcuts: Vec<(&'static str, &'static str)>,
    pub tips: Vec<&'static str>,
}

/// Get keyboard shortcuts for current screen (for status bar)
pub fn get_context_shortcuts(screen: &Screen, batch_mode: bool) -> Vec<String> {
    let mut shortcuts = Vec::new();

    // Common shortcuts
    shortcuts.push("q:Quit".to_string());
    shortcuts.push("h:Help".to_string());

    // Screen-specific shortcuts
    match screen {
        Screen::Home => {
            shortcuts.push("d:Dashboard".to_string());
            shortcuts.push("c:Customers".to_string());
            shortcuts.push("i:Invoices".to_string());
        }
        Screen::Customers | Screen::Invoices | Screen::Articles => {
            if batch_mode {
                shortcuts.push("Space:Select".to_string());
                shortcuts.push("b:Exit Batch".to_string());
            } else {
                shortcuts.push("n:New".to_string());
                shortcuts.push("b:Batch".to_string());
                shortcuts.push("f:Filter".to_string());
            }
            shortcuts.push("Enter:View".to_string());
        }
        Screen::CustomerDetail(_) | Screen::InvoiceDetail(_) | Screen::ArticleDetail(_) => {
            shortcuts.push("e:Edit".to_string());
            shortcuts.push("x:Delete".to_string());
            shortcuts.push("Esc:Back".to_string());
        }
        Screen::Search => {
            shortcuts.push("m:Mode".to_string());
            shortcuts.push("Enter:Search".to_string());
        }
        Screen::Export => {
            shortcuts.push("Enter:Export".to_string());
            shortcuts.push("Esc:Cancel".to_string());
        }
        _ => {
            shortcuts.push("Esc:Back".to_string());
        }
    }

    shortcuts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_screen_help() {
        let help = get_screen_help(&Screen::Home);
        assert_eq!(help.title, "Home Screen");
        assert!(!help.shortcuts.is_empty());
        assert!(!help.tips.is_empty());
    }

    #[test]
    fn test_get_context_shortcuts() {
        let shortcuts = get_context_shortcuts(&Screen::Home, false);
        assert!(shortcuts.len() > 0);
        assert!(shortcuts.iter().any(|s| s.contains("Quit")));
    }

    #[test]
    fn test_batch_mode_shortcuts() {
        let normal = get_context_shortcuts(&Screen::Customers, false);
        let batch = get_context_shortcuts(&Screen::Customers, true);

        // Batch mode should have different shortcuts
        assert_ne!(normal, batch);
        assert!(batch.iter().any(|s| s.contains("Select")));
    }
}
