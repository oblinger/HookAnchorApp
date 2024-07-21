=[[Org]] 

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




# LOG

## jots
   git remote rm origin

chown -R user:group file
symbolize_keys!
  # ANDY says use this to do upgrades of NGINX   
  $ sudo wget http://nginx.org/download/nginx-1.0.11.tar.gz
~-~ 37signals.com/rm
number_to_currency  (in rDocs)   number_to_human  time_ago_in_words
app/login
  auto_login()  # log
what is the current request?  
  -  ActiveController::Base
 request.url  t

NGINX replace on the fly   http://wiki.nginx.org/CommandLine#Upgrading_To_a_New_Binary_On_The_Fly

no pass WD


&block

RAILS_ENV=staging      Rails.env == 'staging'

ActiveAdmin
<%= debug obj %>

http://ad
/app/admin   -- ActiveAdminGem


CMD-OPT-close

-- Quick  --  011 57 4 321 9759
--> add logic to compute all fields !!!!!
- move frank notes...
- passpack
$ $ git mergetool


simple unexpected concree credible emotional story smile
## BUGS
## PLAN
### PRIORITIES From Highest to lowest broken out by time periods
BEFORE DESIGNS (wed/thr)
-  Login support. 
-  Basic test suite

DESIGNS AVAILABLE  (12/22 - 12/26)
- Cut CSS/Javascript to franks level of satistaction
  - First build all pages to an acceptable level
  - Then polish details and corner cases until we no longer want to spend further money on polishing

MAJOR COMPONENTS (12/27-12/31)
- Finish all JS/CSS to a basic level of releasability
- Integrate all major compoenents (sendgrid, SSL, omniauth ...) except payment capture flow.
- Site functioning (e.g. cancel purchase functionality)
  (I expect to be doing much of this, but will love the speedup if your tasks
   get done.)

PAYMENT INTEGRATION (1/1- 1/5 or 1/7)
- Site functioning
- Testing  (we will be doing most of that in house, but best practice ideas always apprectiated)
- Payments  (I will be carrying most of the water on this part)

PRODUCTION ENVIRONMENT
- Production server build
- Final testing
- Security analysis (system level / DB level / Code level / UX level, etc.)

### Team priorities
-  Ready for demo day
--   Funnel planning.  
--   Front page docs.

- Secure relationship with 2-3 tier three merchant
  - Arrive at a contingent plan to integegrate that the UI level & BP level
- Customer Acquisition

### Larger tasks
D  F
   2  -- VIEW/COPY
1- 2  -- MAIL       -- going live
1+    -- PAYMENTS   --
1     -- with andy
3     -- CORE       -- 1-helpers/models  2-controllers
1+    -- DEPLOY     -- .5-capistrano, .5 hardening
1?    -- SSL
3 3   -- TEST/FIX   
2     -- MISC

1     -- ADMIN      -- Admin access
1     -- TEST SUITE    
1     -- Demo day prep (pitch prep)


#### Larger task details
-  Logins
   - basic logic and tracking
   - white list & WL input
   - new pages for signin signup, not-on white lists messages, etc.
   - Adminstrator access
   - Omniauth (FB-connect) from previous build
   - Sorcery 
-  Send Mail
   - Send logic
   - Integrating SMTP and Sendgrid
   - HTML & copy for the messages
-  Core
   - Model level validations & propagation of error messages
   - Testing:  clean up 50+ current errors
   - Testing:  1-2 validation checks on model integrity
-------
-  Dashboard
   - Logic for:  remove user, withdrawl, etc.
   - Screens for responses for each of the above
-  Payments
   - ...
-  Deployment environment
   
#### Timeline


S17     [D]   Prepare view layer and controller layer for Andres
U18     [DA]  Coordinate on Model View Controller layers & sendpage task
M19     [DA]  Coordiante & build sendpage 
        [D]   Add object creation methods to model layer 
T20     [D]   Transit   (coding model layer)
T20-    [A]   Client side validations
        [D]   Daily support
R22-    [A]   Cutting CSS, continuing client validations
        [D]   Daily support
               -->  if A gets done he can look at FB and email connects
