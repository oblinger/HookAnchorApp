# Formalization --



     GUST -- Guaranteed Unforgable (distributed) Self Test

    A Guaranteed Unforgable Distributed Self Test is a distributed process that generates a certificate which
    can be used to prove certain properties must hold through reasoning built the certificate alone.

    - Let W be the web.  In particualr the set of documents, programs, certificates, etc. that have been published
      for public inspection.  The output of a GUST published on W.

    - Let N be a set of computation nodes capable of sending messages to each other and to a public transcript on W 
      (where how a certificate is compiled)

    - Let P be some peer-to-peer distributed process that is published on W, and executed on some of the nodes N.

    - During execution the nodes, N, compile a certificate, C, by sending messages to be published on the web, W, as
      part of the certificate.  (It is important that observers can see the construction of the certificate over time 
      since reasoning about who could have know what when, is a crucial aspect of GUST proofs)

    - Let H be the set of 'honest' nodes, they are the nodes that are executing the published process P exactly as stated.
    - Let D be N\H the set of 'dishonest' nodes, they are not executing P, but rather some other process specfically 
      designed to thwart the goals of self testing.  (We must prove they will fail)

    - Let A be a set of assertions (generally about the execution within the network, but it could be about anything)
      with are to be tested.  In our case these assertions will be statistical guarantees about behavors of the nodes
      within the network.

    A GUST proof is logical proof concluding assertions A directly from:
     - The complied certificate C
     - The published process P
     - Knowledge about the size of H,  in our case it is simply that H is non-empty
