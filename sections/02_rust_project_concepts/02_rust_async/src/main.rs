// This is an example showing off async web requests,
// and some of the patterns we went over in the lesson.
//
// After running this, you should see the first line of the
// sitemap (<?xml version ...), then either the first line
// of the sitemap or the first line of the robots file
// (User-agent: ...), and finally the first line of both
// the sitemap and the robots file.

#[tokio::main]
async fn main() {
    println!("SITEMAP:\n\n");

    // Get the sitemap, and print out
    // the first line to show that we
    // retrieved it.
    //
    // get_text already prints out an error
    // message, so if it fails, just return.
    let Some(sitemap) = get_text("https://cratecode.com/sitemap.xml").await else {
        return;
    };

    println!("{}", sitemap.lines().next().unwrap_or(""));

    println!("\n\nSELECT:\n\n");

    // Race the robots.txt and sitemap.xml, printing out
    // whichever one resolves first.
    // This will probably print the robots.txt since it's
    // smaller, but it's possible you'll see the sitemap here.
    tokio::select! {
        sitemap = get_text("https://cratecode.com/sitemap.xml") => {
            let Some(sitemap) = sitemap else {
                return;
            };

            println!("{}", sitemap.lines().next().unwrap_or(""));
        }
        robots = get_text("https://cratecode.com/robots.txt") => {
            let Some(robots) = robots else {
                return;
            };

            println!("{}", robots.lines().next().unwrap_or(""));
        }
    }

    println!("\n\nJOIN:\n\n");

    // Retrieve both the sitemap.xml and robots.txt, printing them both out
    // after both have been received.
    let (sitemap, robots) = tokio::join!(
        get_text("https://cratecode.com/sitemap.xml"),
        get_text("https://cratecode.com/robots.txt")
    );

    let Some(sitemap) = sitemap else {
        return;
    };
    let Some(robots) = robots else {
        return;
    };

    println!("{}", sitemap.lines().next().unwrap_or(""));
    println!("{}", robots.lines().next().unwrap_or(""));
}

/// Sends out an HTTP request and returns
/// the response as text,
/// returning None if it failed and
/// printing out to the console.
async fn get_text(url: &str) -> Option<String> {
    let req = match reqwest::get(url).await {
        Ok(req) => req,
        Err(err) => {
            eprintln!("An error occurred while sending the request: {err:?}");
            return None;
        }
    };

    let body = match req.text().await {
        Ok(body) => body,
        Err(err) => {
            eprintln!("An error occurred while reading the response: {err:?}");
            return None;
        }
    };

    Some(body)
}
