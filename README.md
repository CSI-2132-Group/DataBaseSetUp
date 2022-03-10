# Description
DataBase Set Up is a script that initializes & resets a database that is used in the final project.

The script is responsible for setting environmental variables & hard resetting the database to a certain pre-defined state.

# Set UP

## 1. Download

Download executable from release

Download SetUp.txt from release. This step is optional as the executable generates the bare minimum SetUp.txt. SetUp.txt releases are populated with specific templates.
Put SetUp.txt in the same directory as the executable

## 2. Setting up environmental variables

### Using executable enviromental variables

Run executable in admin mode. The executable will ask for inputs to define environmental variables if the update tag is inserted or one of the environmental variables is not defined.

```
DB_set_up_windows.exe update
```

### Manual setting environmental variables

There are 5 variables that need to be set inorder for the executable to function
 - DB_host
 - DB_port
 - DB_name
 - DB_username
 - DB_password

## 3. Setting Database

The executable will automatically drop all tables in the database; followed by executing the commands in SetUp.txt
