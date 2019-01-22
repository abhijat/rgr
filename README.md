#### RGR (red-green-refactor)


[![asciicast](https://asciinema.org/a/4qxM8uyQIDTMXYW2KzuoTkKOx.svg)](https://asciinema.org/a/4qxM8uyQIDTMXYW2KzuoTkKOx?t=20)


This is a simple command line application which does the following, given a top level directory and an extension:

* Add a watch for every subdirectory which contains files of that extension
* When files of that extension are changed in any of these watched subdirectories, run the given command

Watching the paths for changes is done by the [notify](https://github.com/passcod/notify) library.

The code can be built using the rust toolchain, install it from [here](https://rustup.rs/)

Then run `cargo build`

##### Running the program:

The program assumes configuration is present in a JSON file. By default it expects a config.json in the same 
directory, but a file can be supplied with the -c or --config flag,

`rgr --config ./sample_configs/django.json`

There are a couple of sample config files in the sample_configs directory. 

##### Configuration:

The following keys are supported as part of the configuration:
```
{
    "command_config": {
      "binary_path": "/tmp/webapp/venv/bin/python", (1)
      "args": [ (2)
        "/tmp/webapp/manage.py",
        "test",
        "app"
      ]
    },
    "debounce_in_seconds": 5, (3)
    "file_extension_to_watch_for": ".py", (4)
    "path_to_watch": "/tmp/webapp" (5)
}
```

1. The command which will be run when a file changes
2. The arguments to be passed to the command
3. Events are clubbed for this duration, passed through to notify. I recommend to keep this around 2-5 seconds
4. File extension which will be scanned recursively under the root path
5. The top level path, the program looks for all subdirectories beneath this (recursively)

`RGR_LOG_LEVEL` is an environment variable which can be used to set the level of logging for the program. 
By default this is set to `debug`.
