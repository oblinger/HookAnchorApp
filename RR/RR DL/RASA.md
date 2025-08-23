
RASA
-?-
n:: a framework that allows developers to build, train, and deploy conversational AI models.
- [Rasa Alg](https://www.youtube.com/watch?v=yGTUuEx3GkA) whiteboard example.  
	- Self Attention -- recurrent within
	- https://www.youtube.com/watch?v=tIvKXrEDMhk

- Attention Videos:  [1](https://www.youtube.com/watch?v=yGTUuEx3GkA), [2](https://www.youtube.com/watch?v=tIvKXrEDMhk), [3](https://www.youtube.com/watch?v=23XUv0T9L5c),  [4](https://www.youtube.com/watch?v=EXNBy8G43MM), 

Analogy:
- Query - Transform of word whose context is being computed (using dot product)
- Keys - Transform of words being used AS the context (also in the dot product)
- Values - Transform of the value of the word begin reweighed by in order to produce the contextualized word



**SINGLE ENCODER BLOCK**
- POSITIONAL ENCODING - Added to each input work to "mix in" positional info
- MULT-HEADED ATTENTION BLOCK - To add contextualization to each token
- FEED FORWARD NET - A standard feedforward net


**THIS IS A SINGLE ATTENTION BLOCK**

X -> KEYS, QUERIES, VALUES. (derives three embeddings via a linear transforms of X)

For i in words
	WEIGHTS_i = Sum_j( QUERY_i dot KEYS_j )

WEIGHTS_i = Normalize( WEIGHTS_i )
WEIGHTS_i = Dropout ????

RESULT_i = Sum( WEIGHTS_i * VALUES_i )



POSITIONAL ENCODING

Fixed embeddings for each postionGoodReads


## STARSPACE ALG
- IDEA: train two different embeddings to map bag of words or bag of tags into points in same star-space.
- related to algs: DIET, & TED

- https://www.youtube.com/watch?v=ZT3_9Kjx7oI
- tags and documents in same clustering space
- build word embeddings that are tied to domain specific texts.

- Builds different embeddings for bag-of-tags and bag-of-words for sentences.
	- Trained so that docs with words and tags yields strong similarity score.


## TED
- GOAL: Goal direct text-based assistant
- IDEA: Unidirectional-Transformer on utterance/action sequence  Flix
- &  Starspace
- https://www.youtube.com/watch?v=j90NvurJI4I