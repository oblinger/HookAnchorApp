# notes to internet name renaming --


    Here's a short Python script that does the BluePages lookup to convert from
    a name (in various forms) to an internet email address.  Usage:

    % lookup tessa lau
    tessalau@us.ibm.com
    % lookup lau, tessa
    tessalau@us.ibm.com
    % lookup oblinger
    oblio@us.ibm.com
    % lookup v castelli
    vittorio@us.ibm.com

    --Tessa


    #!<<>> python

    import string, sys, os

    args = string.join(sys.argv[1:])
    if ',' in args:
             # already in last, first format; do nothing
             pass
    elif ' ' in args:
             # assume first space divides first name from last name
             i = args.index(' ')
             args = args[i+1:] + ', ' + args[:i]

    cmd = 'phone %s format internet' % args
    phone = os.popen(cmd)
    addr = string.strip(phone.read())
    ret = phone.close()
    if ret:
             # phone returned error
             print args
    else:
             print addr
