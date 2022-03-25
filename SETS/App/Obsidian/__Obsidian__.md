
 [Kbd-Shortcuts](__KBD_Shortcuts__.md)  [Forum](https://forum.obsidian.md)   [Discord](http://app/discord)  [Oblio](https://publish.obsidian.md/oblio)  (letmein)  [Website](https://obsidian.md)   [support@obsidian.md](mailto:support@obsidian.md) 
 - Mermaid:   [Syntax](https://mermaid-js.github.io/mermaid/#/./n00b-syntaxReference)   [LiveEditor](https://mermaid-js.github.io/mermaid-live-editor/edit#pako:eNpVkE1qw0AMha8itGohvoAXhcROswm00Ow8WQiPnBma-WEsU4LHd--4aSHVSrz3PQlpxj5oxhoviaKBU6s8lNp2jUl2FEfjGarqJR9YwAXPtwy7p0OA0YQYrb883_ndCkEzH1eMQYz1n8vdan7yb54ztN2RooR4fnROXyHDvrPvpoz_75jEJfXaDVQPVPWUoKH0iEDWATRRhq3yq44bdJwcWV1OmldFoRh2rLAureaBpqsoVH4p6BQ1Ce-1lZCwrLiOvEGaJHzcfI-1pIn_oNZS-ZD7pZZvMgpjfg)    

# ### TODO ### 
- [ ] Add template for rocks (name & description inputs; sub-folder, with link)
- [ ] update kmt so .md will match without num prefix and with/without dunder
- [ ] Get GIT sync to the cloud
- [ ] Get Kbd Maestro sync working

# # TO FIGURE OUT # 
- [ ] Search by recency of edit


# ### REF ### 
## === MY HACKS ===
### --- Keyboard Maestro Safari URL grabber




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


## === IDEAS ===
[Forum Share&Showcase](https://forum.obsidian.md/c/share-showcase/9)  
## === BY PLUGIN ===
### CSS
[meta post](https://forum.obsidian.md/t/meta-post-common-css-hacks/1978)
### Templater
[Docs](https://silentvoid13.github.io/Templater/)  
### DataView
[Snippets](https://forum.obsidian.md/t/dataview-plugin-snippet-showcase/13673)
[showcase](https://forum.obsidian.md/t/dataviewjs-snippet-showcase/17847)
## Workspaces
[video](https://www.youtube.com/watch?v=eOcumDChzEQ)