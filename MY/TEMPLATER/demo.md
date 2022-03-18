

## Date Z
Date now <% tp.date.now() %>
Date next week <% tp.date.now("YY-MM-DD", 7) %>
Date next month <% tp.date.now("YYYY-MM-DD", "P+1M") %>
Date on Tuesday this week <% tp.date.weekday("YYYY-MM-DD", 1) %>
Today's day of the week: <% moment().format("dddd") %>

## File Z
exists <% tp.file.exists("All") %>
folder <% tp.file.folder() %>
folder relative 
