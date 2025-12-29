
[Claude.ai]()   [[LLM]] 



## IDEAS
-  **PRD FIRST** - Always build out a PRD first for any major feature or application. Then explicitly get confirmation from me before beginning to build.
- **ONE COPY** - Be sure with the Builder chain that there's only one version of the binary on my development machine at a time, avoid any copying of this binary in our testing environment, some links or other approaches should be used instead. When building deployment packages copying can happen, but then at the end of the build of the deployment all those copies should be gone other than the final deployment package.


## SETUP
### Permissions

  alias cc="claude --dangerously-skip-permissions"

  1. Permissions Command:
  /permissions
  Use this in-chat to configure tool allowlists

  2. CLI Allow Tools:
  claude --allowedTools="Bash,Read,Write"
