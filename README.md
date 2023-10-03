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

### Scoreboard

- `-p` REQUIRED Prints the score board (use `-n` to limit the lines printed)
- `-l` OPTIONAL Number of entries to print (if not provided the entire table is returned)
- `--unique players` OPTIONAL Only show the first score per a player
- `--unique binaries` OPTIONAL Only show the first score per a binary
- `--unique language` OPTIONAL Only show the first score per a binary
- `--player <playername>` OPTIONAL only show scores for the given player (can be used multiple times to select multiple players)
- `--language <language>` OPTIONAL only show scores for the given language (can be used multiple times to select more than one language)
- `--binary <binary_name>` OPTIONAL only show scores for the given binary (can be used multiple times to select more than one binary)
- `--sort <player/binary/language/time>` OPTIONAL sort the list by the givn column (default is time)

**_Filters:_**  The filters will be aplied in the order they are provided.

### Running a testS

- `-c <COMMAND>` REQUIRED The command that was run (if you have parameters, include quotation marks)
- `-L <language>` REQUIRED The language you wrote your binary in
- `-t` OPTIONAL Test-mode - Results will not be saved to the DB
- `-q` OPTIONAL Quiet Mode - No messages published to the teams channel

To wipe the DB:
- `-w` Wipes the DB (requires root)

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