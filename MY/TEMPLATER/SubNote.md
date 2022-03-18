<%-* const title = tp.date.now() + "  " + await tp.system.prompt("Meeting Title");
		tp.file.cursor_append(" [[" + title + "]]\n");
		tp.file.rename(title) -%>

