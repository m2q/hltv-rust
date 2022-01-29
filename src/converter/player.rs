use substring::Substring;

use crate::data::*;
use crate::ConvertCollection;
use crate::Error;

use crate::tl_extensions::*;

impl<'a> ConvertCollection<'a> for Player {
    fn convert(d: tl::VDom<'a>) -> Result<Vec<Player>, Error> {
        let mut result = Vec::<Player>::new();
        // query selector for match page
        for_matchpage(&d, &mut result)?;
        // query selector for team page
        for_teampage(&d, &mut result);

        Ok(result)
    }
}

/// Parses the DOM according to the schema found on the match page.
fn for_matchpage(d: &tl::VDom, r: &mut Vec<Player>) -> Result<(), Error> {
    let selector = d.query_selector("td.player").unwrap();
    for x in selector {
        let node = x.get(d.parser()).unwrap();
        let id = node
            .children()
            .and_then(|mut i| i.next())
            .and_then(|h| h.get(d.parser()))
            .and_then(|n| n.as_tag())
            .and_then(|t| t.attributes().get("data-player-id"))
            .flatten();

        let mut p = Player::default();

        if let Some(bytes) = &id {
            let s = bytes.as_utf8_str();
            p.id = s.parse().unwrap();
        } else {
            return Err(Error::ConversionError);
        }

        let s = node.inner_text(d.parser()).to_string();
        p.nickname = s;
        r.push(p);
    }
    Ok(())
}

/// Parses the DOM according to the schema found on the team page.
fn for_teampage(d: &tl::VDom, r: &mut Vec<Player>) {
    let mut selector = d.query_selector("div.bodyshot-team-bg").unwrap();
    let parent = selector.next();
    if parent.is_none() {
        return;
    }
    for x in d.select_nodes(parent.unwrap(), "col-custom") {
        let tag = x.get(d.parser()).unwrap().as_tag().unwrap();
        let name = tag
            .attributes()
            .get("title")
            .flatten()
            .unwrap()
            .clone()
            .as_utf8_str()
            .to_string();
        let mut id = tag
            .attributes()
            .get("href")
            .flatten()
            .unwrap()
            .clone()
            .as_utf8_str()
            .to_string();
        id = id
            .substring(8, id.chars().count() - name.chars().count() - 1)
            .to_string();
        let p = Player {
            id: id.parse().unwrap(),
            nickname: name,
        };
        r.push(p);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests if the converter parses player info from a match page correctly.
    #[test]
    pub fn player_match() {
        let input = include_str!("../testdata/player_match.html");
        let dom = tl::parse(input, tl::ParserOptions::default()).unwrap();
        let result = Player::convert(dom).unwrap();
        assert_eq!(
            result[0],
            Player {
                id: 1337,
                nickname: "dist1ll".to_string()
            }
        );
        assert_eq!(
            result[1],
            Player {
                id: 1338,
                nickname: "3rr0r".to_string()
            }
        );
    }

    /// Tests if the converter parses player info from a team page correctly.
    #[test]
    pub fn player_team() {
        let input = include_str!("../testdata/player_team.html");
        let dom = tl::parse(input, tl::ParserOptions::default()).unwrap();
        let result = Player::convert(dom).unwrap();
        assert_eq!(result.len(), 3);
        assert_eq!(
            result[0],
            Player {
                id: 123,
                nickname: "dist1ll".to_string()
            }
        );
        assert_eq!(
            result[1],
            Player {
                id: 124,
                nickname: "3rr0r".to_string()
            }
        );
        assert_eq!(
            result[2],
            Player {
                id: 125,
                nickname: "rabbit".to_string()
            }
        );
    }
}