
table file.mtime as "Last modified" where file.name != this.file.name SORT file.mtime DESC LIMIT 9

EDIT HISTORY
```dataview
table dateformat(file.mtime, "YY-MM-DD") as "Last modified" where file.name != this.file.name SORT file.mtime DESC LIMIT 90
```