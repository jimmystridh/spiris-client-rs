# Spiris Bokföring och Fakturering - Terminal UI

A beautiful and efficient Terminal User Interface (TUI) for managing customers and invoices using the Spiris Bokföring och Fakturering API (formerly Visma eAccounting).

![Spiris TUI Demo](https://img.shields.io/badge/status-beta-yellow)
![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)

## Features

- 📊 **Interactive Dashboard** - Quick access to all features from the main menu
- 👥 **Customer Management** - List, view, create, and search customers
- 🧾 **Invoice Management** - Browse and view invoices
- 🔐 **OAuth2 Authentication** - Secure authentication with automatic token handling
- ⌨️ **Keyboard Navigation** - Fully keyboard-driven interface
- 💾 **Persistent Sessions** - Automatic token storage for seamless usage
- 🎨 **Clean UI** - Beautiful terminal interface built with Ratatui

## Screenshots

### Main Menu
```
┌──────────────────────────────────────────────────────┐
│      Spiris Bokföring och Fakturering - TUI          │
└──────────────────────────────────────────────────────┘
┌──────────────────────────────────────────────────────┐
│ Main Menu                                            │
│                                                      │
│  >> View Customers                                   │
│     View Invoices                                    │
│     Create Customer                                  │
│     Create Invoice                                   │
│     Help                                             │
└──────────────────────────────────────────────────────┘
```

## Installation

### Prerequisites

- Rust 1.70 or later
- Spiris Bokföring och Fakturering API credentials (Client ID & Secret)
- Terminal emulator with Unicode support

### Build from Source

```bash
# Clone the repository
git clone https://github.com/jimmystridh/claude_jungle_bamboo
cd claude_jungle_bamboo/spiris-tui

# Build the application
cargo build --release

# Run the TUI
cargo run --release
```

## Configuration

### OAuth2 Credentials

The TUI requires OAuth2 credentials to authenticate with the Spiris API. Set the following environment variables:

```bash
export SPIRIS_CLIENT_ID="your_client_id"
export SPIRIS_CLIENT_SECRET="your_client_secret"
```

You can obtain these credentials from the [Visma Developer Portal](https://developer.visma.com/).

### Token Storage

After successful authentication, your access token is automatically saved to `.spiris_token.json` in the current directory. This allows you to resume your session without re-authenticating.

**Security Note:** Keep this file secure and never commit it to version control!

## Usage

### Launching the TUI

```bash
# From the spiris-tui directory
cargo run --release

# Or if you've built the binary
./target/release/spiris-tui
```

### Keyboard Shortcuts

#### Global Navigation

| Key | Action |
|-----|--------|
| `Tab` | Next screen |
| `Shift+Tab` | Previous screen |
| `↑` / `↓` | Navigate lists |
| `Enter` | Select / Confirm |
| `Esc` | Go back / Cancel |
| `q` | Quit (from main screens) |

#### Context-Specific Actions

| Key | Action | Available In |
|-----|--------|--------------|
| `n` | Create new | Customers, Invoices |
| `r` | Refresh current view | Customers, Invoices |
| `h` or `?` | Show help | Any screen |

### Screens

#### 1. Authentication Screen

If no valid token is found, you'll be presented with the authentication screen:

1. Press `Enter` to start the OAuth2 flow
2. Copy the authorization URL displayed
3. Open it in your browser
4. Authorize the application
5. The token will be automatically saved

#### 2. Home Screen

The main menu provides quick access to:
- View Customers
- View Invoices
- Create Customer
- Create Invoice
- Help

Use `↑`/`↓` to navigate and `Enter` to select.

#### 3. Customers Screen

- Browse all customers with pagination
- View customer details (number, name, email, phone)
- Press `Enter` to view full customer details
- Press `n` to create a new customer
- Press `r` to refresh the customer list

#### 4. Customer Creation

Fill in the form fields:
1. Name (required)
2. Email (required)
3. Phone (required)
4. Website (optional)

Press `Enter` after each field. The customer is created automatically after the last field.

#### 5. Invoices Screen

- Browse all invoices with pagination
- View invoice number, customer, and total amount
- Press `Enter` to view full invoice details
- Press `r` to refresh the invoice list

#### 6. Invoice Detail View

View complete invoice information:
- Invoice number
- Customer ID
- Invoice date
- Total amount
- VAT amount
- Total including VAT
- Remarks

Press `Esc` to return to the invoice list.

#### 7. Help Screen

Press `h` or `?` from any screen to view the help page with all keyboard shortcuts.

## Features in Detail

### Automatic Token Refresh

The TUI checks if your access token is expired before each API request. If the token is expired and you have a refresh token, you'll need to re-authenticate through the OAuth2 flow.

### Error Handling

- **Network errors**: Displayed at the bottom of the screen
- **Authentication errors**: Redirects to the auth screen
- **API errors**: Shown with context-specific messages

### Loading States

When fetching data from the API, a loading indicator is displayed:
```
Loading customers...
```

### Empty States

When no data is available, helpful messages guide you:
```
No customers found. Press 'n' to create a new customer.
```

## Development

### Project Structure

```
spiris-tui/
├── src/
│   ├── main.rs          # Application entry point
│   ├── app.rs           # Application state and logic
│   ├── ui.rs            # UI rendering
│   ├── auth.rs          # OAuth2 authentication helpers
│   └── screens/         # Screen-specific modules (future)
├── Cargo.toml           # Dependencies
└── README.md           # This file
```

### Running in Development

```bash
cargo run
```

### Building for Release

```bash
cargo build --release
```

The optimized binary will be at `target/release/spiris-tui`.

## Troubleshooting

### Token Expired Errors

If you see authentication errors:

1. Delete `.spiris_token.json`
2. Restart the TUI
3. Complete the OAuth2 flow again

### Display Issues

If you experience display issues:

1. Ensure your terminal supports Unicode
2. Try resizing the terminal window
3. Check that your terminal emulator is up to date

### API Connection Errors

If you can't connect to the API:

1. Check your internet connection
2. Verify your OAuth2 credentials are correct
3. Ensure the Spiris API is accessible

### Missing Customers/Invoices

If data doesn't appear:

1. Press `r` to refresh
2. Check that your account has data
3. Verify API permissions in the developer portal

## Known Limitations

- **Invoice Creation**: Currently under development
- **Pagination**: Limited to first 50 results per endpoint
- **Search**: Not yet implemented
- **Filtering**: Not yet available
- **Editing**: Customers and invoices cannot be edited (view-only)

## Roadmap

- [ ] Complete invoice creation form
- [ ] Implement customer editing
- [ ] Add search and filtering
- [ ] Improve pagination (load more pages)
- [ ] Add articles/products management
- [ ] Export functionality (CSV, PDF)
- [ ] Multi-account support
- [ ] Keyboard shortcut customization

## Contributing

Contributions are welcome! Please see the main [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines.

### Areas for Contribution

- UI/UX improvements
- Additional features (editing, search, etc.)
- Bug fixes
- Documentation
- Testing

## Dependencies

- **ratatui** - Terminal UI framework
- **crossterm** - Terminal manipulation library
- **tokio** - Async runtime
- **spiris_bokforing** - Spiris API client library
- **anyhow** - Error handling
- **serde/serde_json** - Serialization
- **chrono** - Date/time handling

## License

This project is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](../LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Support

For issues, questions, or feature requests, please open an issue on GitHub.

## Acknowledgments

- Built with [Ratatui](https://ratatui.rs/) - A Rust library for building rich terminal user interfaces
- Uses the [Spiris Bokföring och Fakturering API](https://developer.visma.com/api/eaccounting)
- Inspired by modern TUI applications like [lazygit](https://github.com/jesseduffield/lazygit) and [k9s](https://k9scli.io/)
