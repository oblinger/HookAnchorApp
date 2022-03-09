
# Ubuntu Packages
  gconf-editor


apt-get autoremove

# Pricing model

PROBLEM:  allocating payments to those that 'draw users into group'

GOAL:
 - If user wants pkg X alone, it is always cheapest to simply buy pkg X
 

ASSUME:
- apps are in grouped into BUCKETS organzied by *ALTERNATIVES* for similar service


Payments

- Author gets 100% of money for a direct purchase.
- Customer gets 75% of the direct purchase price towards bucket dollars
- Bucket dollars defray the costs of all items in the bucket
- Customers that make subsequet purchases only pay the excess 
  (and the customer's bucket dollars are increased by  75% of that excess payment)
- Customer can 'buy the bucket' or 'by the bucket's core'
  either of these cause all or some pkgs to be 'free' after that




# Learning From Social Crew
## Important activities
### Customer Development Discussions
- Brainstorm questions, group them
- Trying to identify 'must have' capability


# Startup Strategies

## Product Building Testing
### Website testing
#### Traffic tester
- Optomizely                 -- Edit on page
- visual website optomizer   -- 
- google website optomizer   -- 

## Hiring
- 75-80K Engineers in the valley   (with equity)

## Investment Rounds
- Lose 10-20% equity for 12-18 months or runway.

### Angel Strategies
- Advisory rounds:   Raising 150-400K on a 3M-4M Cap

- "So who else are you talking to?"
  "You know, the the usual suspects.  
   But I don't want to jinks these deals before they are final ."
- "Can I count you in?"   "Well how much of the round do you want?"
- "We are raising XXX on YYY cap?"  they say "Oh too much.. grumble"
  *smile*, *silence*, let them settle.
  "Well we are looking for smart money, so if these terms cant work
   then give me a term sheet I can bring back to my co-founders"
  

 **** Tricks
- If you need to go down on terms (because of an awesome investor)
  you can add invetsor shares and make them a super angel.

- ASK --
  - know what your angel's expertise is
  - ask them CRISP strategies questions relevant to their expertise
  - then say.  let me show you are pitch to other angel's and see what 
    how to refine the XXX expertise part of it.
  - 





in wo yo uahave a limi
## Design

Building Empathy     http://www.slideshare.net/LauraKlein1/building-empathy
Remote User Testing  http://www.slideshare.net/LauraKlein1/who-do-i-talk-to-now-diy-user-research-for-startups?from=ss_embed
                     verifyapp.com, http://www.fieldtestapp.com,
                     reinvigorate.net for remote testing
		     http://www.invisionapp.com/ for interactive prototypes
In person testing    http://www.slideshare.net/rick/lean-ux-bootcamp-500-startups-intro-to-usability
 

# Dev Strategies

## Optomizing front end speed
### Tools 
- google Page speed 
- yslow

### Notes

Perception of load speed
  - 80-90
  - Dom Ready
  - On load


Strategies
(1) make thing smaller.
(2) get them closer 
(3) Load things smarter


==SMALLER==
- gzip  (use white list)
- JS min, CSS min, HTML (watch out for text areas)
- SHRINK IMAGES
  - lossy image
  - Strip hidden EXIF data
  - USE: jpegtran & pngcrush / smush.it
  - WebP  (really good but only supported by chrom & opera)


==CLOSER==
CDN
- Akamai
- Level3
- CloudFront is most startup friendly

!!!! Google CDN to host your js libraries


==SMARTER==
- cache-control header
  - set your headers to never expire
    - rename files when you change them
      - trick add last modify date of file to url.  e.g.  style.css?ts=131149627

- reduce HTTP requests
  - combine CSS & JS files
  - use image sprites
  - use data URI's for images
    - only for small images (not supported in all browswers)
  - keep an eye on 3rd party scripts
    - SCRIPT tags are synchronous, (add them dynamically so 
       (google: javascript asynch snipit)
    - don't let them load in HEAD
    (test effect these 3rd party scripts if they go RENDER)


- down FASTER
  - add with / height attr to all IMG tags  (will not reflow page)
  - put CSS in head.  (allows for progressive rendering)
  - put JS at bottom. (because they lock system)
  - Web fonts are terrible for performance  (often 1+ sec to load)
    (google fonts are faster)

TRICKS
- use multiple domains for parallel download (browers will limit by domain)
- minimize use of iframes & cookies
- key eye on 3rd party scripts
- Defer JavaScripts  (wait 'tll on-load or on-trigger)
- Avoid 404's and redirects


====== MOBILE ======
- Use local storage on mobile
  - get 5MB per domain (of dedicated space)
  - reduce load times by downloading entire webite at once (then track additional info needed & add it)

RESOURCES
- WebpageTest 
- YSlow and PageSpeed
- stevesouders.com      # Guru on speed
- torbit.com


# Design Strategies
## White space

- more white space aroud something makes it feel more important/luxurious

- RYTHEM (vertical and horizontal grid lines that info falls onto)
  - CLEAN DESIGN ~~ constant rythem, much aligning on fewer grid lines

- WHAT THE EYE SEES
  - GROUP BY  (similarity, proximity, 
  - EYEs 'close' many patterns

- How to pick margins
  - scale along geometric lines from the outside box.



- Text
  - Recommend: 1.3-1.5 em  == 130% -- 150%  (line height / font height)  Vertical
  - Recommend: 0.0-0.4 em  horizontal kerning
  - Recommend: paragraph  0.75-1.0 em   list item  .5-.8 em

