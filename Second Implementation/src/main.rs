use yahoo_finance_api::{self as yahoo};
use std::time::{Duration, UNIX_EPOCH};
use time::OffsetDateTime;
use time::macros::format_description;
use yahoo_finance_api::Quote;
use plotters::prelude::*;
mod cli_input;

const EXE_START: &str = "\n~~~~~~~~~~ Team 6 - Project 1 Execution Start ~~~~~~~~~~";
const EXE_END: &str = "\n~~~~~~~~~~~ Team 6 - Project 1 Execution End ~~~~~~~~~~~";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", EXE_START);

    // 1. Parse command-line arguments
    let args = cli_input::get_args();

    // 2. Fetch stock quotes for the last 6 months (weekdays)
    println!("\nFetching quotes for stock ticker: {} ...", args.ticker);
    let stock_quotes = get_stock_quotes(&args.ticker).await;
    
    if stock_quotes.is_empty() {
        println!("No stock quotes available for ticker: {}", args.ticker);
        println!("{}", EXE_END);
        return Ok(());
    }

    // 3. Print the min & max closing prices for last 6 months (weekdays)
    print_min_max(&stock_quotes);

    // 4. Plot stock quotes, volatile days and SMA
    println!("\nPlotting chart for stock ticker: {} ...", args.ticker);
    plot_stock_quotes(&args.ticker, stock_quotes).expect("Error");

    println!("{}", EXE_END);
    return Ok(());
}

// Function to fetch stock quotes
async fn get_stock_quotes(symbol: &str) -> Vec<Quote> {
    let provider = yahoo::YahooConnector::new().unwrap();
    let mut stock_quotes = vec![];
    
    // Fetch stock quotes for the past 6 months, 1 day intervals
    match provider.get_quote_range(symbol, "1d", "6mo").await {
        Ok(response) => {
            stock_quotes = response.quotes().unwrap();
            return stock_quotes;
        } Err(_e) => {
            return stock_quotes; // Return empty vector on failure
        }
    }
}

// Function to print the min & max closing prices over the interval
fn print_min_max(stock_quotes: &Vec<Quote>) {
    let min_quote = stock_quotes
        .iter()
        .min_by(|a, b| a.close.partial_cmp(&b.close).unwrap())
        .unwrap();
    
    let max_quote = stock_quotes
        .iter()
        .max_by(|a, b| a.close.partial_cmp(&b.close).unwrap())
        .unwrap();

    let min_time: OffsetDateTime =
    OffsetDateTime::from(UNIX_EPOCH + Duration::from_secs(min_quote.timestamp));

    let max_time: OffsetDateTime =
    OffsetDateTime::from(UNIX_EPOCH + Duration::from_secs(max_quote.timestamp));
    
    let date_format = format_description!("[day]-[month repr:short]-[year]");
    let min_date = min_time.format(&date_format).unwrap();
    let max_date = max_time.format(&date_format).unwrap();

    println!("Min price: {:.6} on {}", min_quote.close, min_date);
    println!("Max price: {:.6} on {}", max_quote.close, max_date);
}

// Function to identify volatile days
fn get_volatie_days(stock_quotes: &Vec<Quote>) -> Vec<&Quote>{
    stock_quotes
        .iter() 
        .filter(|quote| {
            let price_diff = (quote.high - quote.low).abs();
            let percentage_variation = (price_diff / quote.close) * 100.0;
            percentage_variation > 2.0 
        })
        .collect() 
}

// Function to calculate Simple Moving Average (SMA)
fn calculate_sma(stock_quotes: &Vec<Quote>, window_size: usize) -> Vec<Option<f64>> {
    let mut sma_values = Vec::new();
    for i in 0..stock_quotes.len() {
        if i + 1 >= window_size {
            let window = &stock_quotes[i + 1 - window_size..i + 1];
            let average = window.iter().map(|q| q.close).sum::<f64>() / window_size as f64;
            sma_values.push(Some(average));
        } else {
            sma_values.push(None);  // Not enough data to compute SMA for early points
        }
    }
    sma_values
}

