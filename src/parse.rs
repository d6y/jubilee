use soup::prelude::*;

#[derive(Debug, PartialEq)]
pub struct BookAvailability {
    location: String, // e.g., shelf number
    availability: String, // e.g., on loan
}

impl BookAvailability {
    // Implementation for the SPYDUS html output (version 9?)
    pub fn from_html(html: &str) -> Option<BookAvailability> {
        let soup = Soup::new(&html);
        
        // The <div> with class holdings is a table of title availability (if any)
        let section = soup.class("holdings").find()?;

        // Inside that div, the 3rd and 4th TDs (actually across 2 TRs) gives the 
        // location and availability text description
        let book_cells: Vec<String> = section.tag("td").find_all().skip(2).take(2).map(|cell| cell.text()).collect();

        match &book_cells[..] {
            [loc, available] => Some(BookAvailability { 
                location: loc.trim().to_string(), 
                availability: available.trim().to_string() 
                }),
            _ => None,
        }
    }
}

mod test {
    use super::BookAvailability;
    use std::fs;
    #[test]
    fn test_parse_one_result() {
        let expected = BookAvailability { 
            location: String::from("599.938"), 
            availability: String::from("Reservation Allocation Expired (BHCC) (Set: 08 May 2019)") 
         };
        let html = fs::read_to_string("data/one-result.html").expect("HTML missing?");
        assert_eq!(BookAvailability::from_html(&html), Some(expected));
    }
}