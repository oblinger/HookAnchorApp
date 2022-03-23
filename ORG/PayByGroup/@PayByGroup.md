 [[Corp]] 

# MEETINGS

### m2011-00-00 - CureBit -- they connected to many shopping carts

- Magento -- Schema 
  Allen Storm -- high paid consultant 
- shopify -- extremely simple
- Open cart -- 

- REST api -- 


- coding calling source

- use 100% coupon
- look at bill me later


### m2011-00-00 - Docomo
"Masa" Sumita
Rakesh Sood -- 
Manabu 

### m2011-00-00 - Borski

#### Vulnerabilities
- SQL injection --
  --> user ORM layer
  --> if we build SQL string use prepared statements
- XSS -- Cross Site Scrpting (XSS) --
  --> All user enterable data needs to be (sanitized?)
      use whitelist
- CSRF -- Cross Site Request Forgery
  --> preventForgery (in rails)
  --> Have confirm dialog when the actual money is going
- Dont use GET for changing user/server state
  -- browser will gen message if user refreshes page and does duplicate get
  -- crawlers will not crawl it
  -- parameters are in body of request not in URL
- MSGS -- Error messages should be turned off
- COOKIES --
  --> use non-persistent cookies when possible
  --> don't put sensitive data in the cookie
- php server

### m2011-00-00 - Giacomo

Ideas:
- Don't auth the card at invitee signup.  Instead just check funds available.
  then at payment time, veirfy funds immediately before pulling all funds.


==USAGE CASES==
- Org wants to throw in $$ for deposit
  - Org wants to split 
- Org wants to throw in extra $$ at payment time
- Invitee's card will not support full amount at payment time 
  - Later card will support full payment & org wants to resplit 'correctly'
- Different value to different invitees, so different amounts paid





==GP PHASES==
  CONCEPTION    --
  INITIATION    -- Organizer gets to a 'price page' for a merchandise item.
  RESERVATION   -- Merchant removes item temporarily from available inventory for others.
  PAYMEMT       -- Payment in full is made to the merchant.
  ADJUSTMENT    -- After payment time when amounts paid by each invitee are updated
                  (in a way that keeps payment amount appropriate for existing purchase)
  REFUND        -- 
  FINALIZATION  -- 

==DATA==
Invitee Parameters
  MaxAmount  -- total (including deposit) that invitee has agreed they would be willing to pay for trip
  MaxDeposit -- amount invitee has agreed to allow the organizer to use in paying for a deposit.
                (invitee may loose that money if purchase is not made)
  AmountPaid -- amount already charged against invitees card

GP Parameters
  DepositRequired  -- Per person deposit being required 
  DepositRequest   -- Per person deposit being requested




==LOGICAL PAYMENT STEPS==

AT INDEPENDENT CONCEPTION
- Organizer initiates GP w/o a specific inventory item
- Group details are captured.


AT INITIATION
- Organizer executes checkout flow upto 'price page'
- We capture purchase details.
- Organizer specifies group details.
- We send invitation to inivitees.
- (OPTIONAL)  We may perform reservation in this step


AT RESERVATION TIME
- Organzier executes checkout flow, merchant marks inventory as 'paidbygroup'
- Deposit may be paid  (as organizer dictates from self and/or any authorized deposits)
  - Allow organizer to 'ADD/REMOVE/CHANGE' subtotal per invitee (including self).  
  - Allow organizer to 'SPLIT_REMAINING_AMOUNT_EVENLY' 
    (applies to all invitees currently paying zero, who have authorized deposits)
  - Allow organizer to confirm breakdown selected.

AT PAYMENT
- Organizer commits group to purchase
- All invitee cards are checked for available credit
- Organizer specifies purchase split
  (Below amount set against any card must be less than amt available & amt agreed to by invitee)
  - Organizer may 'SPECIFY_SPECIFIC_AMOUNT' for a specific invitee (including self)
  - Organizer may 'SPLIT_REMAINING_AMOUNT_EVENLY' 
    (applies to all accepting invitees that are currently have no amount assigned)
  - Organizer may 'CONFIRM' split once remaining balance is zero
- Specified payment flows are executed.  First from invitees to PBG, then from PBG to merchant.
- Record of all transactions paid are sent to all invitees.  (the purchase is on!)

