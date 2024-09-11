
https://www.notion.so/sportsvisio/2024-08-25-Softbank-Response-Data-613ae5c478964348aadba22678337f2f?pvs=4


- WHEN: For each moment in time we must assess if a shot attempt is happening >**97%**
- WHO: What is the id of a given player **> 95%**
- POINTS: Shot origination 2-point, 3-point or Free throw **>?**
- RESULT: Whether the shot was made or missed **> 88%**

Additionally, we capture the following as needed for items above listed in blue:

- The time intervals when each player is on court. **> .95 F1 score**
    
- Whether the game is in progress or in timeout. **> Not done.**
    
- Who is possessing the ball over at each moment in time. **> ~60% at time of shot**
    
- The shooter’s team color and jersey number: Around 92%, and 60%, respectively.
    

NOTES:

- F1 is a standard statistical measure but is more comprehensive than percent accuracy— it combines precision (chances each prediction is correct) and recall (chances of correctly identifying all cases).
- We will have 2-point, 3-point accuracy measures in Q4 of this year when we release shot charts.

# Sources

- WHEN
    - NOTE: using older version of the system
        - ‣
- Player-minutes
    - NOTE: over LT10
        - ‣
- WHO
    - NOTE: these are proxies for player ID using halo annotations
        - ‣
    - NOTE: over baller10 using the old system:
        - ‣