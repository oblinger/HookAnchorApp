### misc

walmart -- IP queiston





CareAboutItem?
-- Yes.  So what's the problem?
   -- Nothing. all good
   -- Not showing on website
   -- Distribution Issue

   -- No inventory


-- Dont care




ITEM STATUS  %pct %pct-doubled
(1) inventory issue       
    -- should have been retired.
    -- order shortfall
    -- delayed vendor ship
(2) distribution issue
    -- back order
(3) website issue




Item Splits
-- Impt/not   (inventory velocity)
-- Distributor Channels
-- By time
-- ItemType 




## design for excel demo





NAME               SEL     V1    V2   V3
walmart_week       0       
fufillment_center  3
carrier_method     0
process_step       0


## older-discussion

GOAL:   Predict LeadTime(OIID) given 

== NOUNS ==
Unit               --  Number of individual 
OIID               --  OrderedItemID
FC                 --  Fulfillment Center
ODPAIR             --  Origin Destination Pair
CM                 --  Carrier and Carrier Method
SLA                --  Service Level Agreement -- Rush, Expidited, Standard, Value

OPD                --  Order Processing Date
OPdelta            --  OPD - TODAY  (number of shipping days)
FCvol              --  Number of 


== PROPERTIES == 
ODpair(OOID)       --  The origin / destination pair for a given item
Metro(ODpair)      --  A 'region' indicator for the destination.
OMpair(OOID)       --  The origin / region indicator for a given item.
FC(OOID)           --  The FC used for OIID




== TIME BASED TERMS ==
FCvol              --  The number of OOIDs past ACK but not yet DEPART
FCvol(SLA,OPdelta) --  The subset of FCvol with a given SLA and OPdelta

volprior(OIID)     --  The volume of OIID that have higher priority at the FC at time of ACK(OIID)
volpct(OOID)       ==  volprior(OIID)/capacity(FC(OIID))     (The percentage that volprior is of the FC's entire days vol


configuration of the FC



== DEFINITIONS ==
Ftime(x)           ==  FulfilmentTime == DEPART(x) - ACK(x)    (measured in hours)
TNT(x)             ==  DDATE(x) - DEPART(x)                    (measured in days)
LeadTime(x)        ==  FulfillmentTime(x) + TNT(x)

ZIPdelta(dest)     ==  Avg(TNT(z)) - Avg(TNT(m))    where z is an order to 'dest'  where m is an order to metro(dest)


== FORECAST ==


pdf[ dayslip(  ) ]

pdf[ Ftime( FC, OPdelta, SLA, volpct ) ] 
  dayslip


pdf[ TNT(ODpair, CM) ]  --  Variables for consideration:
  depart hour, depart dayofweek, season


--sub-forecasts-for-tnt--


ZIPdelta


pdf[ TNT(OMpair, CM) ]


pdf[ TNT(ODpair, CM) ]  ==  pdf[ TNT(OMpair,CM) ] + ZIPdelta(dest(ODpair), CM)










== ORDER FLOW ==
  ORDER   -->  Cust Places Order
  AUTH    -->  Walmart Authorized Order
  GEN PO  -->  Generate PO to FC
  ACK     --   FC Ack
          --   Pick and Pack
  STAGE   --   Stage For Shipping
  LOAD*   --   Load For Shipping
  DEPART* --   Truck Departure
  SCAN    --   Shipper's scan
  DDATE   --   First Delivery Attempt




