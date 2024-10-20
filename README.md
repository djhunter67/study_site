# Purpose
Use my skills as a web developer to build a web app that\
ingests relevant data from my daughter's elementary school.\
She will use the ingested information to study for relevant\
quizzes and exams.
The leading technologies to help my 2nd grade daughter \
study her vocabulary words are costs that can be avoided. \

## Features
- [ ] Plan the project
  - [X] Architecture
  - [ ] Design
  - [ ] Implementation
  - [ ] Testing
- [X] Design the web app for use by a 2nd grader
- [ ] Implement the design using HTML and SCSS
- [ ] Manually enter vocabulary words and definitions
- [ ] Ingest data from the school's website
- [ ] Display the data in a user-friendly way
- [ ] Allow the user to study using virtual flashcards
- [ ] Allow the user to take a quiz
  - [ ] Multiple choice
  - [ ] True/False
- [ ] Allow the user to track their progress
- [ ] Track user progress over time
- [ ] User login and authentication
- [ ] Export data to a CSV file
- [ ] Export a report of user progress in PDF format
- [X] Free Free Free
- [ ] Roll my own authentication
- [X] Document database wired up to the app
- [X] Locally host to the world (education.hunter-homelab.com)

## Architecture
To keep any users with minimal computer or web application experience from being overwhelmed the bones of the application will be simple and to the point. There will be a landing page with three paths.  One of the three paths will be a login portal.  The other two paths will lead to identical layouts.

The useful destination from the home page will land on a page with three more paths.  On this page, all three paths lead to meaningful work. The meaningful work depends on the design.  As far as architecture is concerned, all meaningful work interfaces will be unique and situation dependent. Each possible path, to include meaningful work paths, will be limited to three more paths, at most. No logout scenario will be available, users can be perpetually logged in.

## Design
There will be an aim to keep buttons, cards, and shapes uniform and aligned throughout the user experience. Text that will be a link will be uppercase and no boxes will have sharp corners.  Colors will be soft and text will be inversely colored to its immediate background. 

### User Experience
Attention to few on screen distractions will be paid.  There are currently no plans to gamify, thus there will be no effort to implement frills, poppers, etc... 

## Design Ideology
Optimizing for future expansion and a simplistic is the primary concern while developing due to the fact that the intended user age group is seven years old (for now).  Inherent in the design age group is the number of intended users is one but design will support multiple users.  Thus, simple expansion to different, more, or complex subjects will be attainable with minimal time and effort.  Auth will be SSO, initially.  This is a golden opportunity to roll a custom auth to know if that is something to be avoided in the future.

## Technologies
### Frontend
- HTML
- SCSS
- HTMX

The technologies used in the frontend are the __best tools for the job__.  To be clear, the __best tool for the job__ is compeletely subjective.  The __best tool for the job__ is the tool that is most familiar to the developer.  It is believed that familiarization with tools upon which most other tools are built is critical to a well-rounded developer.  Thus, the __best tools for the job__ are HTML, SCSS, and HTMX.  Experience with Bulma, Tailwind, and Bootstrap are strongly opinionated libraries and, admittedly, would work well in this scenario but there is more to learn abut CSS and every project is an opportunity to grow.

### Backend
- Rust
- Actix-Web
- MongoDB
- Linux (Arch for development, Ubuntu for production)

Docuemnt databases inately different from relational databases.  Learning about the key differences in practice versus reading about the differences will allow for a more well-rounded developer.  Rust is a language that is not only fun to write but is also a language that is type safe and fast. 

