use spider::website::Website;

pub fn configure_crawler(url: &str) -> Website {

    let mut website: Website = Website::new(url);

    // apply configurations
    website
        .with_respect_robots_txt(true)
        .with_subdomains(false) 
        .with_tld(false) 
        .with_delay(5000) 
        .with_request_timeout(None)
        .with_http2_prior_knowledge(false)
        .with_user_agent(Some("innovation team scraping experiment bot v1.0").into())
        .with_on_link_find_callback(Some(|link, html| {
            println!("Link: {}", link.inner());
            (link, html)
        })) // SM: prints link on each page iteation, useful for progress tracking
        .with_headers(None)
        .with_blacklist_url(None) 
        .with_proxies(None); // SM: may be worth setting up later to help distribute traffic?

    // return website
    website

}