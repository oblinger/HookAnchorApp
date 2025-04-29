
- [[Meta Coms]], [[Coms]], [[COM Next]], [[Calls]], [[Friends]], [[Mentors]], [[BOD]], 


```dataviewjs 
let line = ""
let lines = {}
let timeframe = ""
let when = ["ASAP", "Q2", "Q3", "Q4", "Soon", "Y24", "*", "Q1"]
let star_idx = when.indexOf("*")
let tags = ["Soon", "Core", "Mentor", "Lunch", "Friend", "FAANG", "Ping", "CRM", "Startup", "VC", "NonProfit", "AI", "Calls"];
let crm_contents = await dv.io.load("At/CRM.md")
for (let tag of tags) {
    let pages = dv.pages("#" + tag);  
    dv.el("article", "**" + tag.toUpperCase() + "**")
    lines = {}
    line = " . . "
    for (let p of pages) { 
        if (!crm_contents.contains(p.file.name)) { 
             line = line + "[[" + p.file.name + "]],     " 
             timeframe = null 
             //for (let w of when) {
             //    if (dv.pages("#"+w).indexOf(p)) {timeframe=w}
             //}
        } 
    }
    dv.el("article", line)
    dv.el("article", " . . ")
}
```
