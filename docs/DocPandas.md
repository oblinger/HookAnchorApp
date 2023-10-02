 [[DocSciKit]] 
 - [ ] import pandas as pd
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
    df['delta'] = df['col'].diff()

## DF PROCESSING
df1.join(df2)
pd.merge(df1, df2, on = "col_a", how = "inner")
.shuffle
.batch ???

## DF FILTERING
- df = df[ df.index < '2020-03-31']
- df = df.dropna()   # drops empty entries
- df.asfreq('Q')  # Quarterly ??
- 
- [SO Example](https://stackoverflow.com/questions/51589573/pandas-filter-data-frame-rows-by-function) - 
### COUNTING
- COUNTING:  data["Company Name"].value_counts()
- GROUP BY:  data.groupby("Company Name").size()   (size will return same counts as above)
### ITERATE
    for i in range(len(df)): salary_sum += df.iloc[i]['Employee Salary']
    for index, row in df.iterrows(): salary_sum += row['Employee Salary']
    for row in df.itertuples(): salary_sum += row._4

## DF ADVANCED
.crosstab()


## VISUALIZATION
df['gdp'].plot(legends=True, figsize=(16,8))
sns.headmap()



import matplotlib.pyplot as plt
plt.scatter(X[0, :], X[1, :], c=Y, s=40, cmap=plt.cm.Spectral);   # plots numpy array

## NUMPY

df.values[-1:]  
## CSV
pd.read_csv('data.csv')

## MATRIX MATH

### Multiplication

[a, b] dot [b, c] = [a, c]      
# REF
### r2022-11-08  Pandas Examples
-  [https://github.com/wesm/pydata-book](https://ideaspace.substack.com/p/on-competition?utm_medium=email&utm_campaign=cta&utm_source=substack)  
- 