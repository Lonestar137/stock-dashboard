import time
import requests
import schedule
import json

from dotenv import dotenv_values
from prometheus_client import CollectorRegistry, Gauge, push_to_gateway

CONFIG = dotenv_values()

# Feature flags
DEBUG = True
SKIP_API = True


# Define a function to fetch historical stock data for a given symbol
def fetch_stock_data(symbol):
    params = {
        'function': 'TIME_SERIES_WEEKLY',
        'symbol': symbol,
        'apikey': CONFIG['ALPHA_VANTAGE_API_KEY']
    }
    response = requests.get(CONFIG['ALPHA_VANTAGE_URL'], params=params)
    data = response.json()

    # Extract the last five years of weekly data (Alpha Vantage provides it in the response)
    weekly_data = data.get('Weekly Time Series', {})
    return weekly_data


# Define a function to send stock data to Prometheus Pushgateway
def upload_to_prometheus(symbol, weekly_data):
    registry = CollectorRegistry()
    stock_price_gauge = Gauge('stock_price',
                              'Stock price in USD', ['symbol', 'date'],
                              registry=registry)

    for date, data in list(weekly_data.items()
                           )[:260]:  # Fetch last 5 years (52 weeks * 5 years)
        closing_price = float(data['4. close'])
        stock_price_gauge.labels(symbol=symbol, date=date).set(closing_price)

    # Push the data to Prometheus Pushgateway
    push_to_gateway(CONFIG['PUSHGATEWAY_URL'],
                    job=f'stock_data_{symbol}',
                    registry=registry)


# Function to fetch and upload data for a list of symbols
def process_stock_data(symbols):
    for symbol in symbols:
        print(f"Fetching data for {symbol}...")
        if not SKIP_API:
            weekly_data = fetch_stock_data(symbol)
        else:
            print('Skipping API. . .')
            with open('integration/sample.json', 'r') as f:
                weekly_data = json.loads(f.read())

        if DEBUG and weekly_data != None:
            try:
                with open('integration/sample.json', 'w') as f:
                    f.write(json.dumps(weekly_data))
            except:
                import ptpython
                ptpython.embed(globals(), locals(), mode='vi')

        if weekly_data:
            upload_to_prometheus(symbol, weekly_data)
        else:
            print(f"Error fetching data for {symbol}")


# Function to monitor stock prices periodically
def monitor_stock_prices(symbols, interval_minutes=15):

    def fetch_and_upload():
        for symbol in symbols:
            print(f"Monitoring stock price for {symbol}...")
            weekly_data = fetch_stock_data(symbol)
            if weekly_data:
                upload_to_prometheus(symbol, weekly_data)
            else:
                print(f"Error monitoring stock price for {symbol}")

    # Schedule the task to run periodically (free tier limited to 5 calls per minute)
    schedule.every(interval_minutes).minutes.do(fetch_and_upload)

    # Keep the schedule running
    while True:
        schedule.run_pending()
        time.sleep(1)


def main():
    # Example usage
    stock_symbols = ['AAPL', 'GOOGL',
                     'MSFT']  # List of stock symbols to monitor
    process_stock_data(stock_symbols)  # Fetch historical data


if __name__ == "__main__":
    main()

    params = {
        'function': 'TIME_SERIES_WEEKLY',
        'symbol': 'IBM',
        'apikey': '2OBLSYSNWE2WY9MA'
    }
