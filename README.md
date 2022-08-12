# notion-cli
Command line Notion task management

![List tasks output](/images/task_list.png)

## Description

A command line app that lets you interact with a Notion task board through [Notion's API](https://developers.notion.com/reference/intro).

## Example usage

More details on command usage can be found by running `notion tasks <COMMAND> --help`

### Add

Add a new todo task to the board:

`notion tasks add "Adopt five cats" --status todo`

### List

List all tasks:

`notion tasks list`

List all tasks and their IDs:

`notion tasks list --with-id`

List all in-progress tasks:

`notion tasks list --status doing`

### Done

Mark a task as done by name:

`notion tasks done --name "Boil ocean"`

Mark a task as done by ID:

`notion tasks done 59ad73ec-efbd-4b37-ad53-02c8f3b17c56`

### Update

Move a task to todo and update it's name:

`notion tasks update 59ad73ec-efbd-4b37-ad53-02c8f3b17c56 --to todo --name "Finish civ6 game"`
