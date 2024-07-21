


# Query

```query
tag:#build
```







# DV


```dataview
TABLE file.link AS "Note", section, section.text as TXT
FLATTEN file.sections AS section
WHERE contains(section.text, "#build")
SORT file.name ASC
```



```dataview
TABLE file.link AS "Note", file.tags AS "Tags"
WHERE contains(file.tags, "#build")
SORT file.name ASC
```

