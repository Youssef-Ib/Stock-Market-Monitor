

import yfinance as yf
import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
from sklearn.linear_model import LinearRegression
from datetime import datetime, timedelta

# Constants
EXE_START = "\n~~~~~~~~~~ Execution Start ~~~~~~~~~~"
EXE_END = "\n~~~~~~~~~~~ Execution End ~~~~~~~~~~~"

# Fetch stock quotes for the last 6 months
def get_stock_quotes(ticker):
    end_date = datetime.now()
    start_date = end_date - timedelta(days=180)
    stock_data = yf.download(ticker, start=start_date, end=end_date, interval='1d')
    stock_data.dropna(inplace=True)  # Remove missing values

    # If we have a MultiIndex, drop the top level so that columns become "Open", "High", etc.
    if isinstance(stock_data.columns, pd.MultiIndex):
        stock_data.columns = ["Open", "High", "Low", "Close", "Volume"]

    # if you only have 5 columns, rename explicitly:
    # stock_data.columns = ["Open", "High", "Low", "Close", "Volume"]
    # Or if you see 6 columns, do:
    # stock_data.columns = ["Open", "High", "Low", "Close", "Adj Close", "Volume"]

    print(stock_data.columns)
    print(stock_data.head())
    return stock_data



# Calculate Simple Moving Average (SMA)
def calculate_sma(data, window_size):
    return data['Close'].rolling(window=window_size).mean()

# Identify volatile days (price difference > 2% of close)
def get_volatile_days(data):
    # Calculate the price difference and percentage variation
    data['Price_Diff'] = data['High'] - data['Low']
    data['Percentage_Variation'] = (data['Price_Diff'] / data['Close']) * 100
    
    # Filter volatile days where the percentage variation exceeds 2%
    volatile_days = data.loc[data['Percentage_Variation'] > 2]
    return volatile_days



# Train ML Model for Stock Prediction
def train_predictor(data):
    data['Day'] = np.arange(len(data))
    X = data[['Day']].values
    y = data['Close'].values
    
    model = LinearRegression()
    model.fit(X, y)
    future_days = np.array(range(len(data), len(data) + 30)).reshape(-1, 1)
    predictions = model.predict(future_days)
    return predictions, model

def plot_stock_quotes(ticker, data, sma_20, predictions):
    plt.figure(figsize=(14, 7))
    
    # Plot Closing Prices
    plt.plot(data['Close'], label=f"{ticker} Closing Prices", color='blue')
    
    # Plot SMA
    plt.plot(sma_20, label='20-Day SMA', color='orange')
    
    # Plot Future Predictions
    future_dates = pd.date_range(start=data.index[-1], periods=31)[1:]  # Exclude start date
    plt.plot(future_dates, predictions, label='Future Predictions', color='red', linestyle='--')
    
    # Highlight Volatile Days
    volatile_days = get_volatile_days(data)
    plt.scatter(volatile_days.index, volatile_days['Close'], color='purple', label='Volatile Days')
    
    # Chart Details
    plt.title(f"{ticker} Stock Price Analysis")
    plt.xlabel("Date")
    plt.ylabel("Price")
    plt.legend()
    plt.grid()
    plt.show(block=False)
    plt.savefig("stock_plot.png")




# Main function
def main():
    print(EXE_START)

    # Get user input
    ticker = input("Enter stock ticker symbol: ").upper()
    
    print(f"\nFetching quotes for stock ticker: {ticker}...")
    stock_data = get_stock_quotes(ticker)
    
    if stock_data.empty:
        print(f"No stock quotes available for ticker: {ticker}")
        print(EXE_END)
        return

    # Calculate min and max closing prices
    min_price = stock_data['Close'].min()
    max_price = stock_data['Close'].max()

    # Ensure idxmin and idxmax return single timestamps
    min_date = stock_data['Close'].idxmin()
    max_date = stock_data['Close'].idxmax()

    # If min_date and max_date are Series, extract the first value
    if isinstance(min_date, pd.Series):
        min_date = min_date.iloc[0]
    if isinstance(max_date, pd.Series):
        max_date = max_date.iloc[0]

    # Convert to string format
    min_date = pd.Timestamp(min_date).strftime("%d-%b-%Y")
    max_date = pd.Timestamp(max_date).strftime("%d-%b-%Y")

    # Print the results
    print(f"Min price: {float(min_price.iloc[0]) if isinstance(min_price, pd.Series) else float(min_price):.2f} on {min_date}")
    print(f"Max price: {float(max_price.iloc[0]) if isinstance(max_price, pd.Series) else float(max_price):.2f} on {max_date}")



    
    # Calculate SMA
    sma_20 = calculate_sma(stock_data, 20)
    
    # Train ML predictor
    print("\nTraining machine learning model for prediction...")
    predictions, model = train_predictor(stock_data)
    
    # Plot results
    print(f"\nPlotting stock data for {ticker}...")
    plot_stock_quotes(ticker, stock_data, sma_20, predictions)
    
    print(EXE_END)

if __name__ == "__main__":
    main()
