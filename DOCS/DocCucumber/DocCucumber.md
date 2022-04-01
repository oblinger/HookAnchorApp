

Example tests

On a user profile, a user can add any special accommodations that they may need for volunteering.  This information should show up on rsvps as well.

Scenario: User adds special needs to own profile
Given a User named "Kevin"
And an Event named "PuppyExercises"
And I am logged in as "kevin@govoluntr.com" with password "poorpassword"
And I am on kevin's volunteer edit profile page
When I fill in "Special needs" with "I could use a good hat"
And I press "Save Profile"
Then I should see "Profile Saved"
When I go to the events page for "PuppyExcercises"
And I follow "RSVP"
And I click on "OK" in the alert box
Then I should see "RSVP Confirmed!"
Given I am logged in as organization "PuppyRescue"
When I go to the rsvp page for "Puppy Exercises"
Then I should see "Special accommodations needed: I could use a good hat"


In order to represent my company/school
As a company/school
I want to create a profile that includes all of the users signed up with my company/school

Scenario: Create School
 Given I am on the organization signup page
 When I fill in the following:
| Name | Red Cross |
| First name | kevin |
| Last name | zittle |
| Email | justin@govoluntr.com |
| City | San Jose |
| Phone | 916 123-4567 |
| Password | longpassword |
| Confirm               | longpassword |
And I select "School" from "Type"
And I select "California" from "State"
And I press "Let's go!"
Then I should be on Red Cross's organization page


----


http://ruby.railstutorial.org/ruby-on-rails-tutorial-book

cukes.info

railscasts.com
Branches

In order to keep the development in a clean, distinct, and historical record, we use git to keep a repository.  Once ready to push changes live, follow these steps to ensure you run into no problems.

<make changes in development branch>
git rebase master
git checkout master
git merge <dev branch>
cucumber -p all
git push
cap staging deploy
run cap staging deploy:migrations

Kevin Zittle
Professional Do-Gooder

kevin@govoluntr.com
650-353-7576

Stay Connected:
Facebook : Twitter : GoVoluntr

