

- My fix for running tests:  [here](https://stackoverflow.com/questions/57901794/making-pytest-quieter-when-run-from-pycharm) 



# HOW TO

## Remote Debugging

// On remote instance
sudo apt-get update
sudo apt-get install python3 python3-pip


// Add this at top of your remove .py script
import ptvsd
ptvsd.enable_attach(address=('0.0.0.0', 5678))
print("Waiting for debugger attach...")
ptvsd.wait_for_attach()


// Might be important for this agent to be running on Mac
eval "$(ssh-agent -s)"
