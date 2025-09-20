=[[At]]    [VP Eng](https://www.linkedin.com/in/andrew-huddle-27123227/)  [[@PlayOnSports]] 



# Log

## 2025-09-19  email


Andy, 
We are making good progress on our end, implementing the job scheduling infrastructure needed for the delivery of highlights.  This is a fortuitous convergence on our side since we have recently made significant changes to how AI jobs are run (improving execution speed), and we needed a new scheduler to support this.  We are killing three birds with one stone, adding this, supporting PlayOn, and doing a green field reimplementation of that job queue inside our central system, which was overdue!

Below we have an update to the schema that we will be returning for completed games.  As discussed, it is now much closer to the Pixellot format.  Let us know what you think.

We also wanted to verify that the ffmpeg grabbing operation we are currently performing on your system is the same one we will be doing in production, and that the versions of the files we are retrieving now are identical to those we will be retrieving in December and beyond.  Additionally, do you have specific games in mind that you would like us to use for the "sneakernet" test we are conducting this month, where we send you JSON result files for you to feed into your system manually?  We want to select games that will best demonstrate the highlight games we will be getting later.  How many games do you want to test this month?  3-games?  10-games?  (For us, the effort needed for each game is relatively light, so we can accommodate any testing you want to do on your side.)
[[Attach/SCRIPT_ANALYSIS]] 