AT ADJUSTMENT
- Organizer re-specifies purchase split
  - Organizer may 'SPLIT_FULL_AMOUNT_EVENTLY'.
    Requests optionally zero amount to 'hold out', then ignores current payments and sets each invitees amounts equally
  - Organizer may employ all PAYMENT time splitting functions as well
- Adjusted payment increases are processed for all invitees
- (OPTIONAL) refund may be executed at this time.
- NOTE:  PBG balance is guanrateed to either be positive or zero at completion

AT REFUND
- Payment refunds are executed (also to invitees kicked out of the group)
- NOTE: PBG balance is guaranteed to be zero


AT FINALIZATION
- Typically exeucted automatically at purchase usage time
- If needed 'REFUND' is executed.
- PBG balance is guaranteed to be zero
- No further adjustments are possible










### m2011-00-00 - OLDER

#### Ways to scrape
SCRAPE



Approaches
(1) scraping   reading/twiddling
(2) turking
(3) partnering

Laws
- Tresspass by chentel
  - what is the problem being solved
    server resources.  loss of creative content


Tips
- mobile access is easier
- Tools:  
   php:libcurl+xpath_expr, libpath, 
       tamperData
   python: beautiful soup (maybe ruby), scrapie



- PayByGroup merchant generator
  - generates HTML button for specific merchant
  - Guaranteed to work, cause it opens new
    window (like facebook)
  - PARAMETERS:
    - Constant value parameters are set at
      button generation time 
      (look & feel of button, maybe text)
    - Secure parameters are passed by AJAX call
      from their server side to our server.
    - Hidden parameters on page used to 
      programmatically control details of the signup

    - look/feel

### m2011-00-00 - STARTUP MARKETING -- Jared from Away Find
    http://slideshare.net/technotheory  "Startup Marketing"

- The more you ask from someone (upto a point) 
  the *more* they become vested in your success.  USE THIS

- Just showing up 
  - Events
  - Shared-interest organization
  - Forums
- Being a thought leader --- Write/speak/participate
- Email folks

- Do something
  - send links, send tips, send surveys, send tiny bits of news
  - Be personal  "high steve"   be frequent  2-weeks, 2-months

BLOG
- have one.

EMAIL
- send it directly (forget sending to a list)

MARKETING VS DISTRIBUTION
- consider ways you can make others use

A FEW SEM TIPS
- negative words
- (see slides)


IDEAS
- Geeks on a plane
- 500 startup list
- inbox love pane ???
- Hacker news, AVC, Twitter


#### notes from azat
Jared
@techotheory

Goralnick
the more you ask the more people want to help (investors)
Most people don't SHOW UP!
Relationships are products of time
Build relationships
Showing up over time help to become a part of commuity
1)show up
2) be a thought leader: write, speak, participate
3) email them

Examples:
Inbox Love Panels
Geeks on a Plane
500 Startups List
Hacker News, AVC, Twitter, etc.

Think of getting other people on stage
 What is highrise? CRM! 
User List,
Blog or Newsletter lists
Events RSVPs
LinkedIn
Facebook and Twitter
CRM

Report it

Do Something with lists:
Send links
Send tips
Send surveys
Send a tiny bit of news

Be personal: "Hi Steve,"
Be Frequent: 2wks-2mo
Never know what's going to happen (email sign up list, survey)
Startups are not doing well, direct marketers do realy well!!! It seems redicolous but they much more money then startups! You have to stay infron of your customers, email them. Say something to them.
Get them used to hearing from you. Keep contacts warm
AWeber, Blue Sky Facttory
Drip email sequence=A-OK
Delayd emails, notificatons
Love or hate us is better than keep them pleased
Be okay with pissing people off.
You email them as much as possible but not more.
(stop once the effect drops)
Londer email are more effective. If people un subscribe that's okay cause they would not buy. Notifications are different - they might be annoying.

BLOG:
domain.com/blog - not subdomain or tumblr
People read blog not press releases
Don't sell (hard)
Be part of the conversation (other people's blog) - be thought leader.

HARO
Help a reporter - list of press leads
Habit Labs
bit.ly/jenlaunch
slideshare.net/technotheory
Marketing vs Distribution: cover on web page vs apple app store
 getjar.com
Google App Market Place

bit.le/piratesnew
5 step startup metrics model

KISS, Visual Web Site Optimizer, Google Website Optimizer

Away Find.com
Test everything even if it looks better/new/or Enrique told us :-)

SEM TIPS:
negative keywords
test LOTS of ads
use numbers
SEGMENT!
Conversion Optimizer (Google) after 2wks switch to customer acuqisition instead of Pay-Per-Click

