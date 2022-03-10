# DataBaseSetUp
Data Base Set Up is a script that intializes & reset a databse that is used in the final project.

The script is responsible for setting enviormental variables & hard reseting database to certian pre defined state.

# Set UP

## 1. Download

Dowload executable from release

Download SetUp.txt from release. This step is optional as the executable generates the bare minimum SetUp.txt. SetUp.txt releases are populated with specfic templates.
Put SetUp.txt in the same directory as the executable

## 2. Setting up enviromental variables

### Using executable enviromental variables

Run executable in admin mode. The executable will ask for inputs to define enviromental variables if the update tag is inserted or one of the enviromental variables is not defined.

```
DB_set_up_windows.exe update
```

The executables sets the en

### Manual setting enviromental variables

There are 5 variables that need to be set
 - DB_host
 - DB_port
 - DB_name
 - DB_username
 - DB_password

## 3. Setting Database

The executable will automatically drop all tables in database; followed by executing the commands in SetUp.txt
