.[[DocObsidian]].
  ,   [[Dataview]],
  , [DocObsidian Note](spot://docobsidiannote), 


:: [[mdfind]]


# LINKS 



# PLUGINS
 - PLUGINS:     [mdfind](__mdfind__.md) 
## Quick Switcher

- Selects all entries fuzzy matching prompt string
- Sorts by length, then 

## Obsidian Publish
-  [Oblio](https://publish.obsidian.md/oblio)  (LMI!) 
## Templater
    <%* const fileName = await tp.system.prompt("File name") 
            const templateName = await tp.system.suggester(["Template1", "Template2"], ["Template1", "Template2"]) 
            tp.file.create_new(tp.file.find_tfile(templateName), fileName, tp.file.folder()) 
    %>
[Docs](https://silentvoid13.github.io/Templater/)  

https://forum.obsidian.md/t/create-a-note-and-call-a-template-in-one-step-no-user-function-or-3rd-party-code-editor/26160/6




## USING CSS
[meta post](https://forum.obsidian.md/t/meta-post-common-css-hacks/1978)
## DataView
[[Dataview]],
[Snippets](https://forum.obsidian.md/t/dataview-plugin-snippet-showcase/13673)
[showcase](https://forum.obsidian.md/t/dataviewjs-snippet-showcase/17847)
# TOPICS
## CSS SETTINGS

- [Minimal Guide](https://minimal.guide/Plugins/Style+Settings) - 
- 
## DEV LINKS
- [API Docs](https://marcus.se.net/obsidian-plugin-docs/api),  

## = MY TWEAKS =
### - H2 colored Blue
- Configured the minimal theme for blue H2 headers.
- Prefs -> Community plugs (on left panel) -> Style Settings -> Minimal -> Headers -> H2 Settings -> H2 color  ! "blue color area"
### - CSS Tweaks
- obsidian -> pref -> appearance -> CSS Tweaks
	- Dashboard Plus Plus
	- 
### - Keyboard Maestro Safari URL grabber




Title:  Hyper Optimized URL grabber (Using Keyboard Maestro on the Mac)


Grabbing multiple URLs from my web browser into markdown involved tedious back and forth flipping between two apps. This sucks since I often like to use markdown to record some info exploration, since I can easily include commentary with the links.

I called this “hyper optimized” since one could save 10 web tabs with 20 key strokes, and ten text selections in order to pick the ten titles for the links. I don’t know of any other method as streamlined.

Here is an example: Hit HYPER-L and this macro will create a new link to this page’s URL using the highlighted text as it title. One keystroke and you are done!



Here are the detailed mechanics:

SETUP: The macro assumes the obsidian’s cursor is at a good place for the next URL link to be added (though the window need not be in focus). It assumes the web browser it open to the URL to be captured.

TRIGGER: The user hits control-opt-command L to capture.

TITLE: If there is an active selection then it is copied and used as the title of the markdown link (often there is good text on the page to use as a title). Otherwise the user is prompted for a title, and some optional notes to append after the link.

FLIP-FLIP: The macro then focus onto Obsidian, pastes the link and notes, pauses for a moment so you can see what you just did to your Obsidian document, then it flips back to the browser. There you can close that tab, and you are ready to capture your next web link whenever you find it.

It is a super simple macro, but it is an ABSOLUTE JOY to use if you like to capture notes on your web explorations… getting rid of a tedious bit of hoo-haa. Included here is the gist with the macro, you can upload if you like it. I also included screen shots if you prefer to just manually create the macro… since it is pretty short. If you use Chrome instead of safari, then just replace %SafariURL% with %ChromeURL%

[GIST](https://gist.github.com/oblinger/742c5b1b3b99ad020f0c301abdc62811)

Cheers,  
Dan O



# REF
## = GENERAL DOCS
I don't know exactly what your use case is, but here are some good references.

Dataviewjs docs: [https://blacksmithgu.github.io/obsidian-dataview/api/code-reference/](https://blacksmithgu.github.io/obsidian-dataview/api/code-reference/)  
Dataviewjs examples: [https://forum.obsidian.md/t/dataviewjs-snippet-showcase/17847](https://forum.obsidian.md/t/dataviewjs-snippet-showcase/17847)

Templater docs (including examples): [https://silentvoid13.github.io/Templater/introduction.html](https://silentvoid13.github.io/Templater/introduction.html)

Obsidian plugin API docs: [https://github.com/obsidianmd/obsidian-api](https://github.com/obsidianmd/obsidian-api)  
Example plugin: [https://github.com/obsidianmd/obsidian-sample-plugin](https://github.com/obsidianmd/obsidian-sample-plugin)  
In general, I recommend taking a look at the "Advanced Topics": [https://help.obsidian.md/Obsidian/Index](https://help.obsidian.md/Obsidian/Index)

Hope that helps!
## = CODING IT
### - Writing a Plugin
https://phibr0.medium.com/how-to-create-your-own-obsidian-plugin-53f2d5d44046


## === IDEAS ===
[Forum Share&Showcase](https://forum.obsidian.md/c/share-showcase/9)  
## Workspaces
[video](https://www.youtube.com/watch?v=eOcumDChzEQ)
# LOG 

