#!/usr/bin/env bash

curl --request POST \
  --url https://api.sendinblue.com/v3/smtp/email \
  --header 'accept: application/json' \
  --header 'api-key:xkeysib TOTOKEY' \
  --header 'content-type: application/json' \
  --data '{  
   "sender":{  
      "name":"TOTO TATA",
      "email":"TOTO.TATA@gmail.com"
   },
   "to":[  
      {  
         "email":"TITI.TUTU@gmail.com",
         "name":"TITI TUTU"
      }
   ],
   "subject":"Hello world",
   "htmlContent":"<html><head></head><body><p>Hello,</p>This is my first transactional email sent from Sendinblue.</p></body></html>"
}'
