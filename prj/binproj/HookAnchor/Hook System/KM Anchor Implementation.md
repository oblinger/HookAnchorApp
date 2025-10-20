


**ANCHOR** -- An anchor is a folder containing a Markdown file with the same name.
**ALIAS** -- An anchor is aliased if its markdown file begins "Alias [ [XXX] ]" this indicates the alias name for this anchor.  And in same folder there will be a markdown file XXX.md and as a peer to the folder there will be a sym link XXX that points to the anchor folder.
- Alias can be changed or removed by moving XXX.md to become the anchor markdown and removing the symlink.
- This ensures hard-coded paths never change.
- Aliases are often used in stream folders to provide temporary short names for items whose relevance will fade over time.
- 