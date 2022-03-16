import pandas as pd
import pandas_profiling
d=pd.read_csv('https://raw.githubusercontent.com/mwaskom/seaborn-data/master/planets.csv')
d.profile_report()
d.sample(5)
d.describe()
