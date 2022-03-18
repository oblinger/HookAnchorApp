<%-* const title = await tp.system.prompt("NAMED sub-note's title");
		tp.file.cursor_append(" [[" + title + "]]\n");
		tp.file.rename(title) -%>

