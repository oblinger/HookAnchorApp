
table file.mtime as "Last modified" where file.name != this.file.name SORT file.mtime DESC LIMIT 9

DATAVIEW DATA TABLE

EDIT HISTORY
```dataview
table dateformat(file.mtime, "YY-MM-DD") as "Last modified" where file.name != this.file.name SORT file.mtime DESC LIMIT 400
```
