Hi Dan,

This is not possible with a normal PayPal personal/premiere/business account. The only time a payment is made without a buyer explicitly logging on to the PayPal website is a subscription, which is of a fixed amount each cycle (after any trial period), and paid with whatever funding source the buyer wants.

You can probably do part of what you want with eChecks, that some true merchant accounts would support, and automate it through a payment gateway's API. I don't know whether PayPal's merchant account offering has eCheck support, but if it does, there's no benefit to PayPal over a merchant account from elsewhere.

I know the Authorize.net gateway supports programmatic eCheck payments. You would need a merchant account that supports the Authnet gateway, plus signing up for the eCheck service, plus signing up for CIM (Customer Information Manager) to store the payment details for cutting the electronic checks when the payer isn't at the computer. You would create a customer and payment profile with the CIM API, sending it the customer's name and their bank billing information. You get back a customer ID and a payment profile ID, which you can then use to write yourself new eChecks from that customer without requesting the information again.

That only takes care of moving money one way, though. There's no way to move money TO these peoples' accounts through that API, and I don't know what, say, employers use to automate depositing paychecks to employees... that might be the other half of your project if you take this route.

Kind regards,
Dan Grossman
