# Plan for citeman

 1. Data models
 2. Database / utility cli
 3. User interface (web to start with)
 4. Native GUI
 5. Ability to auto-download metadata (lookup sources etc.)
 5. Deployment - should come as an installer

# Data models

The main data is a collection of documents. These have authors. It would be good if we could
search by author. Don't know how to handle distinct authors with the same name.

Documents can be

 - Journal articles
 - Books
 - Preprints/unpublished
 - Tech reports/white papers
 - Booklet
 - theses
 - Proceedings from conference etc.
 - general catch-all for documents that don't fit in this list

To start with focus on journal articles

# Database

Description of data models should inform database structure. Use sqlite. Use standard file location
in whatever OS we are on. Journal articles go in separate folder, with uuid title corresponding to
metadata record in db.


# Web UI

I would like the server to provide a message passing interface. This can then be used over
a websocket for web, but the same framework can be used for a local native app.
