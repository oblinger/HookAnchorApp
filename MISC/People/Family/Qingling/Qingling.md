  [Interviewing](Interviewing.md)
  [Interviewing](MISC/People/Family/Qingling/Interviewing.md)


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