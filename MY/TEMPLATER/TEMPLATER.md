.[[Templater]].
  [[NAMED sub note]]:
  ADDS:   [[Z TEST TEMPLATE]]















































:: [[QL template]]

:: [[" + title + "]]
< [[Templates]] 

:: [[AT Entry]],   [[DAY Entry]],   [[JOURNAL Entry]],   [[LOGGED sub note]],   [[MEETING Entry]],   [[NAMED sub note]],   [[NOTE entry]],   [[PRJ Entry]],   [[REF Entry]],   [[TEXT Entry]],   [[UNTYPED Entry]],   [[VC Template]],   [[WP Entry]],   [[Z Example Template]],   [[Z JOURNAL header]],   [[Z TEST TEMPLATE]],   [[indent IN]],   [[indent OUT]]
[[SUB Folder Note]] 

[[WEEKLY template]]


from AT template
<%-*
const name = await tp.system.prompt("Entity Name (w/o @)");
await tp.file.move("/AT/@" + name) 
-%>


<% tp.user.shell( {COMMAND: tp.file.selection()} ) %>


https://silentvoid13.github.io/Templater/internal-functions/internal-modules/file-module.html
-   [Documentation](https://silentvoid13.github.io/Templater/internal-functions/internal-modules/file-module.html#documentation)
    -   [`tp.file.content`](https://silentvoid13.github.io/Templater/internal-functions/internal-modules/file-module.html#tpfilecontent)
    -   [`tp.file.create_new(template: TFile ⎮ string, filename?: string, open_new: boolean = false, folder?: TFolder)`](https://silentvoid13.github.io/Templater/internal-functions/internal-modules/file-module.html#tpfilecreate_newtemplate-tfile--string-filename-string-open_new-boolean--false-folder-tfolder)
    -   [`tp.file.creation_date(format: string = "YYYY-MM-DD HH:mm")`](https://silentvoid13.github.io/Templater/internal-functions/internal-modules/file-module.html#tpfilecreation_dateformat-string--yyyy-mm-dd-hhmm)
    -   [`tp.file.cursor(order?: number)`](https://silentvoid13.github.io/Templater/internal-functions/internal-modules/file-module.html#tpfilecursororder-number)
    -   [`tp.file.cursor_append(content: string)`](https://silentvoid13.github.io/Templater/internal-functions/internal-modules/file-module.html#tpfilecursor_appendcontent-string)
    -   [`tp.file.exists(filename: string)`](https://silentvoid13.github.io/Templater/internal-functions/internal-modules/file-module.html#tpfileexistsfilename-string)
    -   [`tp.file.find_tfile(filename: string)`](https://silentvoid13.github.io/Templater/internal-functions/internal-modules/file-module.html#tpfilefind_tfilefilename-string)
    -   [`tp.file.folder(relative: boolean = false)`](https://silentvoid13.github.io/Templater/internal-functions/internal-modules/file-module.html#tpfilefolderrelative-boolean--false)
    -   [`tp.file.include(include_link: string ⎮ TFile)`](https://silentvoid13.github.io/Templater/internal-functions/internal-modules/file-module.html#tpfileincludeinclude_link-string--tfile)
    -   [`tp.file.last_modified_date(format: string = "YYYY-MM-DD HH:mm")`](https://silentvoid13.github.io/Templater/internal-functions/internal-modules/file-module.html#tpfilelast_modified_dateformat-string--yyyy-mm-dd-hhmm)
    -   [`tp.file.move(new_path: string, file_to_move?: TFile)`](https://silentvoid13.github.io/Templater/internal-functions/internal-modules/file-module.html#tpfilemovenew_path-string-file_to_move-tfile)
    -   [`tp.file.path(relative: boolean = false)`](https://silentvoid13.github.io/Templater/internal-functions/internal-modules/file-module.html#tpfilepathrelative-boolean--false)
    -   [`tp.file.rename(new_title: string)`](https://silentvoid13.github.io/Templater/internal-functions/internal-modules/file-module.html#tpfilerenamenew_title-string)
    -   [`tp.file.selection()`](https://silentvoid13.github.io/Templater/internal-functions/internal-modules/file-module.html#tpfileselection)
    -   [`tp.file.tags`](https://silentvoid13.github.io/Templater/internal-functions/internal-modules/file-module.html#tpfiletags)
    -   [`tp.file.title`](https://silentvoid13.github.io/Templater/internal-functions/internal-modules/file-module.html#tpfiletitle)
-   [Examples](https://silentvoid13.github.io/Templater/internal-functions/internal-modules/file-module.html#examples) 


<% tp.date.now("YYYY") + "-W" + tp.date.now("W", -7) %>]]

## "
