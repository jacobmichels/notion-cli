# notion-cli
Command line Notion task management

![List tasks output](/images/task_list.png)

## Description

A command line app that lets you interact with a Notion task board through [Notion's API](https://developers.notion.com/reference/intro).

## Notion setup

This app requires some setup in Notion before use.

### Integration
1. Visit https://www.notion.so/my-integrations and click Create new integration
2. Give the integration a name and ensure your personal workspace is selected
3. Select the following capabilities and click submit
![Required capabilities](/images/capabilities.png)
4. Copy the internal integration token and give it to the app with the command `notion config token set <TOKEN>`

### Page and Database
1. Ensure you're in the same workspace your integration was linked to
2. Click Add a page on the left sidebar
3. Click Templates in the newly created page
4. Expand the Personal dropdown on thr right side and click ✔️ Task List, then Use This Template at the top. You now have a Notion Page and Database setup for use with the cli
5. In the new page, click Share at the top right. Then click the textbox, select your integration and click invite.
5. Run the command `notion config database list` to list eligible databases for task management. The newly created Database should be listed in the command's output. Copy the ID of the database
6. Paste the database ID in the following command: `notion config database set <ID>`

Now you're all set up! 🎉

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
