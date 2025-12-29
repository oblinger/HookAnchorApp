


CNN
-?-
- Convolutional Neural Network
- nn.Sequential(nn.Conv2d(), nn.MaxPool(), nn.Conv2d(), ..., nn.Linear(), nn.Softmax())

Convolution
-?-
- Multiplies successive patches of a multidimensional input by a kernel and sums the result in order to obtain each cell value output in the resulting output
- nn.Conv2d(in_channels=, out_channels=, kernel_size=, stride=)    # e.g. in_channels = 3 (for  R/G/B).   stride=2 or stride=(2,2)
- output_size = M-K/stride+1.    M=input_size, K=kernel_size
- nn.MaxPool(2, stride=)
- torch.max_pool2d(image, stride=,kernel_size=)