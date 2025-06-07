lERROR: Cannot find cmd: spot:://AI MIN
  (should be compressing already???)

- spot: add 'main' for every anchor (if 'main' is not defined)
- Bug:  page w/ paren must not be printed as a wiki-link as paren cannot be parsed
- Spot: spgrab. foo!bar will not add a correct link to 'bar' in foo.  it adds a file link
- OBS - Evernote log of dated entries
    ingest all dated entries and starred entries 
    daily run script via chron or extend alarm.py
- fix history month
- sp grab and sp add should both add a link to clipboard
- LATER: Bulk rename group, move folder, move anchor
- MAYBE: auto scan for broken links in spot

### 2025-06-06  Anchor Folder support in spot
- Add a new command to spot called anchor, which sets up the environment for the named anchor

For Anchor folder FOO it will:
- Opens the anchor obs file FOO/FOO.md folder file is loaded if it exists.
- Opens the notion page named "FOO Shared" if it exists
- Opens the anchor folder if it has more than one file in it
- Opens pycharm if this is a Pycharm project
- Opens claude code if this is a Claude project


### 2024-06-17  ^newjump

- Multiple ALL_CAPS prefixes in the spot defs file indicates a "jump" table
- Scan KMR finds definition file (error if multiple found)
- Scan of Jump content indicates old jumps, deleted jumps (with given prefix), and new jumps to add at end.
- Should look for suffixes as well as full entries when looking at content.  So AMA Kindle or just Kindle.

- Figure out why we get repeated entries in the "::" sections of the doc

- spot grab 1pass mastadon. (got a crash)


- - spot: when folder is missing either delete command, or just change the group of the command to be "broken"
sss- spot: when folder is missing either delete command, or just change the group of the command to be "broken



.[[Spot]].   [Spot Folder](spot://SPOT~Folder) 
  , [[SP]], [[Spot Cmdr]] 




