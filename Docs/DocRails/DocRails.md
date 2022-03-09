# x

#+STARTUP: hidestars
%w( )   array of string   f:rackspace.org
%( )
http://en.wikibooks.org/wiki/Ruby_Programming/Syntax/Literals
https://github.com/franklangston/gpay1/compare/6196d37e4fd7259189843bf9fc85c235544e5d0f...master

# API.RUBYONRAILS          http://api.rubyonrails.org/
# GUIDES                   http://guides.rubyonrails.org/
  command_line
# Ruby                     http://www.ruby-doc.org/core-1.9.2/  
  - Clean Docs&Ref         http://www.tutorialspoint.com/ruby/ruby_quick_guide.htm
# Other Docs
-- GEMS                    http://rubydoc.info/gems    (same as rdoc)

-- API DOC            http://apidock.com/rails   (has all versions)

-- R-DOC (other gems) file:/Users/oblinger/ref/_rails/rdoc/index.html  (has some docs for other gems)
-- RAILS:             http://rubydoc.info/docs/rails/frames
-- ACTIVE ADMIN       http://activeadmin.info/docs/6-show-screens.html
Rubular    http://rubular.com/
HYPERGLOT  http://hyperpolyglot.org/scripting
-- Ruby-Doc.org       http://ruby-doc.org/docs/ProgrammingRuby/html/builtins.html
--  More Docs

Multi DS   http://langref.org/all-languages/pattern-matching
Tight Lang http://www.zenspider.com/Languages/Ruby/QuickRef.html  (not very complete)
RUBY-DOC   http://www.ruby-doc.org/stdlib-1.9.3/libdoc/     (doc folders)
RUBY       https://mail.google.com/mail/#contacts
SHEET MENU http://www.smashingmagazine.com/2006/10/30/cheat-sheet-round-up-ajax-css-latex-ruby/

Cheat sheet http://www.ruby-doc.org/core-1.9.2/String.html#method-i-slice
Cheat sheet http://www.zenspider.com/Languages/Ruby/QuickRef.html
            http://www.ruby-doc.org/core-1.9.2/String.html#method-i-slice
            http://web.njit.edu/all_topics/Prog_Lang_Docs/html/ruby/syntax.html

RAILS
R-DOC      http://railsapi.com/doc/rails-v3.0.8rc1/
R-DOC(loc) file:/Users/oblinger/ref/_rails/rdoc/index.html
API DOC    http://apidock.com/rails
KOANS      http://koans.heroku.com/?koan=about_regular_expressions
http://koans.heroku.com/?koan=about_methods
http://koans.heroku.com/?koan=about_exceptions
http://koans.heroku.com/?koan=about_iteration
http://koans.heroku.com/?koan=about_blocks
http://koans.heroku.com/?koan=about_classes
http://koans.heroku.com/?koan=about_dice_project  22/30 complete

String:    "foo" [/regex/, 1]  or $1   "findall".scan(re)  'find/rep'.sub(re)

# Docs Tree

VIEW HELPERS  http://guides.rubyonrails.org/form_helpers.html
# hold 

APP_CONFIG = YAML.load_file("#{RAILS_ROOT}/config/config.yml")[RAILS_ENV]

 true false nil (all lowercase)
.is_a?(Object)  Array[starts@0,len]  Array[-fromEnd, -fromEnd]  ok off end, beginning off = nil
 Range(a..b) inclusive  (a...b) exclusive
 Hash == means equal 
 Strings  %(!{ string }!)  each in its own buffer

hash[:missing]==nil

  . (class)  $ (jquery)  # (id)  alert(x) > (direct child)   [by_attr]   :last

 (read-from-minibuffer prompt initial-contents keymap read hist)))
http://www.freshblurbs.com/install-rails-3-ruby-1-9-nginx-and-mongodb-debian-lenny

## a new one

## Note about install
Installation
Install Homebrew and follow the steps in the wiki about not sudoing when installing RubyGems.

If you don't want to do it that way (I highly recommend doing it the above way), you can just run this command to install homebrew.

$ sudo chown -R $USER /usr/local
$ curl -Lsf http://github.com/mxcl/homebrew/tarball/master | tar xvz -C/usr/local --strip 1


## Snipits
  http://www.google.com/codesearch#wojYChrHnn0/trunk/app/models/user.rb&q=lang:%5Eruby$%20file:app/models&l=3&type=cs
  http://www.google.com/codesearch#BnFSjpx4gnM/app/controllers/ui_controller.rb&q=lang:%5Eruby$%20file:app/controllers&l=1&type=cs


redirect_to(@club, :notice => 'Club was successfully created.') 


# EXPRESSIONS
[VAR]  $global_var, @instance_var, @@class_var, local_var, CONST_VAR,  ClassName :symbol_name self, nil, true, false __FILE__, __LINE__
  '\n' != "\n"   "eval #{ "in" } the string"  a = [ nil, 'cat', 3.14 ]
  +, -, *, /, %, **, &, |, ^, <<, >>, &&, ||
  foo, bar, baz = 1, 2, 3   foo, = list()
  foo, *rest = list2()
[OPS]  puts (13...19).include?(age) ? "teenager" : "not a teenager"  
        :: [] ** -(unary)  +(unary)  !  ~
	*  /  % +  -  <<  >>
	&  |  ^  >  >=  <  <= <=> ==  === !=  =~  !~
	&&  ||  .. ...  =(+=, -=, ||=, &&=, ...)   not and or 
[TINY]  .nil?

[EXPR] 3+2 3**2  Math.sqrt(9)

# IO_ETC
[IO] puts, print

[FILE] Dir.glob('*')

[LOADING]
BEGIN { \\..\\ }  END { \\..\\ }  # run before load

[FILE]

[DIR]  Dir[s], chdir([s]) delete(s), entries(s), foreach (s) do |f|, getwd, mkdir(s), new(s), 
       open (s) do |dir| .. end, close, pos -> integer, read -> s|nil, rewind
[DATE TIME]
require 'date'