App Sumo

Don't forget:
It's all people
You must give
Paresoma - co-working space in SF
Promote others!!!
jared@awayfind.com
@technotheory

### m2011-10-29 -- SEO. 

THREE PILLARS
(1) On page optimization
    - Stuff to read: SEO training presentation,  moz, beginner guide to SEO, 
    - SEO moz blog.
(2) Links
    - Court link rich folks, journalist, bloggers, ...
    - You need to provide value to the linker
      TRICK: put open source software, presentations, etc. on *your* web-site
      Use a reverse proxy to put virtually move your stuff to your site.
    - Nepotistic linking (simply ask closer connections)
    - Look for bloggers that are a bit above your level (but not alot)
      - complement on a post of theirs, then tell them you have something 
        that might find interest.
    - Find content creators that are already creating 
(3) Building Content


How to find good keywords for your page:
- google ad-words keywords too
  --> you put in a seed list of keywords --> it expands the send
- Spinning: "mad libs"  generate pattern phrase  XXX that provides research for YYY


Looking at your site
- how many steps to your checkout cart?

Type of content on your page
- Pillar content -- super polished (400-800 words) -- gets influential people to link to it.
- Biller content -- solid but mostly just functional.

- listen to the _exact_ words your customers use when talking about their
  problem.  (use those words, it will help w. SEO!)


Information Architecture for your site
- Google site map protocol


TIPS
- Cute one off jokes in your page will give you personality, that will get traction
- check do they search for dress, or dresses more
- check do they search for 'vale'
- in google search you want to make sure more words of your title are shown
  (that will 
- google will use your meta description as the snipit text 
- don't split your juice across two pages  (use 301 'permanent' redirect)
- ALT tag  "dress rush for wedding dresses"
- keep CSS out of page.  both perf and tiny SEO delta


- High touch sales
- Low touch sales

- On page optimization



#### from azat
Japan incubator
bingo card generator
high touch saleamount - big amount
Low touch sale - small amount
1) on page optimization
Seomoz seo training presentation
it's important!
has somebody who is responsible for number of clicks to get to the cart, seo
2) trust flow over links. If the source is trustworthy :
Anchor text ranks, page and domain trustworthy 
get links from bloggers, journalists, programmers , etc not SMBs
DHH (wtfit?)
put your open source software on your werbsite!!! (Not git hub) same as presentation, blog (not tumbler)
subdomains different from domain for seo
301 redirect sometimes works
Its better to have domain/blog than blog.domain
getting smaller bloger is better cause odds are better!
don't buy lunks
Google will kill anyone of us unless they look stupid for their users because we are too big
The bigger the website is better!!! At least several thousands pages!!!
Scalable content generation - CMS
no reason to do this by yourself!
time of co-founders' time is very valuable. Hire it out to cheaper people!!!
content you create on your wrbsite stays forever - great ROI!!!
No UGC - B- content
random freelancers not a good idea
ehow sends 8$ to write and makes 40$ a year Of low quality content
Panda 2.0
Find experts in your niche : if somebody already blog about your subject then they will likely to do it for you for money
look credible! No typos, etc.
invest in making you self look like a big credible company you one day be
start talk like you customers talk to increase business. Don't talk like sales man
a
Seomoz - learn from them but dont use their tools
100$ for information: name, address, email, interests
Mechanical terms?
information architecture
site map
942 activities
Tree strructure
Google is a primarily navigation tool on the internet
project management for hospitals is how you use regular project management - information archetecture, niche pages
the less competition /long tail for trustworthy website 8h
cannot suggest links to google
google wrbmaster
sitelinks Depends on brand
qdd
Older stuff wins big time on ranks!
google switches to fresh mode and new websites get boost visits temporarily
back button is the biggest competitor
be nice, make friends even competitors
Concentrate on creating value for your customers
warm intro vs cold intro

Delisius produce benefit for business. Tweet is next to nothing. Blog is best!
more text on the home page as possible.  Don't hide from crawlers
have easy links to important things
jump links are great!
dont be afraid to use what works!
cheap is a bad word!
call to action with customer value "connect with FB to start planning groups activities "
okcupid blog
black cat
Viral queuz
long term park state in google
Wont rank
Google is sensitive to PR, not their support
Matt cutts support
its okay to put ny times logo if you were featured only in ny times blog 
seo is an aqusition channel
exact match domain bonus
Special niche  domains which has few pages and then redirects to main service

