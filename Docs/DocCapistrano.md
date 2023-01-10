  [__DocCapistrano__](__DocCapistrano__.md)



$ cap lets2 


$ cap 


$ cap lets2 shell   # open shell prompt

$ cap -S branch="<branchname>" deploy   # will deploy a given branch

$ cap lets2 deploy:rollback             # will rollback to the prior deploy
  --> currenly need to comment out require 'whenever/

