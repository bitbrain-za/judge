## Dependencies

 - libssl-dev
 - [mySql](https://linuxhint.com/installing_mysql_workbench_ubuntu/)


## usage

To get the current scores

- `-p` REQUIRED Prints the score board (use `-n` to limit the lines printed)
- `-l` OPTIONAL Number of entries to print (if not provided the entire table is returned)

To add a score

- `-n <NAME>` REQUIRED The name you want displayed on the scoreboard
- `-c <COMMAND>` REQUIRED The command that was run (if you have parameters, include quotation marks)

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