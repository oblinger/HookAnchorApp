# Idea.Neural\_Password --


    GOAL:  Use human memory to store and validate identity.  with following properties
     - Very Strong guarantee thta 'same' person is using system
     - Does not require physical device
     - Can be repeatedly used w/o compromise of "password"
     - Many random attemps will not break system
     - Human ability to retain password will not degrade over time
     - Password is not hard to learn


    BITS
     - 2 bits, which direction to look for associated letter for each image



    APPROACH

    - User remembers a cartoon story composed of a set of panels,  each panel:
      * occurs in a scene, with a protaganus, a verb, and a modifying object
      * MATH:  ( 20Protag * 20Verbs * 20Modifiers ) ^ 2panels  *  Secret word

    - Variable complexity password
      - Many bytes (10-20) required to establish that software instance is at one time was under control of true user.
      - Fewer bytes (2-5) required to restart new session
        (Most important that others cannot watch and steal)
      - Fewest bytes (2) needed to resume timedout session.

    User cannot be tricked into interacting with a rogue interface
     - User's 'login' screen is very unique and a largely constant image so they will not interact with a different image.
     - We must control code used to interact with user.


    EXAMPLE CARTOON

    USER CHOOSES MOOD: Mundane, Funny, Sick, Happy, Action, Chic, Adult, Religious
    SYSTEM PICKS:  OBJECTIVE:    GetLunch     OBJECTIVE MODIFIER:  
    SYSTEM PICKS:  PROTAGANOUS:  Truck, Bunny    HELPER:  Octopus      HINDER:  Rake     OBJECT: Web Browswer
    CHOOSE:  User chooses 'Jimmy' the 'truck,' 'fuzzy' the bunny, 'bernard' the 'octopus', and 'ricky' the rake

    25-OBJECTIVES:  MUNDANE: GetLunch, ReadyForBed,   SICK: RemoveZit,   FUNNY: FoodFight, 
    25-AGENTS:      Truck,Car,Boat   Lion,Dog,Fish  OldMan,Girl,Baby
    25-OBJECT:
    25-TOOLS:       Shovel,Hammer,Saw
