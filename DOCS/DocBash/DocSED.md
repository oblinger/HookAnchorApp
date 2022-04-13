 [[Unix Command line]]
    [SED examples](https://linuxhint.com/50_sed_command_examples/#s11)  

	# Replace all data...js with {{{...}}} in place
	sed -i '' 's/data\(.*\)js/{{{\1}}}/' afile.md