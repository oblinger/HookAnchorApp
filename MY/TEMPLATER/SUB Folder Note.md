<%-*
		const title = await tp.system.prompt("SUB Folder Note");
		const dir = require("path").dirname(tp.file.path(true)) + "/" + title;
		await this.app.vault.createFolder(dir);
		await tp.file.create_new("", title + "/" + title + ".md", false);
		tp.file.cursor_append("[[" + title + "]] ");
		-%>
