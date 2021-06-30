use crate::winnt::*;
use crate::pefile::*;
use crate::msg::*;

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


pub trait ResourceDirectoryVisitor {
    fn init(&mut self) {}
    fn finalize(&mut self) {}

    fn enter_resource_directory(
        &mut self,
        dir: &IMAGE_RESOURCE_DIRECTORY,
        identifier: &EntryIdentifier,
    ) -> std::io::Result<()>;
    fn leave_resource_directory(
        &mut self,
        dir: &IMAGE_RESOURCE_DIRECTORY,
        identifier: &EntryIdentifier,
    ) -> std::io::Result<()>;

    fn visit_resource_data_entry(
        &mut self,
        entry: &IMAGE_RESOURCE_DATA_ENTRY,
        identifier: &EntryIdentifier,
    ) -> std::io::Result<()>;
}

pub struct MessageTableVisitor<'pefile> {
    id_stack: Vec<EntryIdentifier>,
    iterators: Vec<MessagesIterator<'pefile>>,
    pefile: &'pefile PEFile
}
impl<'pefile> MessageTableVisitor<'pefile> {
    pub fn new(pefile: &'pefile PEFile) -> Self {
        MessageTableVisitor {
            id_stack: Vec::new(),
            iterators: Vec::new(),
            pefile
        }
    }

    pub fn into_iter(self) -> impl Iterator<Item=std::io::Result<Message>> + 'pefile {
        self.iterators.into_iter().flat_map(|i| i)
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
}
impl<'pefile> ResourceDirectoryVisitor for MessageTableVisitor<'pefile> {
    fn enter_resource_directory(
        &mut self,
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
        entry: &IMAGE_RESOURCE_DATA_ENTRY,
        _identifier: &EntryIdentifier,
    ) -> std::io::Result<()> {
        if self.is_in_messagetable() {
            if self.id_stack.len() != 2 {
                panic!("unexpected resource directory layout: len={}", self.id_stack.len());
            }
            let lang_id = match self.id_stack[1] {
                EntryIdentifier::Id(x) => x,
                _ => panic!("unexpected entry identifier")
            };
            let iterator = MessagesIterator::new(self.pefile, lang_id.into(), entry)?;
            self.iterators.push(iterator);
        }
        Ok(())
    }
}
