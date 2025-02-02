
```

%matplotlib inline
plt.rcParams['figure.figsize'] = (5.0, 4.0) # set default size of plots
plt.rcParams['image.interpolation'] = 'nearest'
plt.rcParams['image.cmap'] = 'gray'



import matplotlib.pyplot as plt


plt.plot(a_list,label = "1D list of numbers")
plt.xlabel('position')
plt.ylabel('value')
plt.legend()
plt.show()

plt.plot(x_series, y_series).show()

plt.semilogx(np.array(learning_rates), train_error.numpy(), label=)

```
