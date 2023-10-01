# Judge 23.1.1

The aim is to have an application that will score code kata challenges.

Features:
 - Generates a random data set
 - Invokes the challenger program, passes the data set and records execution time
 - Updates a MYSQL database with the player name and results.
 - Posts the results to a teams channel


## Dependencies

 - libssl-dev
 - [mySql](https://linuxhint.com/installing_mysql_workbench_ubuntu/)


## usage

To get the current scores

- `-p` REQUIRED Prints the score board (use `-n` to limit the lines printed)
- `-l` OPTIONAL Number of entries to print (if not provided the entire table is returned)
- `-a` OPTIONAL List all entries (without this only unique entries are selected)

To add a score

- `-c <COMMAND>` REQUIRED The command that was run (if you have parameters, include quotation marks)
- `-t` OPTIONAL Test-mode - Results will not be saved to the DB
- `-q` OPTIONAL Quiet Mode - No messages published to the teams channel

To wipe the DB:
- `-w` Wipes the DB

### Debug Options
- `-v <LEVEL>` OPTIONAL Defaults to `info`
    - error
    - info
    - warn
    - debug
    - trace
- `-o <OUTPUT>` OPTIONAL Defaults to `syslog`
    - syslog
    - stdout