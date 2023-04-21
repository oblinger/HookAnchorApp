

```dataviewjs 
let tags = ["Lunchx", "CRMx"];
let crm_contents = await dv.io.load("At/CRM.md")
for (let tag of tags) {
    let pages = dv.pages("#" + tag);  
    dv.el("article", "## " + tag + " Entries")
    for (let p of pages) { 
        if (crm_contents.contains(p.file.name)) {}
        else { 
            dv.el("article", "- [[" + p.file.name + "]]") 
        } 
    }
}
```

