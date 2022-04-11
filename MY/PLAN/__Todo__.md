- [ ] Dave, Dad, Sammie gifts

THIS WEEK'S TODOS
- [ ] High output management summary
- [ ] Vijaya Response
- [ ] yearbook
- [ ] 
- [ ] fix microwave
- [x] pay ticket
- [x] Order Groc
- [x] Schedule stamper  VSP
- [x] Get QL UX person ASAP
- [x] BoA overpay; 800-669-6607
- [x] get codility stuff out
- [x] BoA overpay; 800-669-6607
















- [ ] Reddick Linked-in updates
- [ ] - [ ] designing life - VC story







- [x] Talk with Phil about Ross Hatton
- [x] intro hatton

- [x] Get meeting with Brian.



- [ ] .
- [ ] .
- [ ] .
- [ ] .
- [ ] Dan Fine
- [ ] look at drive - get hiring tests!
- [ ] passport
- [ ] Rocket Lawyer
- [ ] Get AF accounts cleaned
- [ ] CALLS: call Facebook AI guy; QL BF; l
- [ ] 


## Followups

Missy @ Morgen Thalen  (same place as Mark goins)
- invests in wepay.  hey aren't they already doing this

Alex Bangash -- Trusted Insights (startup)
- will introduce to Eric at Tiny prints

Laurence Marks -- ChoozOn 
- invests in 


............................................................................................

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

# misc
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

## notes
   right@name.com,  wrong/name,  cool@stuff.com,, xxx %#$#, @#45, dan@gmail.com
 

# Before Release
  - Make main server reset automatically and run on boot!

# Timeline

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

# Bugs
  - Front page: signup goes back to front page
  - FP: Facebook connect connection
  - FP: ???  Should we be doing very flexible URL matching?
  - Group Details: If you get a merchant error (on FP), then fix it.  
    the next page still reports merchant not found
  - GD:  Reply deadline field generates error even when date is selected
  - When following a link we should reset the 
  
# Done/Doing
- Add AZAT comments everywhere
- Try to comment parts of whole system


# Top of Heap
## justs
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



## Change passwords
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

# MVP2 Tasks
  https://docs.google.com/document/d/1faaS7DF0CD0-J-bEpV55lvfDfUvTcA0zpd5NSpQj65Q/edit?hl=en_US
## Older MVP NOTES

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

