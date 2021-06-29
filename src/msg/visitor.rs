use crate::winnt::*;
use crate::pefile::*;

#[allow(dead_code,non_camel_case_types)]
enum ResourceType {
    RT_CURSOR = 1,
    RT_BITMAP = 2,
    RT_ICON = 3,
    RT_MENU = 4,
    RT_DIALOG = 5,
    RT_STRING = 6,
    RT_FONTDIR = 7,
    RT_FONT = 8,
    RT_ACCELERATOR = 9,
    RT_RCDATA = 10,
    RT_MESSAGETABLE = 11,
}

pub struct MessageTableVisitor {
    id_stack: Vec<EntryIdentifier>
}
impl MessageTableVisitor {
    pub fn new() -> Self {
        MessageTableVisitor {
            id_stack: Vec::new()
        }
    }
    fn is_in_messagetable(&self) -> bool {
        match self.id_stack.first() {
            Some(f) => match f {
                EntryIdentifier::Id(id) => {
                    *id == (ResourceType::RT_MESSAGETABLE as u16)
                }
                _ => false
            }
            _ => false
        }
    }

    fn print_messagetable (
        &mut self,
        pefile: &PEFile,
        entry: &IMAGE_RESOURCE_DATA_ENTRY,
    ) -> std::io::Result<()> {
        for msg in pefile.messages_iter(0, entry)? {
            println!("{}: '{}'", msg.msg_id, msg.text);
        }
        Ok(())
    }
}
impl ResourceDirectoryVisitor for MessageTableVisitor {
    fn enter_resource_directory(
        &mut self,
        _pefile: &PEFile,
        _dir: &IMAGE_RESOURCE_DIRECTORY,
        identifier: &EntryIdentifier,
    ) -> std::io::Result<()> {
        match identifier {
            EntryIdentifier::NoIdentifier => (),
            _ => self.id_stack.push(identifier.clone()),
        }
        Ok(())
    }
    fn leave_resource_directory(
        &mut self,
        _pefile: &PEFile,
        _dir: &IMAGE_RESOURCE_DIRECTORY,
        identifier: &EntryIdentifier,
    ) -> std::io::Result<()> {
        match identifier {
            EntryIdentifier::NoIdentifier => (),
            _ => { let _ = self.id_stack.pop(); }
        }
        Ok(())
    }

    fn visit_resource_data_entry(
        &mut self,
        pefile: &PEFile,
        entry: &IMAGE_RESOURCE_DATA_ENTRY,
        identifier: &EntryIdentifier,
    ) -> std::io::Result<()> {
        if self.is_in_messagetable() {
            self.print_messagetable(pefile, entry)?;
        }
        Ok(())
    }
}
