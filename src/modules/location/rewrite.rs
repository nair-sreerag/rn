use regex::Regex;

use super::LocationStage;

pub struct RewriteStage {
    original_string: String,
    grouping_regex: String,
    replacement_regex: String,
    should_redirect: bool,
}

impl LocationStage for RewriteStage {
    type StageInitArgs = (String, String, String, bool);

    fn init(args: Self::StageInitArgs) -> Self {
        // let z = T.url;

        let previous_stage_string = args.0;
        let grouping_regex = args.1;
        let regex_to_replace = args.2;
        let redirection_flag = args.3;

        RewriteStage {
            original_string: previous_stage_string,
            grouping_regex,
            replacement_regex: regex_to_replace,
            should_redirect: redirection_flag,
        }
    }

    fn process(mut self) -> String {
        let original = self.original_string;

        let extractor_regex = Regex::new(&self.grouping_regex).unwrap();
        let replacement_regex = self.replacement_regex;

        let replaced = extractor_regex.replace_all(&original, &replacement_regex);

        replaced.to_string()

        // if let Some(captures) = extractor_regex.captures(&original) {

        // } else {
        //     original
        // }
    }
}

#[cfg(test)]
pub mod tests {

    use regex::Regex;

    use super::*;

    #[test]
    // #[should_panic]
    pub fn check_regex_for_single_case() {
        let original = String::from("/api/v1/123");
        let er = String::from("/api/v1/(.*)");
        let rr = String::from("/api/v1/zz?id=$1");

        let rewrite_stage = RewriteStage::init((original, er, rr, false));

        assert_eq!("/api/v1/zz?id=123", rewrite_stage.process())
    }

    #[test]
    pub fn check_for_multiple_regex_cases() {
        let original = String::from("/api/v1/123/456");
        let er = String::from("/api/v1/(.*)/(.*)");
        let rr = String::from("/api/v1/zz?id=$1$2");

        let rewrite_stage = RewriteStage::init((original, er, rr, false));

        assert_eq!("/api/v1/zz?id=123456", rewrite_stage.process())
    }
}
