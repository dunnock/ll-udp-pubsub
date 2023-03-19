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
    print(f"| p99    |   {results.quantile(0.99):5.2f}µs  |")
    print(f"| min    |   {results.min():5.2f}µs🦄|")
    print(f"| max    |   {results.max():5.2f}µs🐌|")
 
