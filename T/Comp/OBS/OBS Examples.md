

```dataview
TABLE file.mtime as Edited, file.size as Size FROM #CRM WHERE file.name!="WaitingFor" and !contains(file.name, "spam") SORT file.mtime desc
```


```dataviewjs 
dv.el(await dv.io.load("My/Plan/quick.md"))
```
[[At.md]] 

- [[@Michael Freed]]

```dataviewjs 
let crm_contents = await dv.io.load("At/CRM.md")
let pages = dv.pages("#CRM"); 
dv.el("article", "END")
for (let p of pages) { 
    if (crm_contents.contains(p.file.name)) {}
    else { 
        dv.el("article", "- [[" + p.file.name + "]]") 
    } 
}
```

