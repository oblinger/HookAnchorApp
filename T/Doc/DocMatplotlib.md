
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



fig, ax1 = plt.subplots()					# Overlays multple plots in one figure
color = 'tab:red'
ax1.plot(cost_list, color=color)
ax1.set_xlabel('epoch', color=color)
ax1.set_ylabel('Cost', color=color)
ax1.tick_params(axis='y', color=color)
    
ax2 = ax1.twinx()  
color = 'tab:blue'
ax2.set_ylabel('accuracy', color=color) 
ax2.set_xlabel('epoch', color=color)
ax2.plot( accuracy_list, color=color)
ax2.tick_params(axis='y', color=color)
fig.tight_layout()							# used to make subplots cleaner

```