// Function to plot stock quotes, volatile days and SMA
fn plot_stock_quotes(ticker: &str, stock_quotes: Vec<Quote>) -> Result<(), Box<dyn std::error::Error>> {
    // Create chart output directory if it doesn't exist already
    let charts_dir = "charts";
    if !std::path::Path::new(charts_dir).exists() {
        std::fs::create_dir(charts_dir)?;
    }

    // Create the filename using the ticker symbol
    let chart_file = format!("{}/{}_chart.png", charts_dir, ticker);

    // Create a chart builder
    let root = BitMapBackend::new(&chart_file, (1280, 720)).into_drawing_area();
    root.fill(&WHITE)?;

    // Calculate the maximum and minimum y-values
    let ymax = stock_quotes.iter().map(|x| x.close).fold(f64::MIN, f64::max) + 5.0;
    let ymin = stock_quotes.iter().map(|x| x.close).fold(f64::MAX, f64::min) - 5.0;

    // Create a chart context
    let mut chart = ChartBuilder::on(&root)
        .caption(format!("{} Closing Price", ticker), ("sans-serif", 40).into_font())
        .margin(25)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..stock_quotes.len(), ymin..ymax)?;

    // Draw the x and y axes
    chart
        .configure_mesh()
        .x_desc("Day")
        .y_desc("Price")
        .x_labels(10)
        .draw()?;

    // Plot the daily closing prices
    chart.draw_series(LineSeries::new(
        stock_quotes.iter().enumerate().map(|(i, quote)| (i, quote.close)),
        &RED,
    ))?.label("Closing Price").legend(|(x, y)| {
        PathElement::new(vec![(x - 10, y), (x + 10, y)], &RED)
    });
   
    // Identify volatile days (>2% intra-day price variation)
    let volatile_days: Vec<&Quote> = get_volatie_days(&stock_quotes);
    
    // Plot error bars for the volatile days
    chart.draw_series(volatile_days.iter().map(|&quote| {
        let idx = stock_quotes.iter().position(|q| q.timestamp == quote.timestamp).unwrap();
        ErrorBar::new_vertical(
            idx,            // X value (index of the day)
            quote.low,      // Minimum price
            quote.close,    // Close price (middle)
            quote.high,     // Maximum price
            BLUE.filled(),  // Color for the error bars
            10              // Width of the error bars
        )
    }))?.label("Volatile Days").legend(|(x, y)| {
        PathElement::new(vec![(x - 10, y), (x + 10, y)], &BLUE)
    });

    // Calculate the 20-day SMA
    let sma_20 = calculate_sma(&stock_quotes, 20);

    // Plot the 20-day SMA on the same chart
    chart.draw_series(LineSeries::new(
        sma_20.iter().enumerate().filter_map(|(i, &sma)| {
            if let Some(sma_value) = sma {
                Some((i, sma_value))
            } else {
                None
            }
        }),
        &GREEN,
    ))?.label("20-Day SMA").legend(|(x, y)| {
        PathElement::new(vec![(x - 10, y), (x + 10, y)], &GREEN)
    });

    // Configure and display the legend
    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::LowerRight)
        .border_style(&BLACK)
        .draw()?;

    // Add dates along the x-axis for every month
    for (i, quote) in stock_quotes.iter().enumerate() {
        if i % 21 == 0 { // ~21 weekdays in 1 month
            let time: OffsetDateTime = OffsetDateTime::from(UNIX_EPOCH + Duration::from_secs(quote.timestamp));
            let date_format = format_description!("[day]-[month repr:short]");
            let date = time.format(&date_format).unwrap();
            let x = if stock_quotes.len() - i >= 3 { i } else { i - 2 };
            chart.draw_series(
                std::iter::once(Text::new(
                    date.to_string(),
                    (x, ymax),
                    ("sans-serif", 12),
                ))
            )?;
        }
    }

    println!("Please locate chart at {}", chart_file);
    return Ok(());
}
