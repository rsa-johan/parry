use parry::Cmd;

fn main() {
    let mut cmd = Cmd::new();
    cmd.arg("name".into(), Some("n".into())).parse();

    let mut name: String = String::from("Null");

    if let Some(n) = cmd.get("name") {
        name = n.to_owned();
    }

    println!("Name: {:#?}", name);
}
