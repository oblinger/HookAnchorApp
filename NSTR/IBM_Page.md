# IBM\_Page --

    All (bcc'd),

    As you know, IBM Research's external website (and your people and/or project pages) are staged on an internal server accessible at the URL <<>>
    That staging server had, until last Thursday, used the filesystem "DFS" and was accessible via samba.watson.ibm.com with your Agora ID.  However, we have now migrated your www-stage content to a new, company-wide, filesystem called GSA (Global Storage Architecture).  You should all have GSA IDs at this time.

    To access your directories, you can 
    RECOMMENDED:  FTP: continue to FTP to www-stage but log in with your GSA ID/password.  Your www-stage filespace will still be located at <<>> (i.e. <<>> or <<>> ).  If you have difficulty logging in and accessing your directory and your GSA id and agora ID are the same, telnet to www-stage or webserver1 and use the passwd command to synchronize your GSA and old Agora ID passwords (we believe that everyone has been required to change their passwords again by now, but just in case, that's how you can get them to match).

    Microsoft Windows: Map a network drive in to \\watgsa.watson.ibm.com\projects\w\www-stage\your-directory-name (log in with GSA ID/password)

    AIX/Linux/Unix:  cd to <<>>   (If you are planning to copy files using the cp and mkdir commands from *IX, you must set the umask first (umask 002) before running the commands, otherwise your files and directories will not be web visible.)

    This information can also be found on the webteam site in the "External Pages" section: <<>>

    The list of who owns what on www-stage, GSA:
    (If a directory you thought you owned is listed with the GSA ID "respage" (communication's ID) and you need to recover that directory, please contact us)


    Don't know your ID?  Forgot your password?
    First, try checking the pdf file above to see what your GSA ID is.
    If you need to update your GSA ID, find out your IDs, or create new directories, use <<>>
    For GSA ID issues in the US, I recommend this shortcut:  <<>>  (Login with your IBM Intranet password)
    Outside the US, if daatmaster (above) doesn't help, try this URL:  <<>>

    More GSA Notes:
    Content on the staging server must be accessible to communications - the staging server is intended for staging content to be published on www.research.  In addition, any content that we were not permitted to access on agora/DFS we didn't migrate.  Please check your GSA directories now to ensure that all content was migrated, we are only saving the old copy (DFS) of the staging server files for two weeks, and then they'll be permanently deleted.  If something is missing, email webmaster immediately so that it can be located.

    Be aware that your new directory permissions use webadm as the owner (that's us) and specify a group for group-level ownership.  Your name is in that group list; if you want to transfer control or add someone to the group so that they can also edit the content, let us know (webmaster@watson.ibm.com).

    Externalization requests will continue to function as they have in the past, go to <<>> and click "Externalization Request" (log in with IBM Intranet ID/password, not your GSA ID).

    GSA migrations for w3.research.ibm.com will begin soon.