### m2011-10-20 -- MVP discussion w. frank


TESTS:
- [N1,N2,...]     Verticals.  will they convert on group pay (use it)
  <----    hard-coded for first vertical
           OO class structures beyond that
- [$]      Customer pays 0-5%  (txt changes)
- [LIFT]   % change when group pay is/is-not available for specific

- [PAIN]   Largest Pain Point -- 
  - Fronting $
  - Hassle
  - Timely Commit


- [C/O]    Invitee closed/open.
- [USAGE]  % that use group pay  (tested by dummy pages)





- We increase:
  - conversion rate.   (A/B testing on merchant (proxy) site)
  - 



### m2011-10-18 -- One line pitch
We're a group payments solution where the organizer is never on the hook for group purchases like vacation rentals, sporting events, concerts, and other purchase. We integrated directly with merchants as a checkout button. 

We are raising 200K on 4million dollar cap.
kickstarter for stuff

personal stories -- skii rental -- camping trips this summer.  nei




= Group Pay =                                 
We're a group payment solution where no single person is on the hook for group purchaces.    
   like vacation rentals, sporting events, concerts, etc.   
We integrated directly with merchants as a checkout button.

We're a group payment solution where the organizer never fronts money for purchases like
   vacation rentals, sporting events, concerts, etc.  






# CHUNCH

P-ON CREATE: http://lets2.paybygroup.com/merch/advanced/merchant_supplied_data/new?parent_id=msd_common_root    
  https://lets2.paybygroup.com/merch/demo/launch_page?merchant_id=vrma_demo3
DEMO LINKS   https://lets.paybygroup.com/merch/demo/launch_page?merchant_id=cabo http://g1.dev/merch/demo/launch_page?merchant_id=cabo  
MERCHANT     http://g1.dev/merchant     https://localhost:3000/merch/demo/launch_page?merchant_id=cabo_121212
MAIN SYS     http://localhost:3000/  https://

Editor       http://localhost:3000/obj/editor/

M ind        http://dev2.paybygroup.com/merch/integrated/group_purchases/new?purchase_id=1519&merchant_id=independent_120601
M Ind        http://localhost:3000/merch/integrated/group_purchases/new?purchase_id=1519&merchant_id=independent_120601
DelayedJobs  https://lets.paybygroup.com/delayed_job/overview

==BASE==
PROD         https://lets.paybygroup.com/
LETS2        https://lets2.paybygroup.com/
M-ind        http://g1.dev/    http://localhost:3000/

==PROD==   
M-LANDING    https://lets.paybygroup.com/merchant   webmaster@paybygroup.com + founders

Invite Link  http://dev2.paybygroup.com/dashboard/t303tj79g4u9qq8f
Impersonate  https://lets.paybygroup.com/admin/user_accounts/872/impersonate
==OTHER==
PBG SHEET    https://docs.google.com/spreadsheet/ccc?key=0AlqVwlvHlzDMdFpKSkhMQ3FkYUpPNWdIMm1qazAycFE&pli=1#gid=0
RLAC         http://rentlikeachampion.com    https://lets.paybygroup.com/merch/integrated/group_purchases/new?purchase_id=2300&merchant_id=RLAC&token=4bb34eb2b092f8c932872c6ff316c4c5a1b1f909
demo LINKS   https://lets.paybygroup.com/merch/demo/launch_page?merchant_id=demo_cabo 
RLAC demo    http://localhost:3000/merch/integrated/group_purchases/new?purchase_id=2300&merchant_id=RLAC_demo&token=4bb34eb2b092f8c932872c6ff316c4c5a1b1f909
LH RLAC      http://localhost:3000/merch/integrated/group_purchases/new?purchase_id=2300&example=1&merchant_id=RLAC_demo
PBG1DOC      http://pbg1docs.herokuapp.com/   https://github.com/franklangston/gpay1-docs/tree/master/contents  U:merchant P:docs
DOCS         https://github.com/franklangston/gpay1/tree/dev/doc/all_doc.md

### SPECIALTY
#### TESTING

TO REBUILD ALL TEST MERCHANT ACCOUNTS
- Logon to webmaster@paybygroup.com  goto ADMIN DASH  click "Create test merchants"

TO LOGON TO THE DASH FOR A GIVEN TEST MERCHANT
- Logon as info+test_XXX@paybygroup.com  pbgmerchant  this will get you to the merch dash for test merchant XXX



