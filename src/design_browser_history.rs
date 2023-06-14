use std::collections::VecDeque;

/// You have a browser of one tab where you start on the `homepage` and you can visit another
/// `url`, get back in the history number of `steps` or move forward in the history number of
/// `steps`.
///
/// Implement the `BrowserHistory` class:
///
/// * `BrowserHistory(string homepage)` Initializes the object with the `homepage` of the browser.
///
/// * `void visit(string url)` Visits `url` from the current page. It clears up all the forward
/// history.
///
/// * `string back(int steps)` Move `steps` back in history. If you can only return `x` steps in
/// the history and `steps > x`, you will return only `x` steps. Return the current `url` after
/// moving back in history at most `steps`.
///
/// * `string forward(int steps)` Move `steps` forward in history. If you can only forward `x`
/// steps in the history and `steps > x`, you will forward only `x` steps. Return the current `url`
/// after forwarding in history at most `steps`.
struct BrowserHistory {
    urls: VecDeque<String>,
    current: usize,
}

impl BrowserHistory {

    fn new(homepage: String) -> Self {
        let mut urls = VecDeque::new();
        urls.push_back(homepage);
        Self { urls, current: 0 }
    }

    fn visit(&mut self, url: String) {
        self.urls.truncate(self.current + 1);
        self.urls.push_back(url);
        self.current += 1;
    }

    fn back(&mut self, steps: i32) -> String {
        if self.current == 0 {
            self.urls[0].clone()
        } else {
            let index = (self.current as i32 - steps).max(0) as usize;
            self.current = index;
            self.urls[index].clone()
        }
    }

    fn forward(&mut self, steps: i32) -> String {
        let steps = steps as usize;
        let index = (self.current + steps).min(self.urls.len() - 1);
        self.current = index;
        self.urls[index].clone()
    }

}

#[cfg(test)]
mod tests {
    use super::BrowserHistory;



    #[test]
    fn example_1() {
        let mut browser_history = BrowserHistory::new("leetcode.com".to_string());
        browser_history.visit("google.com".to_string());
        browser_history.visit("facebook.com".to_string());
        browser_history.visit("youtube.com".to_string());
        let result = browser_history.back(1);
        assert_eq!(result, "facebook.com");
        let result = browser_history.back(1);
        assert_eq!(result, "google.com");
        let result = browser_history.forward(1);
        assert_eq!(result, "facebook.com");
        browser_history.visit("linkedin.com".to_string());
        let result = browser_history.forward(2);
        assert_eq!(result, "linkedin.com");
        let result = browser_history.back(2);
        assert_eq!(result, "google.com");
        let result = browser_history.back(7);
        assert_eq!(result, "leetcode.com");
    }

}
