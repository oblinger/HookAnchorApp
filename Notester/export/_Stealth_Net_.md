# \_Stealth\_Net\_ --

    will take a peek at this soon.

    i have spent the evening trying to solve a "privacy" issue
    of my own.

    It makes my brain hurt.  (I like problems like that)

    So the challenge is to allow citizens of china read messages on a
    'stealth net' built into the internet in a way that:
    (1) The state cannot tell which citizens are accessing the stealth net
    (2) When the state compromises the stealth net (they get access)
        they can only identify a limited set participating people and machines.

    Right now there is a wikipedia subtree that many chinese use to talk
    about china.  The state keeps blocking it, there are ways around that block
    but I fear that the govt knows about this way around and tracks people
    that do it.

    The parameters of the problem:
    - Assume your adversary can see and block all net traffic.
    - Assume that the population is divided into a set of 'safe' citizens
     and 'unsafe' citizens (who will report anything they find out)
    - Assume that most safe citizens can identify a large number of fellow
     safe citizens
    - Assume that your have two types of computer systems, domestic and foreign.
     the foreign systems can be known by the state, but their contents can never
     be searched by the state, and foreign computers can communicate without
      being seen by the state.
    - Assume that the contents of a domestic computer cannot be seen by the state
     until it has been compromised, but then all of its contents are known.


    My idea is that a citizen can download a stealth page by downloading
    three normal web pages and XORing specific contents of images on those
    pages.  Two of the three pages come from places like CNN.com or
    commonly access sites in china.  the third page uses the low order bits
    in gif or mp3 files to encode an XOR pattern placed against the XOR of a hash
    compression of the first two sites.

    I am pretty sure this can be done in a way that the space of
    triple (or quadruple etc) web pages is so large that it is essentially
    unsearchable.

    The state must know where to look.

    Now the trick is to build a setup protocall so that we can infer
    which citizens are compromized, so we can avoid telling them
    what machines we are using to replace identified and blocked machines.

    Another trick which I think I understand, is how a safe citizen might
    suggest a set of friend citizens that they believed where safe, without
    giving away that they were safe (assuming they were wrong about one of their
    suggestions)

    I think I have a solution for this second problem.


    I don't have this whole thing figured out, but it seems one must rely
    heavily on the fact that there exist foreign accomplices that are
    beyond the reach of the state.

    interesting problem with some rather practical implications huh.


    -dan

    my only fear about such software is the bin ladin would probably also
    make good use of it.
