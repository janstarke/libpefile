use crate::msg::Message;
use crate::pefile::*;
use crate::winnt::*;
use encoding_rs::*;
use from_bytes::StructFromBytes;
use packed_size::*;


pub struct MessagesIterator<'pefile> {
    pefile: &'pefile PEFile,

    remaining_blocks: u32,

    low_id: u32,
    high_id: u32,
    current_id: u32,

    rde_offset: usize,
    block_offset: usize,
    entry_offset: usize,

    error_handler: Option<fn(std::io::Error) -> ()>,
    lang_id: u32,
    encodings: [&'static Encoding; 2],
}

impl<'pefile> MessagesIterator<'pefile> {
    pub fn new(
        pefile: &'pefile PEFile,
        lang_id: u32,
        resource_entry: &IMAGE_RESOURCE_DATA_ENTRY,
        error_handler: Option<fn(std::io::Error) -> ()>,
    ) -> std::io::Result<Self> {
        let rde_offset = pefile
            .get_raw_address(resource_entry.OffsetToData as usize)
            .unwrap();
        let mrd = MESSAGE_RESOURCE_DATA::from_bytes(pefile.full_image(), rde_offset)?;

        // go one step back, because we go one blocksize forward before the first result is returned
        let block_offset = rde_offset + MESSAGE_RESOURCE_DATA::packed_size() - MESSAGE_RESOURCE_BLOCK::packed_size();

        Ok(Self {
            pefile,
            current_id: u32::MAX,
            remaining_blocks: mrd.NumberOfBlocks,
            rde_offset,
            block_offset,
            entry_offset: usize::MAX,
            low_id: 0,
            high_id: 0,
            error_handler,
            lang_id,
            encodings: [WINDOWS_1252, UTF_16LE]
        })
    }

    fn handle_error(&self, why: std::io::Error) {
        if let Some(error_handler) = &self.error_handler {
            error_handler(why);
        }
    }

    pub fn do_next(&mut self) -> std::io::Result<Option<Message>> {
        
        let blocksize = MESSAGE_RESOURCE_BLOCK::packed_size();

        // find for next block
        if self.current_id >= self.high_id {
            if self.remaining_blocks == 0 {
                return Ok(None);
            }
            self.remaining_blocks -= 1;
            self.block_offset += blocksize;
            let block = MESSAGE_RESOURCE_BLOCK::from_bytes(
                self.pefile.full_image(),
                self.block_offset,
            )?;

            self.entry_offset = self.rde_offset + block.OffsetToEntries as usize;
            self.low_id = block.LowId;
            self.current_id = block.LowId;
            self.high_id = block.HighId;
        } else {
            self.current_id += 1;
        }

        let entry = MESSAGE_RESOURCE_ENTRY::from_bytes(self.pefile.full_image(), self.entry_offset)?;
        let text_offset = self.entry_offset + MESSAGE_RESOURCE_ENTRY::packed_size();
        let message_length = entry.Length as usize - MESSAGE_RESOURCE_ENTRY::packed_size();

        let message = self.encodings[entry.Flags as usize]
            .decode(&self.pefile.full_image()[text_offset..text_offset + message_length])
            .0
            .to_string();
        self.entry_offset += entry.Length as usize;

        Ok(Some(Message::new(self.current_id, self.lang_id, message)))
    }
}

impl<'pefile> Iterator for MessagesIterator<'pefile> {
    type Item = Message;
    fn next(&mut self) -> Option<Self::Item> {
        match self.do_next() {
            Ok(v) => v,
            Err(why) => {
                self.handle_error(why);
                None
            }
        }
    }
}
