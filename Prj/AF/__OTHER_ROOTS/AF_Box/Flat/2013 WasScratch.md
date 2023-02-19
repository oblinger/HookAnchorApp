











I have been giving lo





    This is a test to see if I can show 

Ok let us see if I can type a mile a minute without looking at the keyboard.  Really I think I can it is a test of my memory of where the keys on the keyboard are...



  |














# TEMPLATE FOR NEW MERCHANTS.
    #   (1) Copy-paste this to create new merchant.  Name should begin :msd_demo_...  (no need for a date in suffix)
    #   (2) Add any parameters from demo_merchant_defaults  (if you want to change the default value listed.)
    #   (3)  The URL for the create page will be:   https://lets.paybygroup.com/merch/integrated/group_purchases/new?merchant_id=demo_...
    #        The URL for the demo merchant page:    https://lets.paybygroup.com/merch/demo/demo_page/show?merchant_id=demo_...
    defobj :msd_demo_excursionist, :msd_demo_merchant_defaults do
      defvalues :merchant_demo_page_image         => "#{U.config[:url_prefix]}/images/merchant/skins/demo/wizards_logo.png"
      defvalues :merchant_demo_page_elements      => [{image: "std_pbg.png", x:200, y:300}]
      defvalues :allow_fixed_per_person           => false                
      defvalues :allow_specified_per_person       => false                
      defvalues :merchant_icon_url                => "#{U.config[:url_prefix]}/images/merchant/skins/demo/wizards_logo.png"

      defvalues :merchant_display_name            => "Xxxx Xxxx Xxxx Xxxx"
      defvalues :merchant_short_name              => "Xxx Xxx"
      defvalues :purchase_cost                    => "1111.00"
      defvalues :purchase_image_url               => "#{U.config[:url_prefix]}/images/merchant/skins/demo/wizards_season_tickets.jpg"
      defvalues :purchase_link_url                => "http://www.nba.com/wizards/"
      defvalues :purchase_name                    => "2 Acela Club Level Season Tix"
      defvalues :purchase_description             => "Get excited for the ... best amenities ... great views of ...."
    end





  #validates :group_purchase_id, :numericality => true
  #validates :user_account_id, :numericality => true


(1) fix bugs as found in server logs
(2) add logging to track application actions
(3) complete items 5-6 from weekend
(4) add state variables and state methods to PBG common docs


- added payment error generation system for testing payments. -- tracked down apparent failures in payment execution. -- added debugging code for tracking down persistent bug in sending email. -- finished state var markdown in common docs -- finished #6 from previous entry -- pushed to production and test push



Here is my bank details: 
Routing: 021 000 021
Account: 621009287665
Bank: 


Here is my bank details: 
Routing: 021000021
Account: 621009287665
Bank: JP Morgan Chase



80 East 42nd
NY, NY 10017

My Name on Account: Gloria Hong
My Home Address: 159 West 85th Apt 3A NY, NY 10024
Final Beneficiary is me: Gloria Hong

