
[[PKM MyObsidian]]  [[PKM]]  .

### UNIFIED NAMESPACE

#### Hook Page

#### Switcher Presentation Ordering


- Case insensitive exact prefix matching has highest priority
- Followed by entries that contain the query string as a case-insensitive exact match
- Entries are alphabetically sorted

EXAMPLE results for "foo"  search:  

Foo							< prefix matches and it is shorter than all others
Foo Bar					< prefix matches space is before all other matches
Foo 0first				< prefix matches and zero is before all other non-space matches
FOOBAR          		< prefix matches, but lack of trailing space pushes this after all spaced entries
2023-01-01 Foo		< Contains match, but is not a prefix
z2011-00-00 Foo	< The Z can be used to force a dated entry later than others


### ALL CAPS

- Used to denote a namespace submenu
- Capitalization of a folder should always match the capitalization of its hook page

An ***all caps*** entry is a "hook of hooks", within the unified namespace it serves as a prefix for many other hooks thus becoming a menu of other hooks.

- For smaller menus we try to arrange each sub-menu item to have a unique starting letter, this allows one to navigate BOTH menu levels in an open loop fashion.

- In some sense, any hook with multiple named facets in already a name space menu, but we don't use all caps for hooks whose sub entries are mostly the universal hook facets (like folder, doc and website).  Instead ALL CAPS is reserved for hooks that have subhooks that themselves might have facets, there by being a full two-level menu.

