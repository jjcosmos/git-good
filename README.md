This is slightly cursed, so bear with me.

This is a process that runs in the background, checking if a process has recently exited (by polling every n seconds). When that happens, a batch file (or technically anything that can execute via cmd) will be run. My setup, as well as the batch at "git-good/batch/commit_and_push.bat" runs git add, commit, and push. This DOES assume that your batch file is in a valid git directory. The exe also assumes that config.ini is contained in the same folder as itself.
