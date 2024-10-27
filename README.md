# Quantum Discord Bot

this Bot is currently under development and lacks a lot features atm

## features

all commands need to start with a ">"!  
arguments have to be seperated by a space

### commands

- echo: repeats the given argument, allows for max 5 recursions |privileged
- joke: prints a (hopefully) funny joke
- help: shows this text
- verify-all: gives a verified role to everyone |privileged
- copypasta: prints a random copypasta from a given selection
- timeout: timeouts the sender for 5 min
- status: prints the status of the configured servers
- avatar: sends the head of the specified player skin
- new-ms: temporary joke command

### other features

- adds reaction roles
- welcomes new members and gives a link to a specified channel

### setup

- download the Quantum_Bot.exe from Releases page
- run it (preferred from command line so you can see the output)
- put your bot token in the created token.txt file
- run it again
- fill out the data.toml file (more info in the data config section)
- run it again

### data config

when newly installed or when an update introduces new config fields the programm may abort and ask you to fill out the data.toml file with the following parameters:

- verified_message_id: the id of the message the bot should check for reations (for example a message stating the rules)
- verified_emoji: the emoji that needs to be reacted with in order to get a specific role
- verified_role_id: the role id of the role that should be given if someone reacted as stated above
- welcome_channel_id: the channel id where the bot should greet new members
- bot_permission_role_id: role id of the role that members need in order to perform commands tagged with privileged
- rules_channel_id: the id of the channel the bot should link to when greeting new members
- copypasta: list of copypasta texts from which the bot randomly chooses one
- server_ip: list of servers the bot lists with the status command
- greetings: list of greetings the bot randomly chooses from
