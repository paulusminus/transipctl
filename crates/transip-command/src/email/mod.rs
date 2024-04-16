use parse_display::{Display, FromStr};

mod r#box;
mod forward;
mod list;

const DELETE: &str = "delete";
const INSERT: &str = "insert";
const ITEM: &str = "item";
const LIST: &str = "list";
const UPDATE: &str = "update";

type DomainName = String;
type MailAddress = String;

#[derive(Default, Display, FromStr)]
#[display("{local_part} {password} {max_disk_usage} {forward_to}")]
#[from_str(default)]
pub struct Box {
    local_part: String,
    password: String,
    max_disk_usage: u32,
    forward_to: String,
}

pub enum EmailCommand<T> {
    Delete(DomainName, MailAddress),
    Insert(DomainName, T),
    Item(DomainName, MailAddress),
    List(DomainName),
    Update(DomainName, T),
}

#[cfg(test)]
mod test {
    use super::Box;

    #[test]
    fn test_from_str() {
        "info CumGranoSalis 456 ".parse::<Box>().unwrap();
    }

    #[test]
    fn test_display() {
        let mailbox = Box {
            local_part: "info".into(),
            password: "CumGranoSalis".into(),
            max_disk_usage: 450,
            forward_to: "".into(),
        };
        dbg!(mailbox.to_string());
    }

}