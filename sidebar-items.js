initSidebarItems({"macro":[["priv_execute","`priv_execute!` executes the code block only if the current context is in privileged mode. ## Example `rust priv_execute!({     hprintln!(\"Privileged!\"); }); `"],["spawn","The tasks must be looping infinitely and call `task_exit` whenever a particular task is done. This makes it complicated to create tasks and also might introduce undefined behavior if task_exit is not called. The `spawn` macro makes it easier to define tasks. It also defines a static variable of type TaskId, which corresponds to the task created."]],"mod":[["config","Kernel configuration.  `Private`"],["events","Kernel routines which assist in Event management."],["kernel","Kernel module and routines declaration. `Private`"],["macros","Macro Definitions"],["messages","Kernel routines which assist in Inter-task Communication."],["resources","Kernel routines which assist in Resource management."],["semaphores","Kernel routines which assist in Inter-task Synchronization."],["system","Kernel Data-structures. `Private`"],["tasks","Kernel routines which assist in Task management."],["time","Kernel routines which assist in Time management."],["types","Exports types defined across other Kernel modules."],["util","Helper functions."],["utils","Utility functions.  `Private`"]]});