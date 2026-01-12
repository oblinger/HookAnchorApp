
| [[ABIO Planning]]                | -------------------------- [[ASP]]                                                                                                  |     |
| -------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------- | --- |
| External                         | [Repo](https://github.com/oblinger/alienbio),     [Docs](https://oblinger.github.io/abio-docs/),     [[legacy/ABIO Legacy\|Legacy]] |     |
| [[ABIO Research]]                | [[ABIO References]]                                                                                                                 |     |
| [[ABIO Planning\|Planning Docs]] | [[ABIO PRD]], [[ABIO Features]], [[ABIO Notes]],                                                                                    |     |
| - Execution Docs                 | [[ABIO Todo]], [[ABIO Roadmap]],                                                                                                    |     |
| [[ABIO Docs\|User Docs]]         | [[Alienbio User Guide]],  [[Architecture Docs]],  [[api/index\|API Reference]]                                                      |     |
|                                  |                                                                                                                                     |     |



  tmux send-keys -t ABIO C-q Escape Escape C-c
  tmux send-keys -t ABIO "stty sane" Enter




  TARGET=$(/opt/homebrew/bin/tmux list-clients -F "#{client_activity}:#{session_name}" | sort -rn | head -1 | cut -d: -f2)

  /opt/homebrew/bin/tmux send-keys -t "$TARGET" C-q Escape Escape C-c
  /opt/homebrew/bin/tmux send-keys -t "$TARGET" "stty sane" Enter
  /opt/homebrew/bin/tmux send-keys -t "$TARGET" "reset" Enter



  TMUX LOCKUP DIAGNOSIS: My tmux session is locked - I can't type anything, but scrolling works and tmux timestamps appear. Manual Ctrl+Q/Escape/Ctrl+C don't help. Before I kill the pane, run these diagnostics to figure out what's wrong:
  # Replace SESSION with the locked session name (e.g., ABIO)
  SESSION=ABIO
  # Check process tree in the pane
  ps -o pid,ppid,state,command -g $(tmux display-message -t $SESSION -p '#{pane_pid}')
  # Check file descriptors
  lsof -p $(tmux display-message -t $SESSION -p '#{pane_pid}') 2>/dev/null | grep -E "0u|1u|2u"
  # Check terminal settings if possible
  tmux send-keys -t $SESSION "stty -a" Enter
  # Try sending reset sequence (probably won't work but try)
  tmux send-keys -t $SESSION C-q Escape Escape C-c
  This may be related to WhisperFlow sending text to tmux. Check ~/Library/Logs/WhisperFlow.log for recent sendToTmux entries to see what was sent before the lockup.
