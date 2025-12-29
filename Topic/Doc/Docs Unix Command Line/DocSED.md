 [[Unix Command Line]]
    [SED examples](https://linuxhint.com/50_sed_command_examples/#s11)  

	# Global Search and Replace
	find . -type f -name "*.md" -print0 | xargs -0 sed -i '' -e 's|- added-by-km-cmd||g'   # Not using REGEXs


	# Replace all data...js with {{{...}}} in place
	sed -i '' 's/data\(.*\)js/{{{\1}}}/' afile.md
	# Replace all 20xx.xx.xx  with  20xx-xx-xx
	sed -i '' 's/20\([0-9][0-9]\)\.\([0-9][0-9]\)\.\([0-9][0-9]\)/20\1-\2-\3/g' **/*.md
