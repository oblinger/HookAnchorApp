# Sh.Job-Control -- Job control (jsh)

         bg [%jobid ...] 
         fg [%jobid ...] 
         jobs [-p|-l] [%jobid ...] 
         jobs -x command [arguments] 
         kill [ -signal ] %jobid 
         stop %jobid ... 
         suspend 
         wait [%jobid ...] 
     
     
     
         When the shell is invoked as jsh, Job Control is enabled  in 
         addition  to  all  of the functionality described previously 
         for sh.  Typically Job Control is enabled for  the  interac- 
         tive  shell  only.   Non-interactive shells typically do not 
         benefit from the added functionality of Job Control. 
     
         With Job Control enabled every command or pipeline the  user 
         enters  at  the terminal is called a job.  All jobs exist in 
         one of  the  following  states:  foreground,  background  or 
         stopped.   These  terms are defined as follows:  1) a job in 
         the foreground has read and write access to the  controlling 
         terminal;  2)  a job in the background is denied read access 
         and has conditional write access to the controlling terminal 
         (see  stty(1));  3)  a  stopped  job  is a job that has been 
         placed in a suspended  state,  usually  as  a  result  of  a 
         SIGTSTP signal (see signal(5)). 
         Every job that the  shell  starts  is  assigned  a  positive 
         integer,  called  a job number which is tracked by the shell 
         and will be used as an identifier  to  indicate  a  specific 
         job.   Additionally the shell keeps track of the current and 
         previous jobs.  The current job is the most recent job to be 
         started  or  restarted.   The previous job is the first non- 
         current job. 
     
         The acceptable syntax for a Job Identifier is of the form: 
     
           %jobid 
     
         where, jobid may be specified in any of the  following  for- 
         mats: 
     
              % or +    for the current job 
     
              -          for the previous job 
     
              ? specify the job for which  the  command  line 
                        uniquely contains string. 
     
              n         for job number n, where n is a job number 
     
              pref      where pref is a unique prefix of the  command 
                        name  (for example, if the command ls -l name 
                        were running in the background, it  could  be 
                        referred  to  as  %ls);  pref  cannot contain 
                        blanks unless it is quoted. 
     
         When Job Control is  enabled,  the  following  commands  are 
         added to the user's environment to manipulate jobs: 
     
         bg [%jobid ...] 
              Resumes the execution of a stopped  job  in  the  back- 
              ground.   If  %jobid  is  omitted  the  current  job is 
              assumed. 
     
         fg [%jobid ...] 
              Resumes the execution of a stopped  job  in  the  fore- 
              ground, also moves an executing background job into the 
              foreground.  If %jobid is omitted the  current  job  is 
              assumed. 
     
         jobs [-p|-l] [%jobid ...] 
     
         jobs -x command [arguments] 
              Reports all jobs that are stopped or executing  in  the 
              background.   If  %jobid  is omitted, all jobs that are 
              stopped or running in the background will be  reported. 
              The following options will modify/enhance the output of 
              jobs: 
              -l    Report the process group ID and working directory 
                   of the jobs. 
     
              -p    Report only the process group ID of the jobs. 
     
              -x    Replace any jobid found in command  or  arguments 
                   with  the corresponding process group ID, and then 
                   execute command passing it arguments. 
     
         kill [ -signal ] %jobid 
              Builtin version of kill to provide the functionality of 
              the kill command for processes identified with a jobid. 
     
         stop %jobid ... 
              Stops the execution of a background job(s). 
     
         suspend 
              Stops the execution of the current shell (but not if it 
              is the login shell). 
     
         wait [%jobid ...] 
              wait builtin accepts a job identifier.   If  %jobid  is 
              omitted  wait  behaves as described above under Special 
              Commands. 
