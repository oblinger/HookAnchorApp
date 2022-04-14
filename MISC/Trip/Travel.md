


            defclass Feature {

            }

            defclass Population {

            }




            // ACTIVATION_FN is a function of STATE1, STATE2, ...,  INPUT1, INPUT2, ..., and CONFIG
            // tspec( fn(varargs=number, state=expr, config=expr, result=number) )

            CFG ActivationFn == fn( head:Symbol, rest: Number keys: None
                    state: STATEVECTOR, config: CONFIGFORM, result: Number )



            // SNAPSHOT -- A snapshot is a mapping of some set of features onto an activation level for each feature.  NOTE: a snapshot is a recording of activation levels without regard to whether they are input or output features.

            // EXPERIENCE -- is a sequence of snapshots.  In some cases these snapshots are to be interpreted as having occurred temporally in order in the world.  In other cases shapshots simply represent distinct experiences w/o any implied temporal ordering.  (Each snapshot has a boolean flag called 'chained'.  When true it signified that this shapshot is to be interpreted as having occured temporally immediately after the preceding snapshot within the EXPERIENCE.
            // Note: we use the term 'experience' since a key usage of experience is to encode data obtained from the embodiment of an agent.  In other caseses however the 'experience' a learning algorithm is exposed to is 'experience' internally derived from other computation.  It makes no different to the receiving learning algorithms, in both cases it represents experience presented from outside over which it is expected to process / learn.


            // SNIPIT -- a short sequence of snapshots that are chained.  A snipit is 'short' in the sense that the system does not attempt to temporally organize within the timeframe of the snipit, though it may operate on concepts that depend on the order of the snapshot chaining.
            // (for example 'increasing' is a concept that does not encode temporal relationships within the snapshots, but it does depend on their ordering)




# Finding Travel Deals

  http://theflightdeal.com        






TheFlightDeal     http://theflightdeal.com        
- Select Source City
- Each deal is for a short time (one day)  but you can travel over a range 
