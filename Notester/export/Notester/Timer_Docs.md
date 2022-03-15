# Notester.Timer\_Docs --

    EXAMPLES
      (start) "qq="   (stop)  "."  (delete last) "-"

    TIMER -- Very quick way to record time usage.  Command format:
             CNTL-SPACE  [ time ] [ timer_name ] [ "'" comment ] command
             TIME        [ "+" ]  [ hour ":" ]   minute

    COMMANDS
            '='       Starts timer with optionally specified name, at optionally specified time.
            '.'       Stops<<>>records current timer & records its time. (time specified is subtracted from duration;  name specified change timer recorded))
            ' '       Stops<<>>records current timer & starts newly specified timer
            ' '       Space by itself pauses <<>> resumes current timer.  paused time accumulated at end of interval when it is recorded.
            
            '<<>>' '?'   Goto Timers Note (help info there too)
            '['       Display time usage report as Excel spreadsheet
            ']'       Display graphical time visualization
            'ESC'     Cancel action
            '-'       Deletes previously recorded time interval, and resets the activity start time to begining of period.


    EXAMPLES:
           gg8:30=    Set current timer to 'gg' and the start time to the nearest 8:30
           30.        Records current timer stopped 30 minutes ago.  
           +40f space Records 40 min activity on current timer, and begins new 'ff' activity.
           20r space  Records current activity ended 20 min ago, and the 'r' activity began 20 minutes ago.
           
         
           
    DETAILS:
    - LOG FILE    <<>>  TimerLog.txt
