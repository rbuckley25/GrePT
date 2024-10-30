# High Level Plan

Provide a directory (of images) and a does this contain "prompt" as an input.
Search through the directory and return a list of all files which contain the prompt object.


## Tasks
 -[] Search thorugh dir for files
 -[] Add default limit to args
 -[] Pass each file to the Gemeni upload API
 -[] Query the Gemeni api uisng the prompt to see if it contains the required data.
 -[] Return a list of all files that match the prompt.