Date.new(2001,2,3)           #=> #<Date: 2001-02-03 ...>
Date.jd(2451944)             #=> #<Date: 2001-02-03 ...>
Date.ordinal(2001,34)        #=> #<Date: 2001-02-03 ...>
Date.commercial(2001,5,6)    #=> #<Date: 2001-02-03 ...>
Date.parse('2001-02-03')     #=> #<Date: 2001-02-03 ...>
Date.strptime('03-02-2001', '%d-%m-%Y')
                             #=> #<Date: 2001-02-03 ...>
Time.new(2001,2,3).to_date   #=> #<Date: 2001-02-03 ...>

 now parse(s) strptime(s,fmt) day, hour, leap?, min, month, sec, year
  ::_httpdate ::_iso8601 ::_jisx0301 ::_parse ::_rfc2822 ::_rfc3339 ::_rfc822 ::_strptime ::_xmlschema ::civil 
  ::commercial ::gregorian_leap? ::httpdate ::iso8601 ::jd ::jisx0301 ::julian_leap? ::leap? ::new ::ordinal 
  ::parse ::rfc2822 ::rfc3339 ::rfc822 ::strptime ::today ::valid_civil? ::valid_commercial? ::valid_date? 
  ::valid_jd? ::valid_ordinal? ::xmlschema 
+ - << <=> === >> 
.ajd .amjd .asctime .ctime .cwday .cweek .cwyear .day .day_fraction .downto .england .friday? .gregorian 
.gregorian? .httpdate .inspect .iso8601 .italy .jd .jisx0301 .julian .julian? .ld .leap? .mday .mjd .mon 
.monday? .month .new_start .next .next_day .next_month .next_year .prev_day .prev_month .prev_year .rfc2822 
.rfc3339 .rfc822 .saturday? .start .step .strftime .succ .sunday? .thursday? .to_date .to_datetime .to_s 
.to_time .tuesday? .upto .wday .wednesday? .xmlschema .yday .year


 sprintf("$%0.02f", product.price) 


---     BEGIN	 class	  ensure   nil	    self     when
	END	 def	  false    not	    super    while
	alias	 defined  for	   or	    then     yield
	and	 do	  if	   redo     true
	begin	 else	  in	   rescue   undef
	break	 elsif	  module   retry    unless
	case	 end	  next	   return   until

FANCY
foo(*[1,2]) == foo(1,2)


EXAMPLES
foo = bar
	foo[0] = bar
	foo.bar = baz
foo+=1


 BLAM RAILS

# BLAM RAILS FNs
RAILS COMMUNITY  http://apidock.com/rails
RUBY  COMMUNITY  http://apidock.com/ruby
Top Ten          http://railsapps.github.com/best-recommended-learning-rails-books-resources.html

  session[:cart_id] = cart.id
  @group_purchase = GroupPurchase.find(params[:id])
  params()  # parameters of the HTTP action
  path_parameters()  # parameters derived from the route path

[String]  sanitize(TEXT)  sprintf("$%0.02f", product.price)
# Overview
## Visibility
### helper_method :foo    # written in app controller; this makes foo available to views
### Misc
                                  [M]odel, [C]ontroller, [V]iew
   - application.rb
   - application_helper.rb
   - application_controller.rb    CONT, 
   - util.rb

### specific includes
    include ActionView::Helpers::UrlHelper   _path _url


## Rails Naming And Case
   - FILES:     PLURAL     ALL_LOWER
   - CLASSES:   SINGULAR   UPPER 
   - MEHODS:               ALL_LOWER
   - CONST:                ALL_UPPER
   - AR:        =CLASS
   - INSTANCE:  SINGULAR   LOWER
   - URL:       *****      LOWER
   - YML:       =FILE    
   URL:     localhost:3000/controller_name/action_name     ??
   File:    app/controllers/controller_name.rb           PLURAL   LOWER
   Table:   class_names                                  PLURAL   LOWER
   File:    /app/models/class_name.rb                    SINGULAR LOWER
   Class:   ClassName  ClassName.all                     SINGULAR UPPER
   CMD      rails generate scaffold ClassName  // rails generate controller ControllerClass

   CONTROLLER  PLURAL  Class: OrdersController  Methods: multi_word_action ??
       File: /app/controllers/orders_controller.rb   /app/layouts/orders.html.erb
   VIEWS

Helper: /app/helpers/orders_helper.rb
Helper Module: OrdersHelper
Views: /app/views/orders/… (list.html.erb for example)

Tests Naming Convention

Unit: /test/unit/order_test.rb
Functional: /test/functional/orders_controller_test.rb
Fixtures: /test/fixtures/orders.yml

Table: orders
Class: Order
File: /app/models/order.rb
Primary Key: id
Foreign Key: customer_id
Link Tables: items_orders

## Rails App File Structure
  - Gemfile 	This file allows you to specify what gem dependencies are needed for your Rails application. See section on Bundler, below.
  - README 	This is a brief instruction manual for your application. You should edit this file to tell others what your application does, how to set it up, and so on.
  - Rakefile 	This file locates and loads tasks that can be run from the command line. The task definitions are defined throughout the components of Rails. Rather than changing Rakefile, you should add your own tasks by adding files to the lib/tasks directory of your application.
  - assets/     IMAGES/ JAVASCRIPT/ STYLESHEETS/    
  - app/ 	Contains the controllers, models, views and assets for your application. You’ll focus on this folder for the remainder of this guide.
    - views
      - layouts APPLICATION.HTML.ERB  -- toplevel template for entire site
  - config/ 	Configure your application’s runtime rules, routes, database, and more.
  - config.ru 	Rack configuration for Rack based servers used to start the application.
  - db/ 	Shows your current database schema, as well as the database migrations. You’ll learn about migrations shortly.
  - doc/ 	In-depth documentation for your application.
  - lib/ 	Extended modules for your application (not covered in this guide).
  - log/ 	Application log files.
  - public/ 	The only folder seen to the world as-is. Contains the static files and compiled assets.
  - script/ 	Contains the rails script that starts your app and can contain other scripts you use to deploy or run your application.
  - test/ 	Unit tests, fixtures, and other test apparatus. These are covered in Testing Rails Applications
  - tmp/ 	Temporary files
  - vendor/ 	A place for all third-party code. In a typical Rails application, this includes Ruby Gems, the Rails source code (if you install it into your project) and plugins containing additional prepackaged functionality.
