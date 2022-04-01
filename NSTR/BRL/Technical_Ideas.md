# BRL.Technical\_Ideas --


    *  A Representation is expressed as a generative grammar, G, of production rules: <<>>
    *  Instances(G) is the possibly infinite set of element that can be generated from G.

    *  A grammar which is an extensible template will designate some of its productions as
       V --> <<>> as extensible.


    *  A representation space, S, is a list of ground instances for each extensible variable
       and a production limit, Li, for each gi in G.  The Instances(S) are those elements
       that can be generated within the production limits and variable instantiation limits
       set in S.   Instances(S) is always finite, and always a subset of Instances(G) that it
       is base on.

    *  Size(S) = | Instances(S) |

    *  S is the smallest represetation of a set of obeservations O from Instances(G) where
       O is subset of Instances(S) and for all rep spaces S' of G where O is a subset of Instances(S'),   
          Size(S) <= Size(S')  

    *  A mapping, M(G,H), translates elements, g in grammar G, into elements h in grammar H.
       M is 1-to-1 but not necessarily onto.

    Let G be the grammar for the observed data.
    Let O = O1, ..., On be observed instaces of G.

    * A mapping, M, from G onto some new representation H has a CompactionRatio, r, where
      r = log( Size(SmallestSpace(M(O),H))  <<>>  Size(SmallestSpace(O,G) )


    We are looking for new representations, H, and Mapping M, such that r is as large
    as possible.
