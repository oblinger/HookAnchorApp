---
sr-due: 2024-09-23
sr-interval: 1
sr-ease: 230
---


AutoRegressive Integrated Moving Average

- Predicts deltas between time steps.  (Uses a moving average)
- [Pencil Paper Intuitive Explanation](https://www.youtube.com/watch?v=4Fiz3dQM_i8) - 
- [Explanation](https://www.youtube.com/watch?v=3UmyHed0iYE) - 

ARIMA
- **AR = [[AutoRegressive]]** - Using the dependent relationship between an observation and some number of lagged observations.  (p = the number of lagged observations, the lag order)
- **I = Integrated** - Using the differencing of raw observations.  (d = the number of deltas; degree of differencing)
- **MA = MovingAverage** - Uses the dependency between an observation and residual errors from a moving average model applied to lagged observations.  
  (q = Order of moving average)

ASSUMES 
- **[[Stationary]]** - 
- **Uncorrelated random error** - Error term is randomly distributed w/ mean and variance are constant over time - Durbin Watson Test
- **No Outliners** - 
- **Random Shocks** - if shocks exist they are mean of zero and constant variance


Box-Jenkins Method (Iterates these steps)
1. IDENTIFICATION - 
	- Check Stationarity - If not then how many differences needed to make it stationary
	- Identify Parameters - 
		- ACF - Autocorrection Function - plot correlation of an observation with each lag value
		  ==> Model is AR if ACF trails off after p   & PACF has hard cut-off after p     (this is the p parameter)
		- PACF - Partial Autocorrelation Function - correlations for observation with lag values not accounted for by prior lagged values
		  ==> Model is AR if PACF trails off after q   & ACF has hard cut-off after q     (this is the q parameter)
		- Is a mix if both ACF and PACF trail off
2. ESTIMATION - Parameter estimation
	- LEAST SQUARES - can be use.   (MA component introduces complexity)
3. DIAGNOSTIC CHECKING - Diagnostic Checking
	- OVERFITTING - Check in sample and uot of sample performance
	- RESIDUAL ERROR - Use density plots, histograms, [[Q-Q Plot]] to see if distribution is non-Gaussian
		- SKEW - non-zero mean suggests bias 
		- NO SERIAL STRUCTURE - Look at ACF and PACF to see if there is further structure to add to the model


Arima(p, d, q)
- p
- d = number of differences done on the data.   d=2 means difference between differences