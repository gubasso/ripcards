pub const ERROR_MSG_NOT_PROJECT_ROOT: &str = "
RipCards Error: Not a Git repository.\n
\n
The 'ripc' command must be run within a Git repository.\n
A RipCards project needs:\n
\n
(1) the presence of a '.git' directory;\n
(2) to be executed at the project root;\n
\n
Please navigate to a ripissue project root directory and try again.\n
\n
If you believe this is a valid project directory, you can create an empty
\x20'.git' directory or initialize a new Git repository with 'git init'.\n
\n
For more information, visit https://github.com/cwnt-io/ripcards\n
";
