import pandas as pd
import pandas_profiling
d=pd.read_csv('https://raw.githubusercontent.com/mwaskom/seaborn-data/master/planets.csv')
d.profile_report()
d.sample(5)
d.describe()


df .iloc


# MY NOTES
## DF CREATION
- pd.DataFrame(np.random.randn(4,3), columns=list('abc'), index=['first', 'sec', 'thr', 'forth'])


## DF COLUMN PROCESSING
    df['a'] = df['a'].apply(lambda x: x + 1)

## DF PROCESSING
### FILTERING
- [SO Example](https://stackoverflow.com/questions/51589573/pandas-filter-data-frame-rows-by-function) - 

## DF ADVANCED
.crosstab()



VISUALIZATION
sns.headmap()