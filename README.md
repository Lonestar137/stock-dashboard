
# Architecture

```
stock-analysis-tool/
├── src/
│   ├── components/        # Reusable UI components
│   │   ├── header.rs
│   │   ├── footer.rs
│   │   └── stock_card.rs  # For individual stock information
│   ├── pages/             # Pages for different views
│   │   ├── dashboard.rs   # Main page with charts, filters, etc.
│   │   ├── stock_details.rs # Detailed page for a specific stock
│   │   └── settings.rs    # User settings/preferences
│   ├── services/          # Fetching data from APIs, WebSocket clients, etc.
│   │   └── api.rs
│   ├── main.rs            # Yew App entry point
└── └── utils.rs           # Utility functions for formatting, etc.
```
