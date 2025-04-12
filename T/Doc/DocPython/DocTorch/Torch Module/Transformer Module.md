

(see [[Transformers]])


```python

transformer_model = nn.Transformer(nhead=16, num_encoder_layers=12)


embed_dim = 240
num_heads = 12
num_layers = 12
encoder_layer = nn.TransformerEncoderLayer(d_model=embed_dim, nhead=num_heads)
transformer_encoder = nn.TransformerEncoder(encoder_layer, num_layers=num_layers)

seq_length = 20
batch_size = 1
x = torch.rand((seq_length, batch_size, embed_dim))
encoded = transformer_encoder(x)
print("Encoded Tensor Shape:", encoded.shape)



```


