
## POUND PAY


require 'balanced'

Balanced.configure('YOUR_API_KEY')

Balanced::Marketplace.my_marketplace.create_merchant('drew@rentlikeachampion.com', '/v1/merchants/MR7y1KTy9uMG4cQiQ4mRI6KZ')

 


## STRIPE
- gem 'stripe'
- stripe initilizer


Dan: probably the best place to start is with our tutorial here:
 https://stripe.com/docs

for storing the card info, you'll probably want a
create_customer call here: https://stripe.com/docs/api#create_customer
Dan: and then for subsequent charging, you'll probably want the create_charge 
call here: https://stripe.com/docs/api#create_charge
Dan: we also have a sample rails app here:
 https://stripe.com/docs/examples




## OMNI AUTH
### SIMPLE OMNI AUTH
- ./Gemfile >> gem 'omniauth' \\ $ bundle install
- ./config/initializers/pbg_omniauth.rb
  ~~> need to register apps
- NOTE:  /auth/twitter


### OMNI AUTH + DEVISE
  (easier to sign in first)
- ./Gemfile >> gem 'omniauth' \\ $ bundle install
- ./config/initializers/pbg_omniauth.rb
  ~~> need to register apps
- NOTE:  /auth/twitter


## FACEBOOK
### FACEBOOK API

web:       : http://facebook.com/developer  (Use me

PayByGroup
APP ID     : facebookAppID='291176440907659'
SECRET     : 17ae6c88c5ab4ed00f7c4343ead96474
Dev Info   : https://developers.facebook.com/apps/291176440907659/summary?save=1
PAGE       : http://www.facebook.com/pages/PayByGroup/212989965433669

TestApp
App ID     : 286913981337732
App Secret : f6969c4c0c4c684f0bd493191674774c
App Info   : https://developers.facebook.com/apps/286913981337732/summary?save=1
             - enter your APP ID  - restart browser

CONSOLE    : https://developers.facebook.com/tools/console/
Permissions: http://developers.facebook.com/docs/reference/api/permissions/


DOCS       : http://developers.facebook.com/
SDK:       : http://developers.facebook.com/docs/reference/javascript/
STK EXCNG  : http://www.facebook.com/pages/Stack-Exchange/141752469180305
             http://stackexchange.com/
             http://facebook.stackoverflow.com/
DATA EXPLR : http://developers.facebook.com/tools/explorer/?method=GET&path=me%2Finterests
FL:  data we want:  http://developers.facebook.com/docs/reference/fql/user/
FL:  tutorial:      https://developers.facebook.com/docs/guides/web/  


### FACEBOOK API NOTES
FB.init({
     appId  : 'YOUR APP ID',
     status : true, // check login status
     cookie : true, // enable cookies to allow the server to access the session
     xfbml  : true, // parse XFBML
     channelUrl  : 'http://www.yourdomain.com/channel.html', // Custom channel URL
     oauth : true // enables OAuth 2.0 });

FB.api(path, "GET", {limit: 3}, function(r) { doit(r) })



## SAMURAI
### Account Info
  WEB:              https://samurai.feefighters.com/users/ea9abf40
  ACCT:             webmaster@paybygroup.com webmaster pass
  ACCT:             feefighters@oblinger.us W1
  File Number:      1394588
  Merchant Number:  267089009885
  Merchant Name:    PAYBYGROUP
  VSMC Sic Code:    7399
  MID No:           2414673
  TID No:           3593171
  SANDBOX:          
  PROCESSOR:        32b261335b34f74ddb3b3d4a
  
### Stuff


TRANSACTION
.success?  .failed?


Web:    https://samurai.feefighters.com/users/ea9abf40/get-started
Token:  0de8d4c66bfb6f37ea423ed0

curl -u 'd422aaf39e747f4aae84c38b:623b781f49d9cdfc9c06ce96' \
-d "transaction[amount]=1.00" \
-d "transaction[currency_code]=USD" \
-d "transaction[payment_method_token]=0de8d4c66bfb6f37ea423ed0" \
-d "transaction[custom]=8290ab" \
   https://api.samurai.feefighters.com/v1/processors/68555be57bbd7c7c19fb6ce1/purchase.xml



<error>
  <echo>
    <url>/v1/processors/68555be57bbd7c7c19fb6ce1/purchase.xml</url>
    <request_method>POST</request_method>
    <payload>
      <transaction>
        <amount>1.00</amount>
        <currency-code>USD</currency-code>
        <custom>8290ab</custom>
        <partner-custom>{"user_agent":"curl/7.21.6 (i686-pc-linux-gnu) libcurl/7.21.6 OpenSSL/1.0.0e zlib/1.2.3.4 libidn/1.22 librtmp/2.3"}</partner-custom>
      </transaction>
    </payload>
  </echo>
  <messages type="array">
    <message subclass="error" context="system.general" key="default">Bogus Gateway: Use CreditCard number 4111111111111111 for success, 4242424242424242 for exception and anything else for error</message>
  </messages>
</error>


wDC4wcacb

