//! Parsing the FOOTNOTES section of a GM Mode $06 document into marker/body entries.

use mode06kit::parser::{footnote_body, parse_footnotes};

/// A verbatim excerpt of GM's `GM CAN mode $06 data` FOOTNOTES section (document `mode6-can`,
/// lines ~2145-2201), including the page-break furniture GM reprints between footnotes.
const FOOTNOTES: &str = "\
   FOOTNOTES

    1 For the following 2004 model year vehicles:

            Cadillac XLR, SRX with 4.6 liter (VIN A) engine

           The following OBDMIDs and TIDs may be defined as shown:

           OBDMID $01
               TID $10 may be replaced with TID $89

           OBDMID $39
               TID $39 may be replaced with TID $B0

            2 For the following 2004 model year vehicles:                                   GMLAN rev3
                    Cadillac XLR, SRX with 4.6 liter (VIN A) engine
                   OBDMID $39 TID $3B: Test is not supported by the vehicle. Test limits and value are invalid.

            3 For the following 2005 model year vehicles:
                    Chevrolet Equinox with 3.4 liter (VIN F)
                   OBDMID $01 TID $88 test limits and value should be divided by 1000.

Footnotes are explained on the last pages of this document.
[page 31]
GM mode $06 data definitions for GM vehicles using GMLAN diagnostic data link

                                            Some items have footnotes, defined on the last pages.

   FOOTNOTES

     6 For the following 2006 model year vehicles:
            Buick Lucerne with 4.6L (VIN Y) engine
           OBDMID $31 TID $A9 test limits and test value should be multiplied by 4 (limit result to
           \u{2013}32.768 -to- +32.767 kPa).
";

#[test]
fn every_footnote_marker_is_parsed_in_order() {
    let footnotes = parse_footnotes(FOOTNOTES);
    let markers: Vec<u32> = footnotes.iter().map(|f| f.marker).collect();
    assert_eq!(markers, vec![1, 2, 3, 6]);
}

#[test]
fn a_footnote_body_keeps_its_applicability_text() {
    let footnotes = parse_footnotes(FOOTNOTES);
    let one = footnote_body(&footnotes, 1).expect("footnote 1");
    assert!(one.starts_with("For the following 2004 model year vehicles:"));
    assert!(one.contains("Cadillac XLR, SRX with 4.6 liter (VIN A) engine"));
    assert!(one.contains("TID $10 may be replaced with TID $89"));
}

#[test]
fn a_wrapped_footnote_body_joins_its_lines() {
    let footnotes = parse_footnotes(FOOTNOTES);
    let six = footnote_body(&footnotes, 6).expect("footnote 6");
    assert!(
        six.contains("OBDMID $31 TID $A9 test limits and test value should be multiplied by 4")
    );
    assert!(six.contains("\u{2013}32.768 -to- +32.767 kPa"));
}

#[test]
fn page_furniture_and_gmlan_tags_are_stripped_from_bodies() {
    let footnotes = parse_footnotes(FOOTNOTES);
    for footnote in &footnotes {
        assert!(!footnote.body.contains("FOOTNOTES"), "{}", footnote.marker);
        assert!(!footnote.body.contains("[page"), "{}", footnote.marker);
        assert!(!footnote.body.contains("GMLAN rev"), "{}", footnote.marker);
        assert!(
            !footnote.body.contains("Footnotes are explained"),
            "{}",
            footnote.marker
        );
    }
    // The GMLAN tag trailed footnote 2's first line; it must be gone but the text kept.
    let two = footnote_body(&footnotes, 2).expect("footnote 2");
    assert!(two.starts_with("For the following 2004 model year vehicles:"));
    assert!(!two.contains("GMLAN"));
}

#[test]
fn a_missing_marker_has_no_body() {
    let footnotes = parse_footnotes(FOOTNOTES);
    assert_eq!(footnote_body(&footnotes, 9), None);
}

#[test]
fn text_without_a_footnotes_section_yields_none() {
    let footnotes = parse_footnotes("01   03   0A   Some Test   0 to 9 counts   1 count / bit");
    assert_eq!(footnotes.len(), 0);
}
