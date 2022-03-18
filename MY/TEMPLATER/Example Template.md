Template Top
<% tp.file.create_new("this is the new files contents", "xample", true).basename %>
<% tp.file.cursor_append("Squirt into orig page") %>
< tp.file.move("MY/" + tp.file.title) >
< tp.file.rename("mynewname")

## Date
Date now <% tp.date.now() %>
Date next week <% tp.date.now("YY-MM-DD", 7) %>
Date next month <% tp.date.now("YYYY-MM-DD", "P+1M") %>
Date on Tuesday this week <% tp.date.weekday("YYYY-MM-DD", 1) %>
Today's day of the week: <% moment().format("dddd") %>

## Tiny
I am tiny

## File
Title of parent note <% tp.file.title %>
Created <% tp.file.creation_date("YY-MM-DD") %>   
Modified <% tp.file.last_modified_date("YY-MM-DD") %>  
exists <% tp.file.exists("All") %>
folder <% tp.file.folder() %>
folder full relative <% tp.file.folder(true) %>
folder path <% tp.file.path() %>
folder relative path <% tp.file.path(true) %>

include section < tp.file.include("[[Example Template#Tiny]]") >

