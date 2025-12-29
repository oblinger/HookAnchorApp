
- [gDoc](https://docs.google.com/document/d/1vmeBfJhZt6RLgHzQD7z5u4oGy1QDP7V7D1exwAIthSE/edit) - 
- [[Software  Challenge.docx]] -

### Test Overview
Design and Build a stand alone Game Scheduling System
Deliverables:
- System design: APIs, DB table designs, Key Classes, etc.


### Notes


Please provide a design for the APIs and relational database support as needed for a basketball game scheduling system.

This exercise should take 4 to 6 hours.  You are allowed to ask me any questions you wish during that period for any clarification you need.  If you find you're running out of time, don’t put the emphasis on capturing every API you need, just the major ones.

This system will allow a coach to add new games associated with their teams, and provide a time and date for each added game.




You need to design explicit support for three classes of users:  Players, Coaches, and Site Admins.

Players & Coaches can sign up and activate an account.
Admins can administer all aspects of an account, teams, players, coaches, games
Administration of admins will be outside the scope of this exercise….just assume they become admin somehow.
Coaches can set up multiple new teams
Once created, the site admin is required to explicitly activate these new teams.
Coaches can add players to the team and send invites to them to join the team.
Invites should have a link to a “simple” signup process
Automatically links the player account to the team
Using this system, coaches can obtain a link that they can send to players to sign up
Players then follow a “Simple” signup process that links them to team
Alternatively players can create an account then later request to join a existing team
Coaches can approve these requests
Players can be on multiple teams
Coaches can coach multiple teams
There can be multiple coaches managing a team
 Coaches can invite other coaches to a team
 Coaches can remove coaches or themselves from a team
 Games can be created by coaches using any team they own against any team in the system.	
Separately coaches can add or edit game date & time
Games can only be edited by the coach that created that game
All updates and additions of games create an alert for all players on the associated teams.
Required Data in tables: (but you can add references or any other additional data you need support your API…like “activated account”)
Account: username, password
Players:  name, jersey number
Coach: name
Team: name, team color
Game: date & time


In designing the back end please ensure:
In general, whatever can be done, can be undone or edited. i.e., Players can be removed from teams, requests can be “not approved,” and teams can be edited.
(design reasonable semantics and user flows for this.)
Think about the flow/journey of a user through the app: Consider the full range of APIs the front end might need to support the functionality described above as secondary APIs that would also be expected for such an app. 
Think about the additional data in tables including references to other tables that you need to support the API
The onboarding and activation and management of Admins outside the scope of the exercise. 










With this information, we would like you to deliver the following: 

Create a relational database diagram that supports these APIs

Provide light documentation “sketch” of the APIs needed to drive this app including:        NOTE: If you find you're running out of time, don’t put the emphasis on capturing every API you need, just the major ones.
Name of the API, 
quick explanation of what it does & why it is needed
inputs/outputs
Authenticated or not
GET/POST…..etc

Provide complete Documentation of 4 of the more “complex” APIs
Include what is in #2
All information need for an app developer to use the API (not a complete list)
Complete documentation of all fields, inputs/outputs
Full examples
Possible errors & messages
User role level that can access the API
Add anything else that will help the developer understand how to use

Information flow and high level user interaction flow for onboarding scenarios (That can be described as a list of bullet points and/or with diagrams).  The onboarding scenarios should include coaches, and players inviting other coaches, and players to the app and to share with them access.  Onboarding should be via email, shared link, or between two mobile devices of two people standing next to each other.
In text describe the basic flow of user actions at a high level that will be needed in these cases to accomplish this onboarding.
Help app developers understand which APIs & in what order the APIs should be called in to support the customers’ journey through the application.

A Non-complete Example of what we are looking for: onboarding flow of a coach
The coach invites another coach
Hit the CoachInvite API
Created email with a link
Other coach clicks the Email link 
Hit register API with CoachID…
Again you will not complete every flow.  But show us you can work thru some more complex ones.


Bulleted list of what the architecture might look like in AWS (or a quick diagram
Pay attention to the kinds of indices and other aspects of the architecture that will need to be in place in order to ensure that all of the APIs you sketched above will be performant.
What other services could you use to support developers
Anything other considerations that you want to highlight as critical considerations you might supply to someone who was architecting this solution.

Scaling considerations - Address how the architecture would allow you to scale
As the usage of this architecture scales up, provide a bulleted list of changes you would propose in order to allow the whole system to be efficient and responsive as the number of simultaneous users was dramatically increased, and if the amount of data was dramatically increased.
