import pandas as pd
import sys

data = pd.read_csv(sys.stdin)
print((data['received_ts'] - data['sent_ts']) .describe())

