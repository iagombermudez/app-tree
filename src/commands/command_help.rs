pub fn execute_help() -> Result<(), String> {
    println!(
        "{}",
        format!(
            "App-tree is a CLI tool that you can used to manage your CLI commands. 
With this tool you can save commands like you would do using alias in bash, but you can also
execute multiple related commands at the same time. For example, you could create a repositories
folder, and include all your projects inside it. Then, you could execute the repositories folder,
calling 'app-tree open repositories' and it will open all of your projects however you like.

List of commands:

<add>:      Useful for adding a new action. You can either add an action as a standalone using 
            'app-tree add <name-of-action> <command>', or you can add it inside a group of actions
            using 'app-tree add <name-of-branch> <name-of-action> <commmand>.
            Params (they must be given in order):
                <name-of-branch>: (optional) the name of the branch the action will be stored in
                <name-of-action>: the name of the action to be added
                <command>: the command that will be executed when calling the action
            Example: 
                $ app-tree add repositories my-awesome-project 'nvim ~/Source/repositories/my-awesome-project'

<remove>:   Removes an action. Follows the same pattern as the <add> command, but it is not necessary to include the 
            action command. 
            Params (they must be given in order):
                <name-of-branch>: (optional) the name of the branch the action will be stored in
                <name-of-action>: the name of the action to be added
            Examples:
                $ app-tree remove repositories my-awesome-project 
                $ app-tree remove repositories //removes all the actions and the branch
                $ app-tree remove my-executable //if my executable is a standalone project, it removes it 

<list>:     List all the actions that have been added.
            Example:
                $ app-tree list

<open>:     Executes a given action or a set of actions if the parameter given is a branch.
            Params (they must be given in order). Both parameters are optional, but at least one must be given: 
                <name-of-branch>: (optional) the name of the branch the action will be stored in
                <name-of-action>: (optional) the name of the action to be added
            Examples:
                $ app-tree open repositories my-awesome-project 
                $ app-tree open repositories //open all the actions and the branch
                $ app-tree open my-executable //if my executable is a standalone project, it opens it 

"
        )
    );
    return Ok(());
}
