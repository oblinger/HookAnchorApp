#dl

Positional Encoding 
-?-
A "positional embedding" is a unique d-dimensional vector that can be added to each embedded token vector to derive a "positional encoding" of the original input.
.
Many choices for this.  In attention is all you need they used sine and cosine w/ different frequencies
![[Screenshot 2024-10-29 at 9.41.23 PM.png]] <!--SR:!2025-04-07,17,210--> 


- Ensures that relative positions pos to pos+k will end up being a linear transform
- PE(pos, 0)  ==  sin(pos)  Fast wave
- PE(pos, d_model) == sin(pos/10000)  Very slow wave

Tokens Embedded as d-dimensional vectors can be summed with a "positional em"