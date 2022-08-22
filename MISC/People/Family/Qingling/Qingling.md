  [Interviewing](Interviewing.md)
  [Interviewing](MISC/People/Family/Qingling/Interviewing.md)

## DATE NIGHTS

### LISTS
- https://blog.opentable.com/best-romantic-date-night-restaurants-san-francisco/
- https://www.timeout.com/san-francisco/restaurants/romantic-restaurants-in-san-francisco
- 

### JAPANESE

#### Saru Sushi (Noe)
- https://www.yelp.com/biz/saru-sushi-bar-noe-valley-san-francisco-3?osq=japanese

### Rintaro $ $ $  mission
- https://www.izakayarintaro.com/
- 

### BIX
- 

### Marrakech Magic Theater
- https://www.yelp.com/biz/marrakech-magic-theater-san-francisco-5

### ContognaSF  (Italian romantic $ $ $)
https://www.cotognasf.com
https://www.yelp.com/biz/cotogna-san-francisco?osq=Cotogna

### Robin (michelin; japanese; omakase)
- https://guide.michelin.com/us/en/california/san-francisco/restaurant/robin

### Serpentine @night


### Atelier Crenn  (French)
- https://www.yelp.com/biz/atelier-crenn-san-francisco?osq=Atelier+Crenn
- 

### Penny Roma (Italian; 20th & Bryant)
-  [Timeout]()  
- https://www.yelp.com/biz/penny-roma-san-francisco?osq=penny+roma


### Trestle (Italian, Jackson Sq)
-  [yelp]()  
- [timeont](https://www.timeout.com/san-francisco/restaurants/romantic-restaurants-in-san-francisco)

# LOG

### t2019-00-00 - QL Notes
Incrementality:
-- Ads are not randomly provided, but aimed at good prospects boosting results
    people choose when to be advertized to
    -- randomized experiementation fixes this  (Instrumental variables)  bid level rnd
	-- num auctions won (somehow) varies by instrumental var

-- gain two values: statistical & economic.
   (spend less on stat as more accurate; "quasi-posterior of covar matrix")


AD STOCK 
-- Treat 'ad' as stock that the reducing over time.
-- Each ad has different "beta-weighted" components that affect incrementality


COMBINING WITH BIDDING STRATEGIES
-- Estimate ad value based on future incremental value obtained

BLACK BOX ESTIMATOR INPUTS:  (see page 16-17)

Fourier Series w. Dynamic Ad stock -- for time varying incremental effects

Non-additive ad stock (direct modeling of integral)

Downsampling negatives in continuous time
   (double counting issue)


continuous time easier to deal w. than batched
-- helps avoid things like reverse-causality biasing. 
   (when bid-behavior feedback is faster than incrmentatily estimating)But 


_