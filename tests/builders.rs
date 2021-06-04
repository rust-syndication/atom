#![cfg(feature = "builders")]

use atom_syndication::extension::*;
use atom_syndication::*;
use std::collections::BTreeMap;
use std::str::FromStr;

fn join_lines(text: &str) -> String {
    text.lines().map(|line| line.trim()).collect()
}

#[test]
fn test_builders() {
    let does = vec![
        PersonBuilder::default().name("John Doe").build(),
        PersonBuilder::default().name("Jane Doe").build(),
    ];

    let feed = FeedBuilder::default()
        .namespace(("ext".to_string(), "http://example.com".to_string()))
        .title("Feed Title")
        .subtitle(Text::plain("Feed subtitle"))
        .id("urn:uuid:60a76c80-d399-11d9-b91C-0003939e0af6")
        .updated(FixedDateTime::from_str("2017-06-03T15:15:44-05:00").unwrap())
        .icon("http://example.com/icon.png".to_string())
        .logo("http://example.com/logo.png".to_string())
        .rights(TextBuilder::default().value("© 2017 John Doe").build())
        .authors(does.clone())
        .contributors(does.clone())
        .category(CategoryBuilder::default().term("technology").build())
        .category(CategoryBuilder::default().term("podcast").build())
        .generator(GeneratorBuilder::default().value("Feed Generator").build())
        .link(
            LinkBuilder::default()
                .rel("self")
                .href("http://example.com/feed")
                .build(),
        )
        .link(
            LinkBuilder::default()
                .rel("alternate")
                .href("http://example.com")
                .build(),
        )
        .entry(
            EntryBuilder::default()
                .title("Entry Title")
                .id("http://example.com/article/1")
                .updated(FixedDateTime::from_str("2017-06-03T15:15:44-05:00").unwrap())
                .authors(does.clone())
                .category(CategoryBuilder::default().term("technology").build())
                .category(CategoryBuilder::default().term("podcast").build())
                .contributors(does.clone())
                .links(vec![
                    LinkBuilder::default()
                        .rel("alternate")
                        .href("http://example.com/article/")
                        .build(),
                    LinkBuilder::default()
                        .rel("enclosure")
                        .href("http://example.com/audio.mp3")
                        .mime_type("audio/mpeg".to_string())
                        .length("1000".to_string())
                        .build(),
                ])
                .published(FixedDateTime::from_str("2017-06-01T15:15:44-05:00").unwrap())
                .summary(Text::plain("Entry summary"))
                .rights(Text::plain("© 2017 John Doe"))
                .content(
                    ContentBuilder::default()
                        .value("Entry content".to_string())
                        .build(),
                )
                .source(
                    SourceBuilder::default()
                        .title("Entry Title")
                        .id("http://source.example.com/content/article/1")
                        .updated(FixedDateTime::from_str("2017-06-03T15:15:44-05:00").unwrap())
                        .build(),
                )
                .extension(("ext".to_string(), {
                    let mut map = BTreeMap::new();
                    map.insert(
                        "title".to_string(),
                        vec![ExtensionBuilder::default()
                            .name("ext:title")
                            .value("Title".to_string())
                            .attr(("type".to_string(), "text".to_string()))
                            .build()],
                    );
                    map
                }))
                .build(),
        )
        .build();

    assert_eq!(
        join_lines(&feed.to_string()),
        join_lines(
            r#"
                <?xml version="1.0"?>
                <feed xmlns="http://www.w3.org/2005/Atom" xmlns:ext="http://example.com">
                    <title>Feed Title</title>
                    <id>urn:uuid:60a76c80-d399-11d9-b91C-0003939e0af6</id>
                    <updated>2017-06-03T15:15:44-05:00</updated>
                    <author>
                        <name>John Doe</name>
                    </author>
                    <author>
                        <name>Jane Doe</name>
                    </author>
                    <category term="technology"/>
                    <category term="podcast"/>
                    <contributor>
                        <name>John Doe</name>
                    </contributor>
                    <contributor>
                        <name>Jane Doe</name>
                    </contributor>
                    <generator>Feed Generator</generator>
                    <icon>http://example.com/icon.png</icon>
                    <link href="http://example.com/feed" rel="self"/>
                    <link href="http://example.com" rel="alternate"/>
                    <logo>http://example.com/logo.png</logo>
                    <rights>© 2017 John Doe</rights>
                    <subtitle>Feed subtitle</subtitle>
                    <entry>
                        <title>Entry Title</title>
                        <id>http://example.com/article/1</id>
                        <updated>2017-06-03T15:15:44-05:00</updated>
                        <author>
                            <name>John Doe</name>
                        </author>
                        <author>
                            <name>Jane Doe</name>
                        </author>
                        <category term="technology"/>
                        <category term="podcast"/>
                        <contributor>
                            <name>John Doe</name>
                        </contributor>
                        <contributor>
                            <name>Jane Doe</name>
                        </contributor>
                        <link href="http://example.com/article/" rel="alternate"/>
                        <link href="http://example.com/audio.mp3" rel="enclosure" type="audio/mpeg" length="1000"/>
                        <published>2017-06-01T15:15:44-05:00</published>
                        <rights>© 2017 John Doe</rights>
                        <source>
                            <title>Entry Title</title>
                            <id>http://source.example.com/content/article/1</id>
                            <updated>2017-06-03T15:15:44-05:00</updated>
                        </source>
                        <summary>Entry summary</summary>
                        <content>Entry content</content>
                        <ext:title type="text">Title</ext:title>
                    </entry>
                </feed>
            "#
        )
    );
}
