

Datafile Timestamping Service

[[Timestamps Folder]] 


### FreeTSA.org

https://www.freetsa.org/index_en.php

#### CREATING THE TIMESTAMP
```



brew install curl   # Get the lastest version of curl

# Create the hash files
openssl ts -query -data kmr.zip -no_nonce -sha512 -out kmr.zip.tsq   
openssl ts -query -data proj.zip -no_nonce -sha512 -out proj.zip.tsq  

/usr/local/opt/curl/bin/curl -H "Content-Type: application/timestamp-query" --data-binary '@kmr.zip.tsq' https://freetsa.org/tsr > kmr.zip.tsr
/usr/local/opt/curl/bin/curl -H "Content-Type: application/timestamp-query" --data-binary '@proj.zip.tsq' https://freetsa.org/tsr > proj.zip.tsr


```

#### VERIFY TIMESTAMP

```


brew install wget
wget https://freetsa.org/files/tsa.crt        # These need to be retrieved at verification time to really prove they are real
wget https://freetsa.org/files/cacert.pem     # this too.

openssl ts -reply -in kmr.zip.tsr -signer cacert.pem -text    # Prints Timestamp Information back out

openssl ts -verify -data kmr.zip -in kmr.zip.tsr -CAfile cacert.pem -untrusted tsa.crt  # Re-hashes & uses signer info to verify .tsr is valid


```

### DigiStamp

Digistamp cannot handle files bigger than 512M  (Ugh!)

So I took the kmr.zip.tsq and proj.zip.tsa file which has the SHA512 has of kmr.zip and proj.zip then I created a stamp for that!

See the FreeTSA.org section above for the creation of the SHA512 hash


### IdenTrust


#### CREATING THE TIMESTAMP
```



brew install curl   # Get the lastest version of curl

openssl ts -query -data kmr.zip -no_nonce -sha512 -out kmr.zip.tsq   # Create the hash file

/usr/local/opt/curl/bin/curl -H "Content-Type: application/timestamp-query" --data-binary '@kmr.zip.tsq' http://timestamp.identrust.com > kmr.zip.tsr

```

#### VERIFY TIMESTAMP  (could not get their PEM file!)
	