#### PBG_ON BOARD
P-ON CREATE: http://lets.paybygroup.com/merch/advanced/merchant_supplied_data/new?parent_id=msd_common_root    

P-ON CREATE: http://lets.paybygroup.com/merch/advanced/merchant_supplied_data/new?parent_id=msd_demo_excursionist   
P-ON CREATE: http://lets.paybygroup.com/merch/advanced/merchant_supplied_data/new?parent_id=msd_demo  
P-ON EDIT:   http://lets.paybygroup.com/merch/advanced/merchant_supplied_data/msd_rlac_demo/edit

     P-ON CREATE: http://lets2.paybygroup.com/merch/advanced/merchant_supplied_data/new?parent_id=msd_demo_excursionist   
P-ON CREATE: http://lets2.paybygroup.com/merch/advanced/merchant_supplied_data/new?parent_id=msd_demo  
P-ON CREATE: http://lets2.paybygroup.com/merch/advanced/merchant_supplied_data/new?parent_id=msd_common_root   
P-ON EDIT:   http://lets2.paybygroup.com/merch/advanced/merchant_supplied_data/msd_rlac_demo/edit

M-LAND G1    http://g1.dev/merch/advanced/group_purchase#new?merchant_id=
M-LAND LH    http://localhost:3000/merch/advanced/group_purchase#new?merchant_id=XXX
M-LAND L2    https://lets2.paybygroup.com/merch/advanced/group_purchase#new?merchant_id=XXX 
M-LAND PROD  https://lets.paybygroup.com/merch/advanced/group_purchase#new?merchant_id=XXX
 
#### MERCH DASH
=== MERCH DASH ===   info+merchant@paybygroup.com  viewdemo
M-LAND G1    http://g1.dev/merchant
M-LAND LH    http://localhost:3000/merchant          webmaster@paybygroup.com + founder
M-LAND L2    https://lets2.paybygroup.com/merchant   webmaster@paybygroup.com + founder  merchant@oblinger.us aaaaaa
M-LAND PROD  https://lets.paybygroup.com/merchant   webmaster@paybygroup.com + founder
EDIT   G1    http://g1.dev/obj/editor/new?form_id=msd_std_gp_form&merchant_id=independent_120601

#### POPUP LINKS
=== LH POPUP ==
CABO         http://localhost:3000/merch/demo/launch_page?merchant_id=cabo 
M-ind        http://localhost:3000/merch/demo/demo_page?merchant_id=demo_vrbo
nil          http://g1.dev/merch/demo/launch_page
M-ind        http://localhost:3000/merch/merch/demo/demo_page?merchant_id=demo_vrbo 
RLAC LIVE    <script src="https://lets.paybygroup.com/merch/integrated/bootstrap.js" type="text/javascript"></script>

=== LETS2 POPUP ==
CABO          https://lets2.paybygroup.com/merch/demo/launch_page?merchant_id=cabo  
VRBO          https://lets2.paybygroup.com/merch/demo/launch_page?merchant_id=demo_vrbo 
nil           https://lets2.paybygroup.com/merch/demo/launch_page 
M-ind         http://localhost:3000/merch/merch/demo/demo_page?merchant_id=demo_vrbo 
RLAC LIVE     <script src="https://lets.paybygroup.com/merch/integrated/bootstrap.js" type="text/javascript"></script>

=== PROD POPUP
FROM RLAC    https://lets.paybygroup.com/merch/integrated/group_purchases/new?purchase_id=2335&merchant_id=RLAC&token=30c2b00b41086dd8d08dcd70ecb49c56d6f679dc
FROM FRANK   https://lets.paybygroup.com/merch/demo/demo_page?merchant_id=demo_vrbo
CABO         https://lets.paybygroup.com/merch/demo/launch_page?merchant_id=cabo 
VRBO         https://lets.paybygroup.com/merch/demo/launch_page?merchant_id=demo_vrbo
nil          https://lets.paybygroup.com/merch/demo/launch_page



### RLAC
EG           https://lets2.paybygroup.com/merch/integrated/group_purchases/new?purchase_id=1517&merchant_id=RLAC_DEMO&example=rlac_eg1


## URLs / Links

