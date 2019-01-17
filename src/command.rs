use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name="todo",about="A simple command line todo app")]
pub enum Command {
    #[structopt(name="add")]
    Add {
        name:String,
    },
    #[structopt(name="list")]
    List {
        #[structopt(subcommand)]
        list_command:Option<ListCommand>
    },
    #[structopt(name="done")]
    Done {
        id:u64,
    },
}

#[derive(StructOpt, Debug)]
pub enum ListCommand {
    #[structopt(name="pending")]
    Pending,
    #[structopt(name="completed")]
    Completed,
    #[structopt(name="all")]
    All,
}
