use anyhow::Result;
use spiris_bokforing::{AccessToken, Client, Customer, Invoice, PaginationParams};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    Home,
    Auth,
    Customers,
    CustomerCreate,
    CustomerDetail(String),
    Invoices,
    InvoiceCreate,
    InvoiceDetail(String),
    Help,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputMode {
    Normal,
    Editing,
}

pub struct App {
    pub screen: Screen,
    pub previous_screen: Option<Screen>,
    pub input_mode: InputMode,
    pub client: Option<Client>,
    pub token: Option<AccessToken>,

    // Screen state
    pub customers: Vec<Customer>,
    pub selected_customer: usize,
    pub invoices: Vec<Invoice>,
    pub selected_invoice: usize,

    // Form inputs
    pub input: String,
    pub input_field: usize,
    pub form_data: Vec<String>,

    // Status/error messages
    pub status_message: Option<String>,
    pub error_message: Option<String>,

    // Loading state
    pub loading: bool,

    // OAuth state
    pub oauth_url: Option<String>,
    pub oauth_waiting: bool,
}

impl App {
    pub fn new() -> Self {
        // Try to load token from file
        let token = Self::load_token().ok();
        let client = token.as_ref().map(|t| Client::new(t.clone()));

        let screen = if token.is_some() {
            Screen::Home
        } else {
            Screen::Auth
        };

        Self {
            screen,
            previous_screen: None,
            input_mode: InputMode::Normal,
            client,
            token,
            customers: Vec::new(),
            selected_customer: 0,
            invoices: Vec::new(),
            selected_invoice: 0,
            input: String::new(),
            input_field: 0,
            form_data: Vec::new(),
            status_message: None,
            error_message: None,
            loading: false,
            oauth_url: None,
            oauth_waiting: false,
        }
    }

    pub fn can_quit(&self) -> bool {
        self.input_mode == InputMode::Normal
    }

    pub fn handle_escape(&mut self) {
        if self.input_mode == InputMode::Editing {
            self.input_mode = InputMode::Normal;
            self.input.clear();
        } else if let Some(prev) = self.previous_screen.take() {
            self.screen = prev;
            self.error_message = None;
        } else {
            self.screen = Screen::Home;
        }
    }

