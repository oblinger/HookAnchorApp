- The standard check to see if a time series is [[Stationary]].

- [Mathy Intuitive Explanation](https://www.youtube.com/watch?v=1opjnegd_hA) - small bit of python code at end too.

    from statsmodels.tsa.stattools import adfuller



DICKEY-FULLER TEST
- Checks if phi = 1 as the null hypothesis (that we DO have a unit root)
- is a kind of T distribution but comparing to the dicky-fuller distribution
- Reject null hyp means we have evidence series is stationary


AUGMENTED DICKEY-FULLER TEST
- ARP model - sum over first p lag-terms


UNIT ROOTS - A time series has a unit root if phi is +1 or is -1 for any lags
==> These cause series to not be stationary


