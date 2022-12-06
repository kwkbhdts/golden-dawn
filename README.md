# golden-dawn
A daily directory maker.

## Binary (for Windows)
See golden-dawn/bin directory.

## Usage
Edit config.toml and run golden-dawn.exe.  
Using with Task Scheduler will enable auto run on suitable trigger.

### config.toml

| Key | Meaning |
| ------------- | ------------- |
| parent_dir | The path of parent directory. golden-dawn creates daily directory in parent directory.  |
| date_format | Name format of daily directory.  See also https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html#specifiers . |
| days_to_move | If elapsed days of directory reachs this value, golden-dawn move that directory to "old" directory. |
| days_to_remove | If elapsed days of directory reachs this value, golden-dawn remove that directory from "old" directory. |