### OLDER -- error triggering links
http://localhost:3000/merch/integrated/group_purchases/new?merchant_id=test_errors   ERR on LH
http://dev2.paybygroup.com/merch/integrated/group_purchases/new?merchant_id=test_errors   ERR on D2
https://lets2.paybygroup.com/merch/integrated/group_purchases/new?merchant_id=test_errors   ERR on L2
https://lets.paybygroup.com/merch/integrated/group_purchases/new?merchant_id=test_errors   ERR on L2

### Older shit info

http://lets2.paybygroup.com/merch/integrated/group_purchases/new?merchant_id=w_fixed_cost   MI on L2  FIXED COST
http://localhost:3000/merch/integrated/group_purchases/new?merchant_id=w_fixed_cost   MI on LH  FIXED COST

http://localhost:3000/merch/integrated/group_purchases/new?merchant_id=independent_120601    MI on LH
https://lets2.paybygroup.com   MI on Lets2

http://localhost:3000/merch/integrated/group_purchases/new?purchase_id=1905&merchant_id=RLAC&example=1   Example on LH

http://localhost:3000/merch/integrated/group_purchases/new?purchase_id=1905&merchant_id=moving_mountains_demo_120812    M-M on Localhost

https://lets2.paybygroup.com/merch/integrated/group_purchases/new?purchase_id=1905&merchant_id=moving_mountains_demo_120812    M-M on Lets2

https://lets2.paybygroup.com   MI on Lets2


https://lets.paybygroup.com/merch/integrated/group_purchases/new?merchant_id=test_errors   ERR on LETS
https://lets.paybygroup.com/merch/integrated/group_purchases/new?purchase_id=1905&merchant_id=RLAC_DEMO&token=e48bb048546b561f2e5246564b52379268a75729   RLACDEMO on LETS


https://lets2.paybygroup.com/merch/integrated/group_purchases/new?purchase_id=1517&merchant_id=RLAC_DEMO&example=1  RLAC DEMO L2
 https://lets2.paybygroup.com/merch/integrated/group_purchases/new?purchase_id=1517&merchant_id=RLAC&example=1  RLAC on L2


### Older shit


PBG DEMO
Here's the link to the demo we can run through on the call: https://lets.paybygroup.com/merch/integrated/group_purchases/new?merchant_id=rocky_mountain_demo_180912. 

http://localhost:3000/merch/integrated/group_purchases/new?merchant_id=rocky_mountain_demo_180912. 

http://genadi.rentlikeachampion.com

curl -v http://genadi.rentlikeachampion.com/property_rentals/pbg_info?purchase_id=1519&token=37aa64bbc7741e5e40a3649e98d6b5384c04c111

https://github.com/franklangston/gpay1/blob/dev/doc/UX_Flow_V1.md  

rlac@paybygroup.com
rlac@paybygroup.co m
ruby -Itest test/unit/

## URLs -- Dev & Staging
Dev2       ddddd
Dev2       

http://localhost:3000   MI on LH
http://dev2.paybygroup.com   MI on D2
https://lets2.paybygroup.com  MI  on L2
https://lets.paybygroup.com  MI  on Production

## URLs
cartoon  http://www.youtube.com/watch?v=qj8FQNwxqQU
Loc Host   http://localhost:3000/merch/integrated/group_purchases/new?purchase_id=1517&merchant_id=RLAC&example=1    LOCALHOST
Prod RLAC  http://lets.paybygroup.com/merch/integrated/group_purchases/new?purchase_id=1519&merchant_id=RLAC&token=37aa64bbc7741e5e40a3649e98d6b5384c04c111  w RLAC PROD 
Prod E.g.  https://lets.paybygroup.com/merch/integrated/group_purchases/new?purchase_id=1517&merchant_id=RLAC&example=1     w/o RLAC  PROD
Dev2 RLAC  http://dev2.paybygroup.com/merch/integrated/group_purchases/new?purchase_id=1519&merchant_id=RLAC&token=37aa64bbc7741e5e40a3649e98d6b5384c04c111   w RLAC DEV2
Dev2 E.g.  http://dev2.paybygroup.com/merch/integrated/group_purchases/new?purchase_id=1517&merchant_id=RLAC&example=1  w/o RLAC  DEV2
Flow Doc   https://github.com/franklangston/gpay1/blob/dev/doc/UX_Flow_V1.md


PBG SPREADSHEET
https://docs.google.com/spreadsheet/ccc?key=0AlqVwlvHlzDMdFpKSkhMQ3FkYUpPNWdIMm1qazAycFE&pli=1#gid=0




