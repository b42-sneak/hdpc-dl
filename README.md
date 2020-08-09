# hdpc-dl

My humble software

## Goals

1. Individual comic download by URL
   1. Crawl the URL given via the CLI arguments
   2. Extract all available metadata
      1. Name
      1. Tags
      1. Author
      1. Category
      1. Likes
      1. Images
      1. Views
      1. Upload date
      1. Download date (=now)
   3. Create a new directory
   4. Write the metadata as JSON into the directory
   5. Extract all image URLs
   6. Download all images from a given comic into the directory
2. Download all on a page
   1. Download the HTML via URL
   2. Crawl the URL given via the CLI arguments
   3. Extract all comic-URLs
   4. Run 1) on all of them
