(My command is mdf)

	Syntax
	      mdfind [-live] [-count] [-onlyin _directory_] _query_ 

Return all files in the users home folder (~) that have been modified in the last 3 days:

    $ mdfind -onlyin ~ 'kMDItemFSContentChangeDate >= $time.today(-3)'


mdf
  --last DAYS  --type 
  
	$ mdfind 'kMDItemFSCreationDate >= $time.today(-3)'       # Created last n days
	$ mdfind -onlyin /User/oblinger/ob/data                   # Under folder
	$ mdfind 'kMDItemFSContentChangeDate >= $time.today(-3)'  # Modified last n days
	$ mdfind 'kMDItemFSName = "*.md"'                        # Name matches
	$ mdfind 'kMDItemFSName = "*.md"c'                     # Name match (case insensitive)
	$ mdfind "kMDItemFSName = '*.md' && kMDItemFSContentChangeDate >= $time.today(-3)"



#### Kinds
> Applications kind:application, kind:applications, kind:app  
> Audio/Music kind:audio, kind:music  
> Bookmarks kind:bookmark, kind:bookmarks  
> Contacts kind:contact, kind:contacts  
> Email kind:email, kind:emails, kind:mail message, kind:mail messages  
> Folders kind:folder, kind:folders  
> Fonts kind:font, kind:fonts  
> iCal Events kind:event, kind:events  
> iCal To Dos kind:todo, kind:todos, kind:to do, kind:to dos  
> Images kind:image, kind:images  
> Movies kind:movie, kind:movies  
> PDF kind:pdf, kind:pdfs  
> Preferences kind:system preferences, kind:preferences  
> Presentations kind:presentations, kind:presentation

#### ATTR
`kMDItemPath`: absolute path.  
`kMDItemFSName`: basename.  
`kMDItemFSSize`: size in bytes.  
`kMDItemDisplayName`: display name.  
`kMDItemContentType`: UTI.  
`kMDItemContentTypeTree`: UTI and parent UTIs.  
`kMDItemKind`: a localized name for the content type.  
`kMDItemFSContentChangeDate`: modification time.  
`kMDItemFSCreationDate`: creation time.  
`kMDItemLastUsedDate`: date last opened.  
`kMDItemDateAdded`: date moved or date first indexed.  
`kMDItemFinderComment`: Spotlight comment.  
`kMDItemTextContent`: plain text content of for example text, HTML, or PDF files.  
`kMDItemDurationSeconds`: for example the duration of an audio or video file.  
`kMDItemTitle`: for example the title of a webloc, PDF, or MP3 file.  
`kMDItemURL`: for example the URL of a webloc file.  
`kMDItemWhereFroms`: the URL or URLs that a file was downloaded from.  
`kMDItemUserTags`: tags.  
`kMDItemAuthors`: for example the artist of an MP3 file.

#### [EXAMPLES](https://web.archive.org/web/20161118160953/http://osxnotes.net/spotlight.html)  
```
mdfind '"exact phrase"'
mdfind -name javascript -onlyin ~/Books
mdfind 'kMDItemFSName=*.pdf'
mdfind 'kMDItemFSName=mdfind&&kMDItemContentType=public.unix-executable'
mdfind kMDItemContentType=com.apple.application-bundle -onlyin /usr/local
mdfind kMDItemContentTypeTree=com.apple.bundle
mdfind kMDItemContentTypeTree=public.movie
mdfind 'kMDItemTextContent="*expose*"cd' # search for files whose text content matches `expose` ignoring case and diacritics
mdfind 'kMDItemTextContent="*search phrase*"c' -onlyin ~/Library/Caches/Metadata/Safari/History
mdfind kMDItemFSSize=$(stat -f%z file) # quickly find potentially duplicate files by size
mdfind 'kMDItemFSCreationDate>$time.iso(2014-03-16T01:03:08Z)' -onlyin . # like `TZ=UTC0 find . -newerBt '2014-03-16 01:03:08'`
mdfind 'kMDItemFSContentChangeDate>$time.now(-86400)' -onlyin ~/Library/Caches/Metadata/Safari/History|xargs mdls -n kMDItemURL|cut -d\" -f2
mdfind 'kMDItemDateAdded>$time.today(-7)' # find files that were moved or first indexed in the last week
mdfind 'kMDItemLastUsedDate!=*' -onlyin . # find files that have not been opened before
mdfind 'kMDItemKind=*movie&&kMDItemPixelHeight>=720'
mdfind 'kMDItemAuthors="Artist Name"' -onlyin ~/Music
mdfind 'kMDItemContentType=*' -onlyin . # list all indexed files (kMDItemFSName=* stopped working in 10.9)
mdfind 'kMDItemURL=*stackoverflow.com*' -onlyin ~/Library/Caches/Metadata/Safari/History
mdfind 'kMDItemFSSize>1e8'
mdfind -0 -onlyin ~/Music 'kMDItemFSName=*.mp3&&kMDItemAudioBitRate<=192000'|xargs -0 mdls -n kMDItemAlbum|sort -u
mdfind $'kMDItemFSName="*\n*"'
mdfind -onlyin / # search only in the main OS X volume, like `mdfind -onlyin /Volumes/Macintosh\ HD`
mdfind 'kMDItemDurationSeconds<=180&&kMDItemContentType=public.mp3' -onlyin .
mdfind 'kMDItemUserTags=*' # find files that have tags
mdfind -s example # like `mdfind "$(PlistBuddy -c 'Print RawQuery' ~/Library/Saved\ Searches/example.savedSearch)"`
mdfind 'search phrase' -0|xargs -0 ls -l
sudo mdfind com_apple_backup_excludeItem=com.apple.backupd # find files excluded from Time Machine backups
mdls -rn kMDItemLastUsedDate -n kMDItemFSName *|tr \\0 \\n|paste - -|sort # sort by date last opened
mdls -rn kMDItemCFBundleIdentifier "$(mdfind 'kMDItemContentType=com.apple.application-bundle&&kMDItemFSName=iTunes.app'|head -n1)"
mdls -n kMDItemURL $(mdfind 'search phrase' -onlyin Library/Caches/Metadata/Safari/History)|cut -d\" -f2
for f in *.pdf;do echo $(mdls -rn kMDItemNumberOfPages "$f") "$f";done
for f in *.txt;do echo $(mdls -rn kMDItemUseCount "$f") "$f";done|sort -n # sort files by times opened
touch /tmp/a.tex;mdls -n kMDItemContentTypeTree /tmp/a.tex # show the UTI and parent UTIs for the .tex extension
```
  