M26     [D]   Transit  (coding parts of controller layer need by A's views)
T27     [D]   Integrating core functional pieces: 
              omni-auth, devise, samurai, SMTP, sendgrid, 
              paymant logic, etc
        [A]   coordinating w. Dan on this integration
        [D]   Finishing model level functionality
M01-R04 [D]   Writing controller/DB logic for all dashboard functions
              (remove user, cancel payment etc,
---- fully test parts of app that A has built

## Outsource
-  link to close facebox glassbox
## KILLER BUGS

- if user accounts exist, what happens to invite friends.
  VFY - email link is busted.  
- going to facebook will cause it to go to org signup.
- fix dates; and times
- get cust name into stripe system
- adds field specifying which payment processor is being used.
- admin passwordlocal

SOON:  What happens when accounts already exist, or FB is another account?

? if you go to click login (but then do the opposite kind of login)
  LATER: if user hits deny on FB auth --> just 'something went wrong'
 SOON: check all pages
SOON: dont we want to get names of invitees?
- only org can edit links, etc.
   - model validate 1 <= min < max
   -  **SOME** kind of explanation of the PBG service
   - images and quotes must be 'real'
   - first page min max calcs fail  (and checks for email sign fail)
   - agree to terms (on ccard page) ignored
   - splash page is wrong
     
     
LATER:  if you enters some email addresses, then connect to gmail, you loose the ones you typed.

## NOW small
- add columns to Transactions:
    user_id, details, amount_authorized  ???


QUESTION
DASH->cancel purchase ->cancel  FAILS
Button to delete user
A: routes names  new/create paths???
A: Share on Facebook, twitter, buttons  pay/merchantconfirm
A! After creat acct, back to signup.  after FB back to test page
A: forgot password, group purchase columns, Add admin boolean
A: model level validations (use locales for text), links directly to pages
A: routes (w. names)
A: ua associations in PU admin table
notes for build
-  SCRIPT  bundle install, rake db:reset, restart

-- After Andy:  @pu->@cpu
- go thru controllers file and convert strings to .yml
- add common pointer to beta.pay....
- @smd.to_best_name
- :broadcast = []

## NOW larger
-  WHOLE APP:   flow, function, payments

-- LOOSING ANDY --
-  PROD    production setup (sql, capistrano, env var telling production) 
           - cap: seeds, bundle install, rake db:migrate  (step by step script)
           - actually push to production
-  EXPLAN  Sorcery (FB connect, oauth2 too), tests, DB-schema
           -  UNDERSTAND:  (DB: mv_db, )   #!/bin/ruby  
           - why is our server so slow??
           -  # Is sanitize needed anymore???
-  SSL/CERT -- 
-  TST     - Model Validations, Tests

## Questions
Hide Status field
- branch prefix YM_; adding an issue; 

$ pull from master
$ deploy feature branch to test

- GIT diff (in textmate?)   history view   others?
  file merge, gitk, 

> viewer of history
> pull request
> viewer of diff
$ rake db:version
$ rake db:migrate:status

http://peepcode.com
http://pipcode.com/  login: leonel.alvarez1965@gmail.com
--> Meet Rails 3  --> advanced GIT  --> jQuery  --> rSpec --> adv cmd-line

password: leogladiador

http://github.com


http://guides.rubyonrails.org

A! rails icon on invite friends page
A! signup page two (always capture parameters there?)
adding to config
is test.paybygroup.com set up as staging???  homepage still routes to localhost
use tabs not spaces in textmate.  remove ^K

http://rubular.com/

## Current Tracking
### DAN TASKS
- if first/last then overrider full_name
- fix terminated != holdout  ////  potential_invitee? ~~ accepted_or_holdout?  
- Clean .js, 
- Client validations ???
- Validations (maybe client side too)
- Logging
#### Frank stuff
--BROADCAST--
user
accepted (all accepted invitees)
invited? (all invitees)
holdout? (all invitees that have either 'invited' or 'viewed' status)

--ID--       FROM-FIELD         TO-FIELD   REPLY-TO   CC
:from_info   info@pbg           -user-
:from_org    "-user-" <info@>   -xxxx-     <info@>

smd.broadcast = nil        |  :accepted?  |  :holdouts?  |  :accepted_or_holdout? 
smd.defaults  = :from_info |  :from_org
smd.subject   = ''


Other things

I also used some user account things like first name using the format @pud.ua.first_name to address the recipient of the email - is that correct?
For the org_account_created and inv_account_created emails, I included an @pud.ua.confirm_email_url variable. Let me know if that works or what to change it to.
If I do an href tag to link to a page like http://lets.paybygroup.com/invite_friends or http://lets.paybygroup.com/reserve_spot, should that work fine for the recipient and take them there if they're logged in or take them to a login page otherwise? If not, do we need to create versions of those links to put in emails?
In certain emails I reference an another invitee when sending to the organizer or all invitees, such as in invitee_signup_alert and uncommitted_invitee_signup_alert. For now I've just left those as all caps in the emails, awaiting your instructions on how to handle that. 
In others I reference the organizer, such as in inv_cc_auth. I left that one as all caps as well for now. If this is too much of a pain for MVP, let me know and I'll make it generic, but we think it will definitely help conversions.
In purch_complete_confirm there's supposed to be a final list of committed invitees. I wasn't sure how to reference that or if it would be feasible to for MVP, so an alternate, I just put "see dashboard" as a hyperlink. Let me know if there's a good way to reference the Paid In list.
In purch_complete_confirm I also have a place for us to include the confirmation details from the merchant themselves, such as a confirmation number, booking information, contact information, etc. - whatever they provide to us when we purchase on our user's behalf. For now should we just manually send out these emails in order to include that? How else can we handle that?
I wasn't sure what to use as the link for people to correct their payment info if their credit card fails when we try to capture the payment, so for now I have the link set up to go to the /reserve_spot page.
I need to know how to call a lightbox via a link in inv_ccfail_withdraw
We need to add a pud object for @pud.cancel_message. This will be capture in the lightbox when the organizer cancels. If we're not capturing anything in the lightbox for MVP, then all cancel messages will be purch_canceled_nomsg_alert. Once we have that, we'll need that object for purch_canceled_orgmsg_alert.





















### ANDY TASKS
- switch to explicit :pu_id in params.
- ViewLayer, Email, GetFB data,  Locic: ReqAccess(email|FB(getdata))
- SSL 
- mail addreess validate
- Do I need security.  sanitize, etc?
- Testing suite
- Gmail connect
- Logins -- Pull login into currnet flow:  FB-connect.  (either devise or samurai)
- contacts from gmail
- Admin level access

-  EMAIL ADDRESS: no spaces, rich format, client validation, what happens when send fails?
-  CAPISTRANO:    Capistrano recipe: https://github.com/andmej/andr.esmejia/blob/rails3/config/deploy.rb 
-  FACEBOOK DATA collect

## OTHER TASKS
-- Soon
- SSL cert 
- design the 'sign-in' link from the homepage 

-- Before Release
++ Solution for closed beta
   - pages for entering whitelist emails
   - logic to deny access everywhere
   - extra pages when user is denied
+ admin screens for all new tables
+ fix URL route names over all pages in app.
- logic to handle both white and black list merchants
- Default for Rails is to use cookies for 'session' ??? !!!!  FIX THIS !!!
+ Setup MY_SQL in both test and production
+ modest server side input field scrubbing across whole application
- protect from forgery CSRF attack protection

- handle logic if org deletes user.


-- After Release
+ full server side input field scrubbing across whole application
- have old values 'timeout' if user comes back to finish signup much later.

-- Later
- FB connect to send messages
- FB connect to get contacts 

  
### Task descriptions

LOGIN SUPPORT
- Leverage existing omni-auth FB connect
  - this should be loaded and configured, basically doing a get against /auth/facbook
    will trigger FB connect, and then the gem will respond with either a get to /auth/failure or a URL that we specify
    You can see that integration in the controller/homepage.rb the views for that controller have been moved to 'old' in the top folder
    (I warn you, you should be sitting down before you open up that controller file.  :-)
- Spend <=3 hours attempting to integrate sorcery
  - It sounded like you had also heard good things about sorcery, so that
    is a great choice.  But I don't want to constrain you.  We are looking
    for the best solution that does not constrain our UX flow since we
    expect to do logins in non-standard order and non-standard ways
    (e.g. getting the user name in one spot and the PW in another)
  - We will re-evalutate our use of devise/sorcery/etc. once you have
    taken the first step.  (e.g. if we are done or close to done, we will
    finish up this prefered approach, if it is taking longer, we may stick
    with the devise as it is working now.

- You should feel free to adapt the model and controller level as needed for this
  my sense is it will be a few columns added... better for me to not be in your dev
  loop on these details... just do it, and we can talk about it afterwards.

- I was imaginging that we would manage login state by simply having 
  a session variable :user_accound_id being set when the user has authenticated
  as well as checking login state from sorcery.
  then just writing our own before filter to perform login redirects if they
  jump back to any page where authentication has not occured.

  (Right now that is not secure, as session seems to be tracked by client side
   cookes by default.  YUCK!   but we will fix that.)

  If you have other thoughts on this and/or additional security thoughts let me know
  Again if it is quck just implement these things I can easily help w. these
  but they are so easy that coordination is probably not worth it.


- Summary of the login flow  (FRANK:  please look at this and clarify or correct as needed)

(1) At the end of the fisrt page of the organizer's signup the org will
    past form data back from first page and then be directed to either
        FB connect or enter email/pass signup.
    The result of these actions need to update session[:purchase_user+id]
    object to add an appropriate new UserAccount object, and set session
    and sorcery login as needed.

(2) All pages will have a before filter that reidrects to a signin page
    which should offer both facebook and email signin.

(3) Invitees are prompted to provide email/pass when they commit to paying into
    the system.  (FRANK, note right now an invitee cannot create an account w/o that)


We also have a 'whitelist' of email addresses that are allow to be organizers.
I will describe that in person. 
 




LATER:  move payments code out of source tree.
LATER:  git ignore payments secret keys

## misc
	beta to production
	check for cashing 
	- just do JSON/YAML logging
	I suggest
	1. we have server running under www user instead of franklangston
	2. have webdev user unstead of franklangston
	3. whitelist ports in IPTABLES
	4. close other loose end in default server configuration.
	5. Create separate server for DB
	6. set up load balancer for app/front end servers
	Any thing else?

### notes
	   right@name.com,  wrong/name,  cool@stuff.com,, xxx %#$#, @#45, dan@gmail.com
 

== Before Release
  - Make main server reset automatically and run on boot!

== Timeline

Su-18  D
Mo-19
Tu-20
We-21
Th-22




  1 refactor
  1 model layer
  3 org pg-1 pg-2 pg-3
  2 dashboard

  2 send email
  1 sandboxed payment integration & testing
  

  h link routing logic
  h invitee payments
  1 invitee dashboard

= Bugs
  - Front page: signup goes back to front page
  - FP: Facebook connect connection
  - FP: ???  Should we be doing very flexible URL matching?
  - Group Details: If you get a merchant error (on FP), then fix it.  
    the next page still reports merchant not found
  - GD:  Reply deadline field generates error even when date is selected
  - When following a link we should reset the 
  
= Done/Doing
- Add AZAT comments everywhere
- Try to comment parts of whole system


## Top of Heap
### justs
	   - Thursday Dec-8th
	     - wanted to go home rather than collaborate on payments integration.  (to work on it fresh next day)
	     - convinced to stay.  stayed from midnight-3:30.  effective collaboration on pay & mail
	     - Friday was supposed to go to Yoga, but left it to go to Google trip... then to Yoga later.
	       (when pushed said 'worked at google'  then 'well a little bit')
	     - back to office @7pm.  refused to work together... wanted to shop ... wanted to work
	       at home later alone.  refused multiple offers to co-work.  Dan offered to work anytime or anyplace.

	   - Within first two weeks at 500
	     actively withheld information.  (e.g. Dan and Azat were both trying to understand how to get
	     connected to the rackspace instances (I think that was the task)).  Dan looked over and realized
	     Azat was already interacting w. Rackspace.  When questioned Azat said,   "oh yes i figured that
	     out long ago."  There was not even a pretense that Azat did not know Dan was actively spending
	     time trying to find what he had already found.  
	   - On many occasions Dan would request to ask/tell Azat something.  Though sitting 2 feet away,
	     he would refuse to speak becuase he was busy.  Repeated requests would get the same response
	     for more than an hour on some occasions, even for simple single sentence queries.

	   - About 1 month into 500
	     - The team had agreed to launch a time-critical testing page.
	     - Within the team this was his task to accomplish, but at as night drew in he declared
	       "I am going home, you can finish it, here is the password to the server, you can 
	        finish it."  Dan stayed and got the server running.

	   - During week of thanksgiving
	     - Azat asserted that 'we have come along way in a short time'  
	       'we have alot to show for the time we have been working'  and
	       'you [Dan] have contributed a lot'
	       'he then asked what Dan planed on working in the next week (while at home and when he came back)'
	     - During the teams absence, Azat created many tables that duplicated existing functionality
	       and then connected these tables to the front end flow he was supposed to be building.  
	     - The aparent goal of the act was made explicit in the next week when Azat declared,
	       that he did not want to build from the backend that Dan had built, but rather wanted to 
	       control all layers of the application we were building (the model layer, the control layer and the 
	       view layer.)  
	     - This was his unilateral way of achieving that goal.

	   - In the week after thanksgiving
	     - Azat declared that Dan's development skills "were a joke."   "You are a good manager,
	       so you should be a manager, but your not a programmer, so I don't know why you are 
	       trying so hard to be one.  You should just leave it to me."  To support this
	       he retold the story when they first met, where Dan did not even know what a MVC
	       (model-view-controller) was.  
     
	     - 'i will consider it' was Azat's response several times when Dan (the CTO) requested
	       functional features or design choices in the code  (e.g. merchant integration)
	     - 3+ hours on multiple occasions over a couple of days were taken up by debates over the
	       best approach to represent PBG data.  (Designing these representations had been
	       the task of Dan, but by unilateral action Azat brough already resolved (and partially
	       implemented) solutions back into consideration.

	     - Becuase Azat was the only team member with CSS/Javascript skills he was 
	       necessary for the development of the needed functionality.
	     - Dan Frank and Azat collectively discussed the need for Dan to contribute to this effort
	       for speed sake.
	     - Azat make it clear that he was only interested in development where he control all layers of the
	       development, and that Dan could "work on wireframes" or seperable components like payment processing.

	     [Frank in your words you can describe the occasions where Azat say long term 
	        he could not work with/for Dan]


	   - Two weeks after thanksgiving.
	     - Dan initiated a discussion about a respect of each others skills
	     - Azat declined repeated attempts to talk about specfic background and skills
	     - He did declare that is was obvious to him by observation that Dan did not have
	       effective skills, and that Azat was stronger in all relevant areas.
	     - Frank repeatedly asked if Azat thought there were some areas where Dan had skills to
	       contribute.  Finally Azat conceeded that there must be something, but Frank never succeeded
	       in getting Azat to articulate such a skill.
	     [In Dan's opinion of the interaction, Azat showed an absolute contempt for Dan's technical
	      abilities across the board, and a firmly stated resolution to *not* work with Dan, in a way 
	      that shares any collaborate ownership of the development process.  The only option Azat was
	      open to, was one where Dan produced stand alone components that Azat absorbed into an 
	      application that he had sole control over.]



### Change passwords
	   - we should do *reset* password and ensure the request only goes to us
	   - FB.   (we moved the homepage right?)
	     remove as a 'developer'
	   - github, rackspace, aws
	   - rackspace instances
	   - pass pack

	- F:samurai --  PCI Self Assessment Questionnaire (SAQ) Version A. 
	- Merge DBs back together.  Make mock flow page 1, and invitee flow operate over it.
	- Fix invitee signup and GP dashboard
	- Front end for invitee setup
	- Front end for GP dashboard

## MVP2 Tasks
	  https://docs.google.com/document/d/1faaS7DF0CD0-J-bEpV55lvfDfUvTcA0zpd5NSpQj65Q/edit?hl=en_US
### Older MVP NOTES

	= END TASKS =
	  - clean DB fields
	= FINAL CHECKS =   (3 days after complete, before launch)
	  - test end to end system w. sandbox
	    (verification at GUI & DB level)
	  - test end to end system w. live transactions
	    (verification at GUI & DB level)
	  - DB backups tested

	= LAYOUT / FRONT END / FINISH FLOW =
	  - Pixel level cleaning
	  - Front end error checking
	  - Finish pay flow
	  - Ad-hoc per merchant field list

	= AUTH =
	  - integration of loaded Devise w. current flow
	  - signup ordering issue
	  - FACEBOOK CONNECT 
	= SEND-GRID INTEGRATION / Rails sendmail
	  - devise send mail
	  - html formatted send (in our code)

	= PAYMENT STUFF =
	  - Integration w. real merchant account.
	    - testing w. live transactions
	  - Integrate pay flow
	    - Test error code in finished flow
	  - Purchase time payment processing
	    - Test error codes
	  - Cancel activity ACTION



### CHUNCH

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

#### SPECIALTY
##### TESTING

	TO REBUILD ALL TEST MERCHANT ACCOUNTS
	- Logon to webmaster@paybygroup.com  goto ADMIN DASH  click "Create test merchants"

	TO LOGON TO THE DASH FOR A GIVEN TEST MERCHANT
	- Logon as info+test_XXX@paybygroup.com  pbgmerchant  this will get you to the merch dash for test merchant XXX



##### PBG_ON BOARD
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
 
##### MERCH DASH
	=== MERCH DASH ===   info+merchant@paybygroup.com  viewdemo
	M-LAND G1    http://g1.dev/merchant
	M-LAND LH    http://localhost:3000/merchant          webmaster@paybygroup.com + founder
	M-LAND L2    https://lets2.paybygroup.com/merchant   webmaster@paybygroup.com + founder  merchant@oblinger.us aaaaaa
	M-LAND PROD  https://lets.paybygroup.com/merchant   webmaster@paybygroup.com + founder
	EDIT   G1    http://g1.dev/obj/editor/new?form_id=msd_std_gp_form&merchant_id=independent_120601

##### POPUP LINKS
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



#### RLAC
	EG           https://lets2.paybygroup.com/merch/integrated/group_purchases/new?purchase_id=1517&merchant_id=RLAC_DEMO&example=rlac_eg1


### URLs / Links

#### OLDER -- error triggering links
	http://localhost:3000/merch/integrated/group_purchases/new?merchant_id=test_errors   ERR on LH
	http://dev2.paybygroup.com/merch/integrated/group_purchases/new?merchant_id=test_errors   ERR on D2
	https://lets2.paybygroup.com/merch/integrated/group_purchases/new?merchant_id=test_errors   ERR on L2
	https://lets.paybygroup.com/merch/integrated/group_purchases/new?merchant_id=test_errors   ERR on L2

#### Older shit info

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


#### Older shit


	PBG DEMO
	Here's the link to the demo we can run through on the call: https://lets.paybygroup.com/merch/integrated/group_purchases/new?merchant_id=rocky_mountain_demo_180912. 

	http://localhost:3000/merch/integrated/group_purchases/new?merchant_id=rocky_mountain_demo_180912. 

	http://genadi.rentlikeachampion.com

	curl -v http://genadi.rentlikeachampion.com/property_rentals/pbg_info?purchase_id=1519&token=37aa64bbc7741e5e40a3649e98d6b5384c04c111

	https://github.com/franklangston/gpay1/blob/dev/doc/UX_Flow_V1.md  

	rlac@paybygroup.com
	rlac@paybygroup.co m
	ruby -Itest test/unit/

### URLs -- Dev & Staging
	Dev2       ddddd
	Dev2       

	http://localhost:3000   MI on LH
	http://dev2.paybygroup.com   MI on D2
	https://lets2.paybygroup.com  MI  on L2
	https://lets.paybygroup.com  MI  on Production

### URLs
	cartoon  http://www.youtube.com/watch?v=qj8FQNwxqQU
	Loc Host   http://localhost:3000/merch/integrated/group_purchases/new?purchase_id=1517&merchant_id=RLAC&example=1    LOCALHOST
	Prod RLAC  http://lets.paybygroup.com/merch/integrated/group_purchases/new?purchase_id=1519&merchant_id=RLAC&token=37aa64bbc7741e5e40a3649e98d6b5384c04c111  w RLAC PROD 
	Prod E.g.  https://lets.paybygroup.com/merch/integrated/group_purchases/new?purchase_id=1517&merchant_id=RLAC&example=1     w/o RLAC  PROD
	Dev2 RLAC  http://dev2.paybygroup.com/merch/integrated/group_purchases/new?purchase_id=1519&merchant_id=RLAC&token=37aa64bbc7741e5e40a3649e98d6b5384c04c111   w RLAC DEV2
	Dev2 E.g.  http://dev2.paybygroup.com/merch/integrated/group_purchases/new?purchase_id=1517&merchant_id=RLAC&example=1  w/o RLAC  DEV2
	Flow Doc   https://github.com/franklangston/gpay1/blob/dev/doc/UX_Flow_V1.md


	PBG SPREADSHEET
	https://docs.google.com/spreadsheet/ccc?key=0AlqVwlvHlzDMdFpKSkhMQ3FkYUpPNWdIMm1qazAycFE&pli=1#gid=0




