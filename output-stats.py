import pandas as pd
import sys

data = pd.read_csv(sys.stdin)
print(((data['received_ts'][1000:] - data['sent_ts'][1000:]) * 1e-3).describe().round(2))