Directory tree
  app/views/layouts/application.html.erb      # Generic outer frame
  assets/sytlesheets/applications.css
  db

## Special Objects
### Flash -- messaging
    # Shortcuts
    flash.alert = "You must be logged in"
    flash.notice = "Post successfully created"
    # flash is a map past to the next page load.  flash.now is used in this load.
    flash[:notice] = "Post successfully created"
    redirect_to posts_path(@post)

    <% if flash[:notice] %> <div class="notice"><%= flash[:notice] %></div> <% end %>
## Control Flow
### before_filter
    before_filter :some_method, except: [:this_method, :that_method]
## Pkgs
### Testing
    require 'test/unit/assertions'
    include Test::Unit::Assertion
    assert false, "ffff"

# MVC -- Routing
## resource :controller  -- REST Methods
   NAME&MEANING      HTT-Method	    Route Path                     Controller & Action
   index (showall)   posts GET	    /sentences(.:format)	   {:controller=>”posts”, :action=>”index”}
   new   (createnew) new_post GET   /sentences/new(.:format)	   {:controller=>”posts”, :action=>”new”}
   -> create         posts POST     /sentences(.:format)	   {:controller=>”posts", :action=>”create”}
   edit              edit_post GET  /sentences/:id/edit(.:format)  {:controller=>”posts”, :action=>”edit”}
   ->update          post PUT	    /sentences/:id(.:format)	   {:controller=>”posts”, :action=>”update”}
   show    (one)     post GET	    /sentences/:id(.:format)       {:controller=>”posts”, :action=>”show”}
   destroy (delete)  post DELETE    /sentences/:id(.:format)	   {:controller=>”posts”, :action=>”destroy”}

## resources :resource_name :only => [:index, :create] :pathname => 'fooo???'
## get/post/match 'controller_name/:key/etc' => 'controller#action', as: 'ruby_path_fn', via: :post
   match 'path/:key/etc' => 'controller#action', :as 'url_fn'
    :as =>  'ruby_path_fn'  # '_path' will be appended
    :via => :get            # HTTP verb (short hand cmd 'GET...')
    :via => [ :get, :post ] # match multiple
    :defaults => { :format => 'jpg' } # short hand for
    :constraints =>         # constrains matches
   get  'path'              # same a match with via: :get
   post 'path'              # same a match with via: :post

## map
    map.connect ':controller/:action/:id', :controller => 'blog'


## Misc
   root_url   base URL for the app

 post "versions/:id/revert" => "versions#revert" :as => "revert_version"

  
app/config/routes.rb
  APP_CLASS::Application.routes.draw do |map| \\
    resources :tweets   <-- generates default routes
    match 'new_tweet' => "CONTROLLER#ACTION"
    match 'all' => "Tweets#index", :as "all_tweets"  <-- defines  all_tweets_path
    match 'all' => redirect("/suffix")  "http://cnn.com"
    match 'xx/:zipcode' => "CONT#ACTION"  <-- params[:zipcode] will be set in controller.
    root :to => ""  <--  root_path 

Depot::Application.routes.draw do
  get "store/index"
  resources :products
  root to: 'store#index', as: 'store'
endUsing activeresource (3.1.1) 
 
## actions
match "/stories" => redirect("/posts")
match "/stories/:name" => redirect("/posts/%{name}")
match "/stories/:name" => redirect {|params| "/posts/#{params[:name].pluralize}" }
match "/stories" => redirect {|p, req| "/posts/#{req.subdomain}" }

#As long as Sprockets responds to call and returns a [status, headers, body], the router won’t know the difference between the Rack application and an action.
match "/application.js" => Sprockets

## docs
Rack:  http://yehudakatz.com/2009/12/26/the-rails-3-router-rack-it-up/

# MVC -- Views       (.erb stuff)
## Helpers
   SEE:  API doc  http://apidock.com/rails/ActionView/Helpers/UrlHelper/link_to
### Form_tag helpers
#### <%= Form_tag PATH_FOR_POST, method => :get do %> <% end %>
     Form_tag URL, OPT... do
     :multipart - If set to true, the enctype is set to “multipart/form-data”.
     :method - The method to use when submitting the form, usually either “get” or “post”.
     :authenticity_token - Authenticity token to use in the form. Use only if you need to
     :remote - If set to true, will allow the Unobtrusive JavaScript drivers to control the submit behavior

#### ALL
    hidden_field_tag, image_submit_tag label_tag number_field_tag password_field_tag phone_field_tag 
    radio_button_tag range_field_tag search_field_tag select_tag submit_field_tag telephone_field_tag 
    text_area_tag text_field_tag url_field_tag
#### <%= text_field_tag :name, 'initial val' %>
  <%= text_field_tag 'ip', '0.0.0.0', :maxlength => 15, :size => 20, :class => "ip-input" %>
#### <%= label_tag 
#### <%= text_area_tag
  <%= text_area_tag 'name', 'initialcontent', :size => "25x10" %>
    * :size - A string specifying the dimensions (columns by rows) of the textarea (e.g., “25x10”).
    * :rows - Specify the number of rows in the textarea
    * :cols - Specify the number of columns in the textarea
    * :disabled - If set to true, the user will not be able to use this input.
    * :escape - By default, the contents of the text input are HTML escaped. If you need unescaped contents, set this to false.
    * Any other key creates standard HTML attributes for the tag.
#### <%= submit_tag 'msg'
     <%= submit_tag 'Complete sale', :confirm => "sure?", :disable_with => "Please wait..." %>
#### <%= check_box_tag :remember_me, 1, params[:remember_me] %>
### <%= render 'inner_form' %>    # Note: leading '_' is dropped
# Renders the same partial with a local variable.
  render :partial => "person", :locals => { :name => "david" }
    <%= render 'other_controller/inner_form' %>
   render NAME :template  # NAME can be an absolute file, a path relative to 'views' dir
   render :inline => "<% products.each do |p|%> <p> <%= p.name %> </p> <% end %>"
   render :json => @product     # Renders some json
   render :xml => @product 
### Redirect_to
    redirect_to :action => "show", :id => 5
    redirect_to post
    redirect_to "http://www.rubyonrails.org"    # Absolute URL
    redirect_to "/images/screenshot.jpg"        # Prepends current protocol
    redirect_to articles_url
    redirect_to :back                           # Back to caller

### <%= link_to TXT, URL
        :confirm => 'really do it? ',
        :method  => :post,
        :remote  => true,    #  driver to make an Ajax request to the URL
  <%= link_to 'TEXT', URL %>
  <%= link_to 'Del', product, :confirm "yo, for real? " :method :delete %>
  <%= link_to "Pr", profile_path(@profile) %> == <%= link_to "Pr", @profile %>
  <%= link_to(@profile) do %>
    <strong><%= @profile.name %></strong> -- <span>Check it out!</span>
  <% end %>   

### <%= button_to "ok", items_path, method: :get %>
  <%= button_to "Checkout", new_order_path, method: :get %>
  <%= button_to 'Empty cart', cart, method: :delete, confirm: 'Are you sure?' %>
### <%= hidden_field_tag 'token', 'VUBJKB23UIVI1UU1VOBVI@' %>
### <%= image_tag(“icon.png”, :size => “16x10”, :alt => “Edit Entry”) %>
      :mouseover => “/images/mouse_over.png”, :height => "55", :width => "200"
### Form_for
hidden_field  label number_field password_field phone_field radio_button range_field search_field
telephone_field text_area text_field url_field
### Others
  <%= image_tag(URL) %>
  <%=  sanitize(TEXT) %>


#### check_box_tag(name, value = "1", checked = false, options = {}) 
#### email_field_tag(name, value = nil, options = {}) 
#### field_set_tag(legend = nil, options = nil, &block) 
#### file_field_tag(name, options = {}) 
  <%= form_tag '/upload', :multipart => true do %>
    <label for="file">File to Upload</label> <%= file_field_tag "file" %>
    <%= submit_tag %>
  <% end %>
#### hidden_field_tag(nbuame, value = nil, options = {}) 
#### image_submit_tag(source, options = {}) 
#### label_tag(name = nil, content_or_options = nil, options = nil, &block) 
#### password_field_tag(name = "password", value = nil, options = {})    min: max: in: step: (plus all txt opts)
#### password_field_tag(name = "password", value = nil, options = {}) 
#### phone_field_tag(name, value = nil, options = {}) 
#### radio_button_tag(name, value, checked = false, options = {}) 
#### <%= ??? url_field_tag(name, value = nil, options = {}) 
#### ??? search_field_tag(name, value = nil, options = {}) 

## notes
app/views/MODELNAME/index.html.erb   show.html.erb    <% ... %>   <%= ... %>
## app/veiws/layouts/application.html.erb   <%= yield %>    <--- main layout (used by all pages)
     <%= stylesheet_link_tag :all %>           <-- includes app/public/stylesheets
     <%= javascript_include_tag :defaults %>   <-- includes 
     <%= csrf_meta_tag %>

## PATHS action_class_path(:key=>'val', ...)
  action = new, edit, show, 
  tweets_path, new_tweet_path, tweet,  edit_tweet_path(tweet), 
  tweet, menthod-> :delete

EXPR
  sanatize(text)   
  truncate(strip_tags(product.description), :length => 80)

### <%= form_for ...    
<%= form_for @instace_var do |f| %>           # NOT A FUCKING LOOP
  <p>
    <%= f.label :name, "Name:" %>
    <%= f.text_field :name, size: 40 %>
  </p>
<% end %>

f.label .text_field 

## ERB Stuff
  <%= "Ruby Code" + @rubyVar %> 
link_to "text", CONTROLLER_VERB_path

<% if notice %>
<p id="notice"><%= notice %></p>
<% end %>
#
 MVC -- Controller

===CONTROLLER===    <--- Model calls in controller
app/controllers/CLASS(s)_controller.rb
  class CLASSsController < ApplicationController \\ ... \\ end
    -def-> before_filter :METHOD, :only=>[...]   <-- init
    -var-> params, session, flash, ACTION_CLASSs_path   (lowercase)
    -exe-> redirect_to(tweets_path, :notice="sorry...")
    def show \\ @var=Class.find(params[:id]) \\ end          <--- view code shoved into here
        @var  <--- in view code
      respond_to do |format|
        format.xml { render :xml=>@tweet }
      render :action=>'XXX'  --> views/CLASSs/XXX.html.erb
    def show, index, new edit, create, update, destroy

## SANITIZE
    CSRF -- Cross Site 
    XSS  -- cross site S  default protection in Rails
      plain strings are HTML escaped at view level
      SafeBuffers are passed directly

   include ActionView::Helpers::SanitizeHelper
   sanitize(x)  h(x) 
## HTML
### <div id="columns">
### <ul <li> 

## Supporting Functions
### Rendering


    render_to_string(:partial => "mvp/cancel_purchase/confirm")

# MVC -- Controllers

  TableClass.all  .order(:field)
# MVC -- Models
## DEFINITION 
   class Model < ActiveRecord::Base
     has_many :tasks                                   # ??? means a back pointer in 'tasks'
     has_many :assignments, :through => :tasks         # Recursive thru 2 pointers???
     attr_accessor :name, :email
     attr_accessible :name, :as => :admin_role         #  model.update(... :as => admin_role)
       
     attr_reader, attr_writer, attr_accessor
   end
### Table Relationships
                KEY
    has_one     other_table  :address    
    has_many    other_table  :orders
    belongs_to  this_table   :
    has_and_belongs_to_many both  (need intermediate table to hold association)
    has_many    :tasks


     :as => 
    
### Validations
#### -custom-        validate :f, :the_name_of_some_method_in_this_class
       def the_name_of_some_method_in_this_class;  if f.blank?; errors.add("err msg"); end
#### length          validates :name, :length => { :minimum => 2, :maximum =>10, :is=>6 }
class Person < ActiveRecord::Base
  validates :name, :length => { :minimum => 2 }
  validates :bio, :length => { :maximum => 500 }
  validates :password, :length => { :in => 6..20 }
  validates :registration_number, :length => { :is => 6 }
end
#### Numericality    validates, :f, :numericality => { :only_integer => true }
class Player < ActiveRecord::Base
  validates :points, :numericality => true
  validates :games_played, :numericality => { :only_integer => true }
end
:greater_than – Specifies the value must be greater than the supplied value. The default error message for this option is “must be greater than %{count}”.
:greater_than_or_equal_to – Specifies the value must be greater than or equal to the supplied value. The default error message for this option is “must be greater than or equal to %{count}”.
:equal_to – Specifies the value must be equal to the supplied value. The default error message for this option is “must be equal to %{count}”.
:less_than – Specifies the value must be less than the supplied value. The default error message for this option is “must be less than %{count}”.
:less_than_or_equal_to – Specifies the value must be less than or equal the supplied value. The default error message for this option is “must be less than or equal to %{count}”.
:odd – Specifies the value must be an odd number if set to true. The default error message for this option is “must be odd”.
:even – Specifies the value must be an even number if set to true. The default error message for this option is “must be even”.

#### Presence
class Person < ActiveRecord::Base
  validates :name, :login, :email, :presence => true
end
   * Inclusion       validates :size, :inclusion => { :in => %w(small medium large), :message => "%{value} not valid" }

#### Uniqueness
     validates :zipcode, :uniqueness => true
class Holiday < ActiveRecord::Base
  validates :name, :uniqueness => { :scope => :year,
    :message => "should happen once per year" }
end
class Holiday < ActiveRecord::Base
  validates :name, :uniqueness => { :scope => :year,
    :message => "should happen once per year" }
end

   * Format
       validates :legacy_code, :format => { :with => /\A[a-zA-Z]+\z/,
                 :message => "Only letters allowed" }
### Validators
#### validates :field_name1, ..., :numericality=>true
               :presence, :uniqueness=>true :format => { :with=> :message=> }
     modifiers
         :allow_nil :allow_blank => blank_val
         :if => condition_method, :unless => :condition_method

    validates :first_name,:last_name, :email, :source_type_id, :presence => true
    validates :email, :uniqueness=>true, :format => {
      :with => /\A([^@\s]+)@((?:[-a-z0-9]+\.)+[a-z]{2,})\Z/i,
      :message => 'must be a valid email address'
    }
    validates :source_type_id, :numericality => true

    validates_uniqueness_of 
  
### Validation Options :allow_nil, :allow_blank, :message=>"", :on=>:create, :if/:unless => :paid_with_card?
class Coffee < ActiveRecord::Base
  validates :size, :inclusion => { :in => %w(small medium large),
    :message => "%{value} is not a valid size" }, :allow_nil => true
end
### Validation Errors   .clear .size .errors -> hash of arrays of errors associated w. each field (or :base)
### Valid types
     primary_key, string, text, integer, float, decimal, 
     datetime, timestamp, time, date, binary, boolean
### Migrations
    $ rails g migration rename_customer_in_transaction_record rename_column custom details

    http://api.rubyonrails.org/classes/ActiveRecord/Migration.html

     * create_table(name, options)
     * drop_table(name)
     * rename_table(old_name, new_name)
     * add_column(table_name, column_name, type, options)
     * rename_column(table_name, column_name, new_column_name)
     * change_column(table_name, column_name, type, options)
     * remove_column(table_name, column_name)
     * add_index(table_name, column_name, index_type)
     * remove_index(table_name, column_name)

## ACCESS 
### DB accessors
http://guides.rubyonrails.org/active_record_querying.html
#### find_by_FIELD1_and_FIELD2(x,y)
#### BASE   Model.find(10) 
     Model.find(id)
##### Client.where("orders_count = ? AND locked = ?", params[:orders], false)
      Model.where("column_name = ?", col_val)       .first!  .last!
      lst=Person.where(["name = :n and age = :a", {:n => 'dan', :a => 24}])
      Person.where(["user_name = :u", { :u => user_name }]).first
      Person.where(:user_name => user_name, :password => password, :gender => 'male').first

#### WHERE
      Client.where("orders_count = ? AND locked = ?", params[:orders], false)
      TABLE.where(name: 'dave').each do |order| puts order.amount;  end
#### LIST RESTRICTORS
##### ALL
.where
select
group
order
reorder
reverse_order
limit
offset
joins
includes
lock
readonly
from
having
##### where
##### select specfic columns
      Client.select("viewable_by, locked")

### Active Record Methods
 .find(idx)
  Tweet-->tweets t=Tweet.new  t=Tweet.find(3)  puts t[:id]==puts t.id
          .new .create(:field val, ...) .destroy  .destroy_all  .save .errors
          .find(id)  .first .last .all .count
   CHAINABLE .find(id,id,...)  .order(:field) .limit(#)  .where(:field=>val)  
UPDATE:  .anyField=x  .attributes={..}  .update_attributes={...}

class Project < ActiveRecord::Base
  belongs_to :portfolio
  has_one   :project_manager
  has_many  :milestones
  has_many  :deliverables, through: milestones
  validates :name, :description, presence: true :non_disclosure_agreement, acceptance: true
  validates :short_name, uniqueness: true

##### misc
     Model.all,              
     Model.find, find_by_attributes, find_first, find_last, find_one, find_or_instantiator_by_attributes, 
          .find_some, find_with_associations, find_with_ids,
     apply_join_dependency
     Model.exists?( :column => 'value' )
     inst.object_id == inst2.object_id

#### Active Record Methods
 .find(idx)
  Tweet-->tweets t=Tweet.new  t=Tweet.find(3)  puts t[:id]==puts t.id
          .new .create(:field val, ...) .destroy  .destroy_all  .save .errors
          .find(id)  .first .last .all .count
   CHAINABLE .find(id,id,...)  .order(:field) .limit(#)  .where(:field=>val)  
UPDATE:  .anyField=x  .attributes={..}  .update_attributes={...}

TABLE.where(name: 'dave').each do |order|
  puts order.amount
end

class Project < ActiveRecord::Base
  belongs_to :portfolio
  has_one   :project_manager
  has_many  :milestones
  has_many  :deliverables, through: milestones
  validates :name, :description, presence: true :non_disclosure_agreement, acceptance: true
  validates :short_name, uniqueness: true

## UPDATE
http://guides.rubyonrails.org/active_record_validations_callbacks.html
### VALIDATING UPDATES 
create,  create!
save,    save!,    update
update_attributes!(:key=>val,...)
### NON VALIDATING UPDATES
decrement!
decrement_counter
increment!
increment_counter
toggle!
touch
update_all
update_attribute
update_column
update_counters

### .valid?  .invalid?  .errors[]   # tests validitiy of current object
### DB manipulation
    .new [instCreate] .save[!] [writeToDB] .create [new+save]
    Model.delete_all .create(:field => "value") 
## Errors &
   
      flash[:alert] = @pu.ua.errors.full_messages.join('; ')
## notes
  app/models/tweet.rb \\ class Tweet < ActiveRecord::Base \\ validates ... \\ end
  validates :FIELD, ..., :presence=>true, :uniqueness=>true,
            :numbericality=>true, :numbericality (greater_than_or_equal_to: 0.01)
            :length=>{:minimum=>0,:maximum=>9}
            :format=>{:with=>/.*/}, :inclusion=>{:in=>[1,2,3]}
                      :exclusion=>{:in=>[]} :acceptance=>true,
                      :confirmation=>true :message "XX must be ..."}
  belongs_to :class_singular   # creates field_name_id
  has_many   :class_plural     # creates lowercase_pluralized_accessor
  has_one    :class_singular   # creates one-to-one 

  default_scope :order => 'title'

## callbacks

after_find()
after_initialize()
before_validation()
around_save(); /../ yield /../ end

## YAML access
http://www.skorks.com/2010/04/serializing-and-deserializing-objects-with-ruby/

## JSON access
map = ActiveSupport::JSON.decode('{ "a":3 }')  # WARN!!  keys are STRINGS not symbols
array = ActiveSupport::JSON.decode('["zero", "one", "two", "three"]')
ActiveSupport::JSON.encode(array)

user = User.find(1)
user.as_json    # => { "user": {"id": 1, "name": "Konata Izumi", "age": 16, "created_at": "2006/08/01", "awesome": true} }
ActiveRecord::Base.include_root_in_json = false

user.as_json(:only => [ :id, :name ])
user.as_json(:except => [ :id, :created_at, :age ])
user.as_json(:methods => :permalink)                   # Calls method and includes result
user.as_json(:include => :posts)                       # Includes associations

user.from_json(json)

# MVC -- Database
  
# Rails Debugging

## In rails console
   load Rails.root.join('lib/s.rb')
require Rails.root.join("lib", "s")


## Using debugger -n- other stuff
  http://bashdb.sourceforge.net/ruby-debug.html#rdebug-command_002dline-options
http://cheat.errtheblog.com/s/rdebug/
  InGemfile:  gem 'ruby-debug19', :require => 'ruby-debug'
  InCode:     debugger  # server will drop into debugger

>>> http://jeremyhubert.com/articles/debugging-in-rails.html
    logger.info("SESSION=" + session.inspect)
    logger.info("PARAMS=" + params.inspect)
    <%= 'string'.inspect %>  <%= debug @object %>  # prints on page
    <%= sanitize($LOAD_PATH.join('<br>')) %>
<%= debug @post %>  // prints YAML
<%= simple_format @post.to_yaml %>  // simple format puts each line on its own
## Using RDB
   >> Gemfile
      gem 'ruby-debug19'
   $ gem install ruby-debug19
   $ bundle install

   >> in_code.rb  
      debugger

## In View
   <%= image_tag "http://lorempixel.com/266/350/" %>
   <%= link_to "Send Payment", "javascript:alert('Not implemented yet');", :class => "orange button" %>
## In controller
  logger.debug "The object is #{@object}"
  RAILS_DEFAULT_LOGGER.debug @object
  return render :text => "The object is #{@object}"
    The render :text call will dump the text to the screen and halt execution of the current action. This will allow you to see what you’re working with right away, without having to pass variables to a view. It’s great for quick debugging.
## In model
   RAILS_DEFAULT_LOGGER and logger.debug is one of your best options, if not the only one.
## Outside The App Folder
   $ rails console    # runs ruby w. entire app in it!
   $ tail -f log/development.log   # watch dev.log
   $ rails console >> 
     puts h=Rails.application.routes.named_routes.helpers.map(&:to_s).grep(/path/).sort()
## ERRORS
## Could not find a JavaScript runtime. 
   >> ~/proj/xxx/Gemfile
     gem 'execjs'
     gem 'therubyracer'
   $ bundle install
   
## logger
   Logger.tagged("BCX") { Logger.info "Stuff" }
config.log_tags = [ :subdomain, :uuid ]          # in environment .rb files('{ "a":3 }')  # WARN!!  keys are STRINGS not symbols
array = ActiveSupport::JSON.decode('["zero", "one", "two", "three"]')
ActiveSupport::JSON.encode(array)

user = User.find(1)
user.as_json    # => { "user": {"id": 1, "name": "Konata Izumi", "age": 16, "created_at": "2006/08/01", "awesome": true} }
ActiveRecord::Base.include_root_in_json = false

user.as_json(:only => [ :id, :name ])
user.as_json(:except => [ :id, :created_at, :age ])
user.as_json(:methods => :permalink)                   # Calls method and includes result
user.as_json(:include => :posts)                       # Includes associations

user.from_json(json)

# MVC -- Database
  
# Rails Command Line Tools
## rails generate
   
   $ rails g controller ClassName method_name1 method_name2
   $ rails g model Product name:string description:text
   $ rails g scaffold ClassName field1:type field2:type
   $ rails g migration add_order_id_to_line_item order_id:integer
   $ rails g migration destroy_user_infos
     def up
       drop_table :user_infos
     end
     def down
     end
### rails g migrate
    $ rails g migration change_field1_format_in_my_table
      >> change_column :my_table, :my_column, :datetime
 
## rails destroy
   $ rails destroy model user_info
## rails
   $ rails new RAILS_APP_NAME        Gemfile >> (gem 'execjs'\\gem 'therubyracer') DEL public/index.html
   $ rails server --port=80
   $ firefox http://localhost:3000/
   $ rails --version
   $ rails console
   $ rails c production

The most common rails commands are:
 generate    Generate new code (short-cut alias: "g")
 console     Start the Rails console (short-cut alias: "c")
 server      Start the Rails server (short-cut alias: "s")
 dbconsole   Start a console for the database specified in config/database.yml
             (short-cut alias: "db")
 new         Create a new Rails application. "rails new my_app" creates a
             new application called MyApp in "./my_app"

In addition to those, there are:
 application  Generate the Rails application code
 destroy      Undo code generated with "generate"
 benchmarker  See how fast a piece of code runs
 profiler     Get profile information from a piece of code
 plugin       Install a plugin
 runner       Run a piece of code in the application environment (short-cut alias: "r")

## bundle -- maipulates dependencies defined in the 
   $ bundle install          # will install required gems (in current proj???)
   $ bundle show GEM_NAME    # will show path to the gem being used
## gem
   $ gem list 
   $ gem install NAME --version VVVV

## rake
   $ rake db:migrate RAILS_ENV="production"
   ! ON STAGING:   rake db:reset
   $ rake -T              # DOCS
   $ rake ... RAILS_ENV=  # = development production staging test
   $ rake db:reset        # db:drop db:create db:schema:load 
   $ rake db:setup
   $ rake db:drop         # KILLS entire DB  (all tables)
   $ rake db:schema:load  # Loads DB with current versions of all schema
   $ rake db:schema:dump  # BLOWS AWAY CURRENT schema.rb and loads from current DB tables
   $ rake db:migrate      # apply created migrations
   $ rake db:rollback     # rolls last migration back

   $ rake db:seed 
   $ rake routes          # see all the app's routes
   $ rake db:create       # needed?

# SUPER DEL:  rails g migration destroy... ; run everywhere 
#             locally delete both;  rm schema.rb ; rake db:drop ; rake db:create ; rake db:migrate
# 

# remove migrations you don't need, remove schema file locally, 
# rake db:create, rake db:migrate

 * DELAYED JOB
   $ RAILS_ENV=production2 ./script/delayed_job start
   $ RAILS_ENV=production2 ./script/delayed_job stop
   $ RAILS_ENV=production2 ./script/delayed_job restart
   $ RAILS_ENV=production2 ./script/delayed_job status
# Rails Debugging
## CONSOLE 
   binding.pry  .attributes  .methods
   U.load 'lib/s.rb'  load Rails.root.join('lib/s.rb')
   require Rails.root.join("lib", "s")
## pry
   ls [obj]           # everything you can know about 
   cd obj    cd ..    # go "in" / go "out"


## In Testing 
   -   https://gist.github.com/fedesoria/5086690

## In View
    <%= 'string'.inspect %>  <%= debug @object %>  # prints on page
    <%= sanitize($LOAD_PATH.join('<br>')) %>
<%= debug @post %>  // prints YAML
<%= simple_format @post.to_yaml %>  // simple format puts each line on its own
 @form.to_yaml.gsub("\n", '<br>').html_safe
   <%= image_tag "http://lorempixel.com/266/350/" %>
   <%= link_to "Send Payment", "javascript:alert('Not implemented yet');", :class => "orange button" %>
## In controller
  logger.debug "The object is #{@object}"
  RAILS_DEFAULT_LOGGER.debug @object
  return render :text => "The object is #{@object}"
    The render :text call will dump the text to the screen and halt execution of the current action. This will allow you to see what you’re working with right away, without having to pass variables to a view. It’s great for quick debugging.
## In model
   RAILS_DEFAULT_LOGGER and logger.debug is one of your best options, if not the only one.

## Debuggers
   binding.pry
    >  https://plus.google.com/hangouts/_/3c58ec4e7a63cf91f0392ef25a44f1277ac23eef
    > show-method
### rubymine -- IDE
### docs
  http://bashdb.sourceforge.net/ruby-debug.html#rdebug-command_002dline-options
  http://cheat.errtheblog.com/s/rdebug/
  InGemfile:  gem 'ruby-debug19', :require => 'ruby-debug'
  InCode:     debugger  # server will drop into debugger
  >>> http://jeremyhubert.com/articles/debugging-in-rails.html
### Using RDB
   >> Gemfile
      gem 'ruby-debug19'
   $ gem install ruby-debug19
   $ bundle install

   >> in_code.rb  
      debugger

## Outside The App Folder
   $ rails console    # runs ruby w. entire app in it!
   $ tail -f log/development.log   # watch dev.log
   $ rails console >> 
     puts h=Rails.application.routes.named_routes.helpers.map(&:to_s).grep(/path/).sort()
## ERRORS
## Could not find a JavaScript runtime. 
   >> ~/proj/xxx/Gemfile
     gem 'execjs'
     gem 'therubyracer'
   $ bundle install
   
## logger
   Logger.tagged("BCX") { Logger.info "Stuff" }
config.log_tags = [ :subdomain, :uuid ]          # in environment .rb files
# Rails Topics
## ASSERT ions
assert                  boolean 
assert_equal            expected, actual 
assert_raise            *args 
assert_raises           *args, &block 
assert_instance_of      klass, object 
assert_nil              object 
assert_kind_of          klass, object 
assert_respond_to       object, method 
assert_match            pattern, string 
assert_same             expected, actual 
assert_operator         object1, operator, object2 
assert_nothing_raised  *args &block
assert_not_same         expected, actual 
assert_not_equal        expected, actual 
assert_not_nil          object 
assert_no_match         regexp, string 
assert_throws           expected_symbol, &proc 
assert_nothing_thrown   &proc 
assert_in_delta         expected_float, actual_float, delta 
assert_send             send_array

# Rails Assertions

assert_response         type 
assert_redirected_to    options = {} 
assert_template         expected 
assert_recognizes       expected_options, path, extras={} 
assert_generates        expected_path, options, defaults={}, extras = {} 
assert_routing          path, options, defaults={}, extras={} 
assert_tag              *opts 
assert_no_tag           *opts 
assert_dom_equal        expected, actual 
assert_dom_not_equal    expected, actual 
assert_valid            record 


## Tools
   $ rails dbconsole     # access to database
## Hello World
  $ rails new t1
  $ cd t1
  >> ./Gemfile
     gem 'execjs'
     gem 'therubyracer'
  $ bundle install      # Will get the required gems

  $ rails generate scaffold Person name:string age:integer
  $ rake db:create   # NOT NEEDED
  $ rake db:migrate 
  $ rails server
  $ ps -augx | grep rails
  $ firefox http://localhost:3000/people 

bundle install          # will install required gems (in current proj???)
bundle show GEM_NAME    # will show path to the gem being used

## Rail Nigles
### Tell rails to use all .css sheets
   >> app/views/layout/application.html.erb  
      <%= stylesheet_link_tag    :all %>
## Rails Docs
  - LOADED http://localhost:8808/ Docs for loaded gems   %gem server &   
  - API    http://api.rubyonrails.org/ 
Ruby:      http://ruby-lang.org/   

TryRuby:   http://

Classes & Methods:   
   Google:  Ruby <classname>    --> http://ruby-doc.org
   Http://ruby-doc.org/core

Language:

RubyToolBox:    (big packages)
   http://ruby-toolbox.com/

GitHub:   
   http://github.com/rails

DOWNLOAD
 http://railsapi.com/


 Book:    http://pragprog.com/book/rails4/agile-web-development-with-rails
 Basics:  http://www.codeschool.com/courses/rails-for-zombies
 Basics:  http://www.codeschool.com/courses/try-ruby

http://api.rubyonrails.org/


http://github.com/rails/rails

## === GEMS ===
## Formtastic
   rails generate formtastic:install
    -> config/initializers/formtastic.rb
# Rails Gems
## RVM
   $ rvm use ruby-1.9.2-p290
   $ rvm gemset create gpay1
   $ rvm use ruby-1.9.2-p290@gpay1
   $ rvm use ruby-1.9.2-p290@gpay1 --default

## Devise
   WIKI:    https://wiki.github.com/plataformatec/devise
   DOCS IN:  devise_for  http://rubydoc.info/github/plataformatec/devise
   # INSTALL
   $sudo gem install devise --version=1.1.rc0   # for rails 3
   >> Gemfile
     gem 'devise', '1.1.rc0'
   $ bundle install
   $ rails generate devise_install
   >> development.comfig
     config.action_mailer. ....

   # instead of $ rails generate devise User   
   >> Model/User.rb
   devise ...
   attr_accessible
   
   # Add views
   xxxx$ rails generate devise_views
   $ rails generate devise:views

## Capistrano
   # Config  /config/deploy.rb


   # Install
   >> Gemfile  gem 'capistrano'
   $ capify .  
   $ emacs config/deploy.rb
   $ bundle exec cap deploy:setup        # buids dirs on remote

   $ cap deploy

## next line




   user_signed_in?   current_user.email  <-- in view layer

## Rails Pkg Notes

=== DEPLOYMENT ENVIRONMENT ===
- HEROKU   http://www.heroku.com/
  Easiest.  No management, no image building.   No filesystem access, no custom binaries.
- WEBBY http://webbynode.com/
  Don't need to manage image.  Do have access to filesystems / unix console, etc.  (not as robust as Linode)
- LINODE
  More mainstream VPS.  need to manage each image & its execution.


=== EXECUTION ENGINES ===
- JNODE    

## Web Servers
### Apache+Passenger
### Mongrel

    # need to recompile fastthread for 1.1.5 on snow lepord
sudo gem uninstall mongrel
sudo gem uninstall fastthread
sudo gem install mongrel

$ sudo gem install fastthread

# Rails Tricks
## Notices / Flash
   flash.alert.now = "Invalid password"   # will present in *this* view
   flash.alert = "Invalid password"   # will present *after* the next redirect

## Generating Random
   def generate_token(column)
     begin
       self[column] = SecureRandom.urlsafe_base64
     end while User.exists?(column => self[column])
   end

ActiveSupport::SecureRandom.base64(5)  .hex(2)  .random_bytes(1) .random_number(1000)

## Cookies
   cookies.permanent[:auth_token] = user.auth_token
   cookies[:auth_token] = user.auth_token
   cookies.delete(:auth_token)

## Redirects
   redirect_to url, :notice => "Logged out." :alert => "Pass expired!

## Send email (from RC #274)
   $ rails g mailer user_mailer password_reset
   # Trigger in some controller
   UserMailer.password_reset(self).deliver

   # in UserMailer
   def password_reset(user)
     @user=user
     mail :to => user.email, :subject => "Yo!"
   end

   # in password_reset.text.rb
   ...   <%= edit_password_reset_url(@user.password_reset_token) %>

   config.action_mailer.default_url_options = { :host => "localhost:3000" }

## Auth
   


## Https
   # In controller
   force_ssl

     

## rvm getset
   $ rvm 1.9.2@railspre --create    # Creates a new gemset
   $ gem install rails --pre
   
# Rails Bugs
## CAP deploy gem version bug    Could not find rake-10.0.3
   
## Logger WARN message
   Use the 'thin' web server

# Stuff

$LOAD_PATH


exec:~/bin/.emacs


  # Adds merchant integration information to a group purchase
  def self.include_info(gp, id)
    gp.merchant_integration = create(gp)
  end

  


  
