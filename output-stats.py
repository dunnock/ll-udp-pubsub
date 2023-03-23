import pandas as pd
import sys

for file in sys.argv[1:]:
    data = pd.read_csv(file)
    results = (data['received_ts'][1000:] - data['sent_ts'][1000:]) * 1e-3

    print(file)
    print( "|        |  time      |")
    print( "|--------|------------|")
    print(f"| mean   |   {results.mean():5.2f}µs  |")
    print(f"| std    |   {results.std():5.2f}µs  |")
    print(f"| p01    |   {results.quantile(0.01):5.2f}µs🦄|")
    print(f"| p9     |   {results.quantile(0.9):5.2f}µs  |")
    print(f"| p999   |   {results.quantile(0.999):5.2f}µs🐌|")
 
