# LLM Token Cost Dashboard

A web-based dashboard built with Leptos (Rust + WebAssembly) to estimate token usage and costs for various Large Language Models (LLMs).

## Features

- **Model Selection**: Choose from a comprehensive list of LLM models with search functionality
- **Accurate Token Counting**: Uses tiktoken-rs for precise token estimation matching OpenAI's tokenizer
- **Cost Calculation**: Real-time cost estimation based on input/output tokens
- **Cost Projections**: View projected costs for 100, 1K, 10K, and 100K requests
- **Model Price List**: Browse and compare pricing for all available models in a sortable table
- **Info Page**: View application version, framework details, and data source credits
- **Modern UI**: Beautiful, responsive interface built with Thaw UI components
- **Live Pricing**: Fetches latest model pricing from LiteLLM repository
- **Fast & Lightweight**: Compiled to WebAssembly for optimal performance


## Prerequisites

- Rust (latest stable version)
- `trunk` for building and serving the WebAssembly app
- `wasm32-unknown-unknown` target
- Docker (optional, for containerized deployment)

## Setup

1. Install the required Rust target:
```bash
rustup target add wasm32-unknown-unknown
```

2. Install Trunk (the build tool for Rust WASM applications):
```bash
cargo install trunk
```

## Running the Application

1. Start the development server:
```bash
trunk serve --open
```

This will:
- Build the application in debug mode
- Start a local development server (usually at `http://127.0.0.1:8080`)
- Automatically open the application in your default browser
- Watch for file changes and rebuild automatically

## Building for Production

To create an optimized production build:

```bash
trunk build --release
```

The built files will be in the `dist/` directory.

## Docker Deployment

### Build Docker Image

```bash
docker build -t llm-token-dashboard .
```

### Run with Docker

```bash
docker run -p 8080:80 llm-token-dashboard
```

Access the application at `http://localhost:8080`

### Pull from GitHub Container Registry

```bash
docker pull ghcr.io/vincentzhangz/llm-token-dashboard:latest
docker run -p 8080:80 ghcr.io/vincentzhangz/llm-token-dashboard:latest
```

## How to Use

1. **Select a Model**: Choose your desired LLM model from the dropdown menu
2. **Enter Input Text**: Type or paste your prompt in the "Input Text" area
3. **Enter Expected Output**: Add the expected completion/output text
4. **View Results**: The dashboard will automatically calculate:
   - Token counts for input and output
   - Individual costs for input and output tokens
   - Total cost per request
   - Projected costs for multiple requests (100, 1K, 10K, 100K)


## Project Structure

```
src/
├── main.rs              # App component and routing setup
├── models.rs            # Data models for pricing information
├── pages/               # Page components
│   ├── home.rs          # Cost calculator page
│   ├── model_price_list.rs  # Model pricing table page
│   ├── info.rs          # Info page with version details
│   └── mod.rs           # Page module exports
└── utils/               # Utility functions
    ├── pricing.rs       # Fetch model prices from API
    ├── tokens.rs        # Token estimation logic
    └── mod.rs           # Utils module exports
```

## Token Estimation

The dashboard uses [tiktoken-rs](https://github.com/zurawiki/tiktoken-rs) for accurate token counting. It uses the `cl100k_base` tokenizer.
This ensures accurate token counts that match what you would see when using these models via their APIs.

## Pricing Data

Model pricing is fetched from the [LiteLLM](https://github.com/BerriAI/litellm) repository:
```
https://raw.githubusercontent.com/BerriAI/litellm/main/model_prices_and_context_window.json
```

The pricing data is loaded when the application starts and includes:
- Input cost per token
- Output cost per token
- Maximum token limits
- Context window sizes
- Provider information
- Model capabilities

### Credits

Special thanks to the [BerriAI](https://github.com/BerriAI) team and contributors for maintaining the comprehensive LiteLLM pricing database.

## Development

### Build Script

The project uses a `build.rs` script to automatically extract dependency versions from `Cargo.toml` at compile time, ensuring version information displayed in the Info page is always accurate.

### Code Organization

- **Pages**: Each page is a separate component in the `src/pages/` directory
- **Utils**: Reusable functions are organized in the `src/utils/` directory
- **Models**: Data structures are defined in `src/models.rs`
- **Routing**: Managed by Leptos Router in `src/main.rs`

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT
