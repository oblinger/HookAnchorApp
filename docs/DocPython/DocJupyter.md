

### Launch on Mac

	// assuming installed
	jupyter lab


  ### Runs magic command from inside a python script
  
	  // First one much pip install IPython
	from IPython.terminal.embed import InteractiveShellEmbed    
	ipshell = InteractiveShellEmbed()  
	ipshell.dummy_mode = True  
	print('\nTrying to call IPython which is now "dummy":')  
	ipshell.magic("%timeit abs(-42)")  
	ipshell()  
	print('Nothing happened...')