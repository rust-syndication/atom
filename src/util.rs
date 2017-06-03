use std::io::BufRead;

use quick_xml::events::Event;
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;

use error::Error;

pub fn atom_text<B: BufRead>(reader: &mut Reader<B>) -> Result<Option<String>, Error> {
    let mut buf = Vec::new();
    let mut ended = false;
    let mut s = None;

    match reader.read_event(&mut buf)? {
        Event::Text(text) => s = Some(text.unescape_and_decode(reader)?),
        Event::CData(text) => s = Some(reader.decode(&*text).into_owned()),
        Event::Start(element) => {
            let mut innerbuf = Vec::new();
            let mut depth = 1;
            let mut writer = Writer::new(Vec::new());

            writer.write_event(Event::Start(element))?;

            loop {
                match reader.read_event(&mut innerbuf)? {
                    Event::Start(start) => {
                        depth += 1;
                        writer.write_event(Event::Start(start))?;
                    }
                    Event::End(end) => {
                        writer.write_event(Event::End(end))?;

                        depth -= 1;
                        if depth < 1 {
                            break;
                        }
                    }
                    Event::Eof => return Err(Error::Eof),
                    evt => {
                        writer.write_event(evt)?;
                    }
                }

                innerbuf.clear();
            }

            s = Some(String::from_utf8(writer.into_inner()).unwrap());
        }
        Event::End(_) => ended = true,
        Event::Eof => return Err(Error::Eof),
        _ => {}
    }

    if !ended {
        let mut depth = 1;
        while depth > 0 {
            buf.clear();
            match reader.read_event(&mut buf)? {
                Event::Start(_) => depth += 1,
                Event::End(_) => depth -= 1,
                Event::Eof => return Err(Error::Eof),
                _ => {}
            }
        }
    }

    Ok(s)
}
