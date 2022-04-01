# AR.Click\_Tester --

    [ON SLAVE]
    > run sniffer
    $ ipconfig

    [ON MASTER]
    > run sniffer
    $ ipconfig               # to get IP addr
    $ cd scripts
    jython clicker.py
      -h mstrIP            # address where systems runs
      -m mstrIP            # listens for entity clicks
      -s slaveIP           # slave blinks entities
