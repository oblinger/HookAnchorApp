
sklearn.metrics
-?-
Visualizations
```python
from sklearn.metrics import confusion_matrix, classification_report, roc_curve, auc
#
yhat = model(X_test)
y_pred_test_labels = (yhat > 0.5).float()
cm = confusion_matrix(y_test, y_pred_test_labels)
plt.imshow(cm, interpolation='nearest', cmap=plt.cm.Blues)
#
y_pred_test = model(X_test).detach().numpy()
fpr, tpr, thresholds = roc_curve(y_test, y_pred_test)
roc_auc = auc(fpr, tpr)
#
print("Classification Report:\n", classification_report(y_test, y_pred_test_labels, target_names=['Loss', 'Win']))
```
<!--SR:!2025-03-22,1,250-->



#dl 
#review


## Examples

### Confusion Matrix
```python
yhat = model(X_test)
y_pred_test_labels = (yhat > 0.5).float()
cm = confusion_matrix(y_test, y_pred_test_labels)

plt.figure(figsize=(6, 6))
plt.imshow(cm, interpolation='nearest', cmap=plt.cm.Blues)
plt.title('Confusion Matrix')
# plt.colorbar()
tick_marks = range(2)
plt.xticks(tick_marks, ['Loss', 'Win'], rotation=45)
plt.yticks(tick_marks, ['Loss', 'Win'])
cm
```


### Plot ROC curves
```python
# Print classification report
print("Classification Report:\n", classification_report(y_test, y_pred_test_labels, target_names=['Loss', 'Win']))

# Plot ROC curve
y_pred_test = model(X_test).detach().numpy()
fpr, tpr, thresholds = roc_curve(y_test, y_pred_test)
roc_auc = auc(fpr, tpr)

plt.figure()
plt.plot(fpr, tpr, color='darkorange', lw=2, label=f'ROC curve (area = {roc_auc:.2f})')
plt.plot([0, 1], [0, 1], color='navy', lw=2, linestyle='--')
plt.xlim([0.0, 1.0])
plt.ylim([0.0, 1.05])
plt.xlabel('False Positive Rate')
plt.ylabel('True Positive Rate')
plt.title('Receiver Operating Characteristic (ROC)')
plt.legend(loc="lower right")
plt.show()
```


### Show Model Weights
```python
weights = model.linear.weight.data.numpy().flatten()
features = X.columns
Create a DataFrame for feature importance:
feature_importance = pd.DataFrame({'Feature': features, 'Importance': weights})
feature_importance = feature_importance.sort_values(by='Importance', ascending=False)
print(feature_importance)
Plot feature importance plt.figure(figsize=(10, 6))
plt.bar(feature_importance['Feature'], feature_importance['Importance'])
plt.xlabel('Features')
plt.ylabel('Importance')
plt.title('Feature Importance')
plt.xticks(rotation=45)
plt.show()
```


from sklearn.metrics import average_precision_score, classification_report
from sklearn.metrics import log_loss, matthews_corrcoef, roc_auc_score