    pub async fn handle_enter(&mut self) -> Result<()> {
        if self.input_mode == InputMode::Editing {
            self.form_data.push(self.input.clone());
            self.input.clear();
            self.input_field += 1;

            // Check if form is complete
            if self.should_submit_form() {
                self.submit_form().await?;
                self.input_mode = InputMode::Normal;
            }
        } else {
            match &self.screen {
                Screen::Home => self.handle_home_enter(),
                Screen::Customers => {
                    if !self.customers.is_empty() {
                        let customer = &self.customers[self.selected_customer];
                        if let Some(id) = &customer.id {
                            self.previous_screen = Some(Screen::Customers);
                            self.screen = Screen::CustomerDetail(id.clone());
                        }
                    }
                }
                Screen::Invoices => {
                    if !self.invoices.is_empty() {
                        let invoice = &self.invoices[self.selected_invoice];
                        if let Some(id) = &invoice.id {
                            self.previous_screen = Some(Screen::Invoices);
                            self.screen = Screen::InvoiceDetail(id.clone());
                        }
                    }
                }
                Screen::Auth => {
                    if !self.oauth_waiting {
                        self.start_oauth().await?;
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn handle_home_enter(&mut self) {
        match self.selected_customer {
            0 => self.screen = Screen::Customers,
            1 => self.screen = Screen::Invoices,
            2 => self.screen = Screen::CustomerCreate,
            3 => self.screen = Screen::InvoiceCreate,
            4 => self.screen = Screen::Help,
            _ => {}
        }
    }

    pub fn handle_up(&mut self) {
        match self.screen {
            Screen::Customers if !self.customers.is_empty() => {
                if self.selected_customer > 0 {
                    self.selected_customer -= 1;
                }
            }
            Screen::Invoices if !self.invoices.is_empty() => {
                if self.selected_invoice > 0 {
                    self.selected_invoice -= 1;
                }
            }
            Screen::Home => {
                if self.selected_customer > 0 {
                    self.selected_customer -= 1;
                }
            }
            _ => {}
        }
    }

    pub fn handle_down(&mut self) {
        match self.screen {
            Screen::Customers if !self.customers.is_empty() => {
                if self.selected_customer < self.customers.len() - 1 {
                    self.selected_customer += 1;
                }
            }
            Screen::Invoices if !self.invoices.is_empty() => {
                if self.selected_invoice < self.invoices.len() - 1 {
                    self.selected_invoice += 1;
                }
            }
            Screen::Home => {
                if self.selected_customer < 4 {
                    self.selected_customer += 1;
                }
            }
            _ => {}
        }
    }

    pub fn handle_left(&mut self) {
        // Could be used for pagination
    }

    pub fn handle_right(&mut self) {
        // Could be used for pagination
    }

    pub fn handle_char(&mut self, c: char) {
        if self.input_mode == InputMode::Editing {
            self.input.push(c);
        } else {
            match c {
                'r' => {
                    if self.client.is_some() {
                        self.refresh_current_screen();
                    }
                }
                'n' => {
                    match self.screen {
                        Screen::Customers => {
                            self.previous_screen = Some(Screen::Customers);
                            self.screen = Screen::CustomerCreate;
                            self.start_form();
                        }
                        Screen::Invoices => {
                            self.previous_screen = Some(Screen::Invoices);
                            self.screen = Screen::InvoiceCreate;
                            self.start_form();
                        }
                        _ => {}
                    }
                }
                'h' | '?' => self.screen = Screen::Help,
                _ => {}
            }
        }
    }

    pub fn handle_backspace(&mut self) {
        if self.input_mode == InputMode::Editing {
            self.input.pop();
        }
    }

    pub fn next_screen(&mut self) {
        if self.client.is_some() {
            self.screen = match &self.screen {
                Screen::Home => Screen::Customers,
                Screen::Customers => Screen::Invoices,
                Screen::Invoices => Screen::Help,
                Screen::Help => Screen::Home,
                _ => return,
            };
        }
    }

    pub fn previous_screen(&mut self) {
        if self.client.is_some() {
            self.screen = match &self.screen {
                Screen::Home => Screen::Help,
                Screen::Customers => Screen::Home,
                Screen::Invoices => Screen::Customers,
                Screen::Help => Screen::Invoices,
                _ => return,
            };
        }
    }

    fn start_form(&mut self) {
        self.input_mode = InputMode::Editing;
        self.input.clear();
        self.form_data.clear();
        self.input_field = 0;
    }

    fn should_submit_form(&self) -> bool {
        match self.screen {
            Screen::CustomerCreate => self.input_field >= 4, // name, email, phone, website
            Screen::InvoiceCreate => self.input_field >= 3,   // customer_id, description, amount
            _ => false,
        }
    }

    async fn submit_form(&mut self) -> Result<()> {
        if let Some(client) = &self.client {
            match self.screen {
                Screen::CustomerCreate => {
                    let customer = Customer {
                        name: Some(self.form_data[0].clone()),
                        email: Some(self.form_data[1].clone()),
                        phone: Some(self.form_data[2].clone()),
                        website: if self.form_data[3].is_empty() {
                            None
                        } else {
                            Some(self.form_data[3].clone())
                        },
                        is_active: Some(true),
                        ..Default::default()
                    };

                    match client.customers().create(&customer).await {
                        Ok(_) => {
                            self.status_message = Some("Customer created successfully".to_string());
                            self.screen = Screen::Customers;
                            self.load_customers().await?;
                        }
                        Err(e) => {
                            self.error_message = Some(format!("Failed to create customer: {}", e));
                        }
                    }
                }
                _ => {}
            }
            self.form_data.clear();
            self.input_field = 0;
        }
        Ok(())
    }

    pub async fn load_customers(&mut self) -> Result<()> {
        if let Some(client) = &self.client {
            self.loading = true;
            let params = PaginationParams::new().pagesize(50);
            match client.customers().list(Some(params)).await {
                Ok(response) => {
                    self.customers = response.data;
                    self.loading = false;
                    self.error_message = None;
                }
                Err(e) => {
                    self.error_message = Some(format!("Failed to load customers: {}", e));
                    self.loading = false;
                }
            }
        }
        Ok(())
    }

    pub async fn load_invoices(&mut self) -> Result<()> {
        if let Some(client) = &self.client {
            self.loading = true;
            let params = PaginationParams::new().pagesize(50);
            match client.invoices().list(Some(params)).await {
                Ok(response) => {
                    self.invoices = response.data;
                    self.loading = false;
                    self.error_message = None;
                }
                Err(e) => {
                    self.error_message = Some(format!("Failed to load invoices: {}", e));
                    self.loading = false;
                }
            }
        }
        Ok(())
    }

    fn refresh_current_screen(&mut self) {
        match self.screen {
            Screen::Customers => {
                let app = self.clone();
                tokio::spawn(async move {
                    let mut app = app;
                    let _ = app.load_customers().await;
                });
            }
            Screen::Invoices => {
                let app = self.clone();
                tokio::spawn(async move {
                    let mut app = app;
                    let _ = app.load_invoices().await;
                });
            }
            _ => {}
        }
    }

    async fn start_oauth(&mut self) -> Result<()> {
        self.oauth_waiting = true;
        self.status_message = Some("Starting OAuth flow...".to_string());

        // Get credentials from environment
        let client_id = std::env::var("SPIRIS_CLIENT_ID")
            .unwrap_or_else(|_| "your_client_id".to_string());
        let client_secret = std::env::var("SPIRIS_CLIENT_SECRET")
            .unwrap_or_else(|_| "your_client_secret".to_string());

        let oauth_config = spiris_bokforing::auth::OAuth2Config::new(
            client_id,
            client_secret,
            "http://localhost:8080/callback".to_string(),
        );

        let handler = spiris_bokforing::auth::OAuth2Handler::new(oauth_config)?;
        let (auth_url, _csrf, _verifier) = handler.authorize_url();

        self.oauth_url = Some(auth_url);
        self.status_message = Some("Copy the URL above and open in browser".to_string());

        Ok(())
    }

    fn load_token() -> Result<AccessToken> {
        let token_path = Self::token_path();
        let contents = std::fs::read_to_string(token_path)?;
        let token: AccessToken = serde_json::from_str(&contents)?;
        Ok(token)
    }

    pub fn save_token(&self) -> Result<()> {
        if let Some(token) = &self.token {
            let token_path = Self::token_path();
            let json = serde_json::to_string_pretty(token)?;
            std::fs::write(token_path, json)?;
        }
        Ok(())
    }

    fn token_path() -> PathBuf {
        let mut path = std::env::current_dir().unwrap();
        path.push(".spiris_token.json");
        path
    }
}

impl Clone for App {
    fn clone(&self) -> Self {
        Self {
            screen: self.screen.clone(),
            previous_screen: self.previous_screen.clone(),
            input_mode: self.input_mode.clone(),
            client: self.client.as_ref().map(|c| Client::new(c.get_access_token().clone())),
            token: self.token.clone(),
            customers: self.customers.clone(),
            selected_customer: self.selected_customer,
            invoices: self.invoices.clone(),
            selected_invoice: self.selected_invoice,
            input: self.input.clone(),
            input_field: self.input_field,
            form_data: self.form_data.clone(),
            status_message: self.status_message.clone(),
            error_message: self.error_message.clone(),
            loading: self.loading,
            oauth_url: self.oauth_url.clone(),
            oauth_waiting: self.oauth_waiting,
        }
    }
}
