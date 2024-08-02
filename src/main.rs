mod stack;
use stack::Stack;

mod transitions;

fn main() {
    {
        let url = format!(
            "vscode://vadimcn.vscode-lldb/launch/config?{{'request':'attach','sourceLanguages':['rust'],'waitFor':true,'pid':{}}}",
            std::process::id()
        );
        std::process::Command::new("code").arg("--open-url").arg(url).output().unwrap();
    }
    println!("Hello, world!");
}

