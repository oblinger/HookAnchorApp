
import tensorflow as tf


tf.constant(np.random(3,1), name = 'X')



### MATH

- tf.add(x, y)    
- MATRIX:  tf.matmul(x, y)
- tf.cast(z, tf.float32)

- tf.keras.activations.sigmoid   .relu

## COST FN

[tf.keras.losses.categorical_crossentropy(y_true,y_pred)](https://www.tensorflow.org/api_docs/python/tf/keras/losses/categorical_crossentropy)   


## VARIABLE


    initializer = tf.keras.initializers.GlorotNormal(seed=1) 
    W1 = tf.Variable(initializer(shape=(25, 12288)), name='W1')


## TENSOR
### TENSOR CREATE

- -   [tf.one_hot(labels, depth, axis=0)](https://www.tensorflow.org/api_docs/python/tf/one_hot) 
- tf.transpose()
  


## GRAD TAPE

