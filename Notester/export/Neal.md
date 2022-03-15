# Neal --



    > Here's what I would like you to do for the several hours you have agreed to spend on this tomorrow nite:
    >
    > Reread your original patent application
    > Read the latest amendment as it explains how we modified the claims to reflect earlier cited art, and what we felt at that time was the novelty of your invention.
    > Read the newly cited Stuart art.  Don't bother reading Yakhini.
    > Read my analysis below of the novelty of your invention relative to the Stuart art.
    > State where your invention is novel relative to the Stuart art and why adding in a distance metric (per Yakhini) would still not make your invention obvious.

    Neal, as I have come to depend on, you have organized the information required, and the questions sought in a very compact form.  Thanks as always!

    * I have read original, and our amendment.
    * I read Stuart.  Here is my initial reactions Stuart verses our patent.

    - Both we and Stuart depend centrally on unsupervised induction to generalize user context, yet this is where the similarities end.
      They apply this general technique to web log data.  They must contend with representing trajectories of a visitor's traversal thru
      a web site.  We, on the other hand, contend with the multiple independent sources of context data we are provided for each
      user (see our patent for detailing of this).  Also we contend centrally with context as induced by the unstructured natural language
      requests (queries) made by our users.  This feature space does not map onto web-log traversals (nor to web log traversals map onto
      our feature space.)

    - The outputs of the two systems differ both in form and goal.  The output of our system is a set of computable context terms.
      Each term is used to categorize the users themselves.  Our primary aim is to have new meaningful descriptors of the users themselves, to
      be used computationally to improve subsequent retrieval.  Not to visualize how the input terms relate to each other across multiple
      user's interactions.  

      Suart on the other hand is primarily aiming exactly at such visualizations of the relation between these input features of the 
      user trajectory.  As they say "Although the method is applicable to a wide range of transactions, the method is particularly 
      useful for understanding how visitor's behave on a web site."  (SUART Col5 line 8-11) The output relates features in the input
      (different URLs visited by users).  We provide groupings of the user's themselves without any concern for capturing how the 
      underlying retrieved educational resources are related to each other.  Our grouping of the users themselves are guided by subsequent
      utility those groupings serving the retrieval process, not in visualizing relationships among the retrieved resources. (or even visualizing relationships between users)

    - This difference in form of the output can is highlighted, by our use of "corrective" information as users explicitly correct
      context values automatically chosen by out system.  In this case the user is essentially providing a piece of supervised knowledge
      declaring that they are not an example of one type of user context, but rather are an element of a different user context.

      This corrective input only makes sense in a situation like ours where the output is used to drive some down stream computational
      task where context terms can have correct or incorrect values for specific users.  

      In Suart, on the other hand, there is not place for such input, because such an input makes no sense.  The output of their system, 
      visualizes trends of a groups of visitors, it makes no sense for a single user to disagree about the summary provided of all user
      trajectories.  Had the summary been different, that user's placement would have been different, but given the summary provided, 
      there place within it, is simply a function of what they did on the web site, it makes no sense to claim it is wrong.


    - To be concrete about the difference in perspective here consider an example of an induced context for us
      - Technically savvy user vs. non-technical sales person:  Such a distinction is found to be relevant because
        it separates technical verses marketing resources very well.
      - A visualization of a very common backtracking done by many users that visit some portion of a web site.
      Both are clusterings, but they are very different in form, inputs and goal.


    Summary:
    - We use a combination of unsupervised and supervised learning, they only use unsupervised since that is all that makes sense in their context.
    - Our input space must capture lexical features of unstructured features; their input space must capture the temporal and structural features of
      a web space link traversal.
    - Our output is organized around users, not the retrieved resources; their output is organized around relationships between the traversed URLs.
    - Out output is a computable (non-visual) concept whose utility is measured by its use is subsequent retrieval.
      Their output is a visualization of how the structure of the underlying resources (web site) is used during user traversal.  The utility of  
      such clusterings is measured by the clarity of this visualization of the structure of the underlying resources (URLs).






    -----------------
    NEALS COMMENTS:


    Target, such as an individual or a group, moves between data and the navigation data is selected and aggregated to indicate a behavior in relation to the objects.
    Is it the aggregating and representation of navigational data that is supposed to be similar to Dan's invention?
    Fig 1 - there is some comparison of behavior between entities and for the same entity over time.
    Fig 2 - some working with clusters or groups and historical analysis.
    goal of this may be to analyze web usage logs of clusters of users, especially related to e-commerce and there is some graphical representation based on the text mining, there is some generation of context vectors for a visitor's website and a labeling of clusters, there is a relational database.
    Fig 19 - context vectors are based on website content, web sites are clustered and labeled based on content (vs. non-content data we use).
    ? how does Stuart do clustering? 
    Stuart is not about search but about navigation - different domain?  May see sequential navigation as analogous to exploring a proposed results set from a search.  Goal is a bit different - purchase of a good or service vs. selection of a resource.  Stuart wants to understand why people abandon Internet shopping carts and what needs to be changed about the Web site pages,including dynamically, to increase sales including how to manage special promotions and see their effects.  Stuart invention is to represent all the available high dimensional information (definition col 3 lines 2-17) for effective decision making.  Is this for benefit of one individual?  Purpose of invention is consolidate and correlate high dimensional properties of web site utilization info (col 3 lines 36-40).  Do particular types of visitors behave differently? - goal is to personalize the experience but not (I believe) to personalize it for similar users or to explicitly present a user context for selection - this could be a way we modify the claims.  Goal is to present this overview to web site operator, not directly to modify what is presented to user or similar users? This invention pcks an indicia to aggregate one user's navigation at site and articulate a behavior associated with it.  The indicia (our user context) has associated attributes - but if it's not developed via unsupervised learning and a distance metric, would it have been obvious to do so if combine Yakhini?  Stuart does want to change the content presented to web users so as to increase their purchasing - in some sense the same as we want to make use of context cluster discovery to do a better job of presenting resources to users in response to their search based on content in the query.

    #### NEAL, I agree the goals are different as you say here.


    Stuart does relate behavior to a cluster of visitors - but how does he identify the cluster?
    col 5 (lines 43-47) - non-content related behavior of users, or at least how their behavior varies in different time periods - is this the same as non-topical information context (raw context) - I don't think so - it's still all related to content selection and when user does it, or in what sequence. Stuart's system can't know other things about user (but could it be extended to know their connection speed, for example?).  He does mention collection of demographic information (col 7 line 29). We do track path of user thru the resources presented as they explore, just as Stuart tracks progress of user thru Web site. Difference is we got list from a search but we do present top ones first, just as a web site is hierarchical, so I'm not sure this difference is significant.
    relational memory database is different than our user interaction database but how? (col 8 top)
    attributes (col 8 top) can be defined for entities (top level domain of visitor) or relationship (time spent by a particular user at a particular web page).
    aggregate relationship info to identify summary behavior for a group (like females) - and store this in relational memory database.  But how does Stuart identify the group for which he does this? (see col 8 lines 44-55) - it's like the identification of the group is obvious and doesn't need to be described - but that's the whole purpose of Dan's patent application.
    Stuart compares behavior for a single entity over time as well as between entities.
    Stuart discussed how clusters could be a group of visitors who one or more common demographic characteristics (col 9 lines 33-39).
    Stuart's invention definitely makes use of historical data - behavior at the web site over time, and displays it graphically for understanding.  But focus of his invention doesn't appear to be the discovery of clusters of users but to taking known clusters and understanding their differential use of a web site vs. other clusters.

    #### I agree.  We both relate retrieved resources with users (the retriever).  But we make no attempt to uncover structure among the retrieved resources.
    Visualizing this structure is the focus of Suart, on the other hand.  We focus on the structure among users as a function of their lexical queries.


    Stuart is going to explain how he identifies a cluster from their web usage without having demographic information (col 11 line 41 and next paragraph, Fig. 5). Sounds like an objective for clustering process is set and attributes are selected for the clustering process - this is very different than our process for identifying new user contexts. 
    col 13 - Stuart is focused on representing number of users in a user context by indicia size superimposed on web pages, not in providing a choice of user context to individual user - or identify such as context so it could inform the resources suggested for the user during the search.  So how does he get validation from users (supervised learning) that the identified user contexts are accurate? (is this part of Dan's application or just the unsupervised identification?)

    #### No as we say above, he does not (and cannot) accept validation.   big difference.

    Focus of Dan's invention is the identification of new user contexts, not so much on how changes in the resources returned in the search affect user behavior - is that used to validate the identified user contexts in #7? (use of supervised learning to augment unsupervised?) - is it the combo of the two that is novel in Dan's application?.  We are starting with a fixed set of resources to search, although our system could be used by providers of resources to tune their offerings. It's primarily focused on the users themselves.  Stuart is interested in representing the data in his equivalent of the user interaction database to improve the web site (the resources) for clusters of visitors so as to improve the web site (col 14 lines 23-46) and Dan's invention is focused on identifying the clusters themselves by analyzing the data (vs. pre-picking the clusters based on obvious characteristics) so users have a more efficient search experience. Key is how Stuart identifies non-obvious clusters from data. Clearly he doesn't use a distance metric since he had to cite Yakhini.

    #### We are on the same page here.


    Should we add mention of unsupervised learning to primary claim, or even combo of unsupervised and supervised if that's in Dan's specification?

    #### Our use of unsupervised, and supervised learning shows significant difference with Suart.  Others have considered such a combination, so we are
    not patenting the combination itself, but rather our use of such a combination in this situation.

    Looking at page 14 of Dan's specification, we need to add some specifics from here to our primary claim.  Does Stuart have a batch process that occurs asynchronously and constantly to apply unsupervised machine learning - does he analyze the records in batch, even if he needs Yakhini for a distance metric?  How does Stuart identify clusters, even though he doesn't use a distance metric? Why wouldn't it have been obvious for Stuart to use  a distance metric to analyze web usage patterns of different users to look for clusters?  Output of Dan's clustering is potential new context attributes. I don't believe that Stuart actually discovers the attributes for his entities and relationships by any mathematical means (col 7 line 20 - col 8 line 30).  A behavior is an aggregation of relationships.
    (col 17 lines 1-16). Appears that the context vector for Stuart is based on a text analysis of the content of the pages navigated by the user or the selected cluster of users, and not on the non-content related aspects of users (bandwidth, organizational entity, etc.).  col 17 lines 32-43 "Each target (user or cluster of users) may be associated with a summary context vector that generally classifies the areas of interest for a particular target."
    ? how are context vectors used?

    #### yes, though such an extension of Suart might be considered obvious.

    *** here's how Stuart identifies clusters - very different from what we do, really uses text analysis of surfed pages, or just says all men or all women - col 18 lines 14-25 - "The information profile can thereby be used to identify clusters or groups. For example, a group could be identified that is interested in handheld electronic devices..." - so Stuart either uses content analysis (needs Yakhini for a distance metric - but it would still be focused on content of the surfed web pages) or he picks all the men or all the women.  He also uses some demographic info to cluster users - col 18 lines 28-42 "Fig 18 shows an alternative or additional method 270 for identifying or classifying visitors.  Domain info is extractef from an visitor's IP address.  col 18 lines 45-50 "Context vectors from many visitors' web sites can then be clustered in Step 278 to identify collections of visitors having similar informational content.  With the visitors so clustered, in step 280, a label is then assigned to each of the clusters depending on the general information content vector."  col 19 lines 4-17 "In step 296, a text-mining web robot or "webbot" is launched to the URL addresses identified in step 294 in order to interrogate or analyze the content of web sites corresponding to the URL addresses. Such "webbots" are well-known in the art. (Dan - your application only relies on the query to supply content related info re user). 

    #### We primarily rely on the user's query for context, but we also use terms from the retrieved resources as Suart does.

    In step 298, context vectors are created and summarized based on web site content and stored in the relational database or other database.  After the context vectors have been generated for many visitors, the context vectors are clustered (how?? - does it matter if a distance metric was used?) to identify collections of visitors coming from domains having a similar content, in step 300.  Next, in step 302, context vectors within each cluster may be summarized and labeled so as to identify each cluster. Data pertaining to the labeled clusters may then be stored in a database.  (col 19 lines 29-32) - "It is further appreciated that the webbot may collect information pertaining to other aspects of a web site, or web pages within a web site - ex aesthetics of web site - ... (col 19 line 42-44) "It is then possible to cluster such context vectors so as to identify clusters of web pages as described above - however, based on aesthetics rather than textual content."
    We need to talk about how Dan's use of context vector in the specification is different than that discussed by Stuart and how we explicity make use of user's confirmation of user context in the overall system.  We look at result set similarity which is different than the content analysis of the surfed web pages of Stuart.

    #### Use of validation is central to our rebuttal.  Use of a similarity metric is not a significant different.  we should not attempt to hang our hat on this
         since it is an obvious extension of Suart.




    Well Neal, let me know if there are further questions, and thanks again for the wonderful organization.  In the end, it did take a chunk of time, but it 
    would have been significantly longer without your focusing.

    Cheers, 
    Dan
