#### RGR (red-green-refactor)

This is a simple command line application which does the following, given a top level directory and an extension:

* Add a watch for every subdirectory which contains files of that extension
* When files of that extension are changed in any of these watched subdirectories, run the given command

This is done by using the [notify](https://github.com/passcod/notify) library.


