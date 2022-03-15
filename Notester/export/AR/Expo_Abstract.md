# AR.Expo\_Abstract --

    Technical Objective:
      Learn computer-based procedures automatically
      by multiple demonstrations of those procedures.

    Technical Approach:
      We record a procedure as a sequence of system states and user actions.
      We learn a graph of state/action nodes which captures
      the similar portions of each trace as well as the branching/looping
      structure inherent in the traces.  The resulting graph of
      nodes and associated rules can be used to automatically or
      semi-automatically execute the learned procedure in future situations.

      We use an extension of
      traditional Hidden Markov Modeling where we induce rules for the next
      state function in each node of the model.
