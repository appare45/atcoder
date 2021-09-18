#!/bin/sh
if [ "$#" -gt 0 ]
then 
  cp -R template/ $1
  code $1
else
  read NAME
  cp -R template/ $NAME
  code $NAME
fi