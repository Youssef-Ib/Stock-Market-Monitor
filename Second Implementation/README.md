
# ECE-522-Project-1

## ECE 522 Project 1: Stock Market Monitor

This project monitors the stock prices for a given stock ticker over the last 6 months. The program fetches stock data, performs financial analysis (such as Simple Moving Average and volatility detection), and generates a chart showing the stock's daily closing prices along with the analysis.

### Crates Used:

- **clap**: For parsing command-line arguments, including the stock ticker symbol.
- **yahoo_finance_api**: To fetch historical stock prices from Yahoo Finance.
- **tokio**: For asynchronous operations, specifically used to handle async API calls.
- **plotters**: To generate the graphical output (a line plot of the stock's closing prices, with volatility indicated using error bars and the 20-day SMA.
- **time**: For handling and formatting timestamps and dates.

### Financial Analysis Algorithm:

1. **Volatility Detection**: The program identifies "volatile" days where the difference between the stock's intra-day high and low prices exceeds 2% of the closing price. These volatile days are highlighted using error bars on the chart.
2. **20-Day Simple Moving Average**: The program calculates the 20-day SMA of the stock's closing prices and overlays it on the same chart as the closing prices. The SMA helps visualize the overall trend of the stock over time.

### Charting Setup:

- **Line Plot**: The daily closing prices are displayed as a line plot.
- **Error Bars**: Volatile days are highlighted using vertical error bars that show the low, close, and high prices for those days.
- **20-Day SMA**: A green line represents the 20-day Simple Moving Average to show the stock's recent trend.
- **Autoscaling**: The y-axis is scaled to fit the minimum and maximum prices from the stock data.
- **X-axis**: Dates are plotted at regular intervals (approximately one label per month).

### **Usage Instructions**:

- Add the following lines to Cargo.toml file:

  - yahoo_finance_api = "2.3.0"
  - tokio = { version = "1.41.0", features = ["full"] }
  - clap = { version = "4.5.19", features = ["derive"] }
  - plotters = "0.3.7"
  - time = "0.3.36"
- Additionally, before running the program, execute the following line(s):
  cargo add clap --features derive
- Execution options:

  - Help: cargo run -- --help
  - Enter user input using: cargo run -- -t "Name of Stock"
    - Sample usage: cargo run -- -t "AAPL"
