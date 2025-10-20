# Rical

The latest calendar app for minimalists.

("screenshot" coming soon, it'll just be copy pasted from the terminal)

This app is far from complete! Check back soon, mark your calendars...

## Core Objectives
- Organize events or tasks at certain dates/times
- Offer (ideally cross-platform) system notifications for the events of each day
- Store calendars in a full-scale database and allow multiple accounts, for scalability and cross-device usage
- Offer a lightweight, intuitive TUI frontend with efficient keybinds

## Architecture requirements
**Rical Backend**
- Basic authentication (log in with username and password to get JWT)
- Store users and their private calendars
- Access and write calendars via a friendly and simple API

**Rical Frontend UI**
- Display an intuitive calendar TUI, much like existing calendar GUI apps
- Each day should show tasks and users can complete them, view details, navigate across the calendar, etc.
- Allow the user to configure the backend URL and authenticate with the backend, storing their auth tokens persistently to avoid constant signouts
- Allow the user to complete tasks and add tasks

**Drical (the Daemon for Rical)**
- Offer system notifications once a day for the calendar tasks that will appear that day
- Work cross-platform; constantly run in the background, but be very lightweight
- If the user has not opened their computer/ran Drical for multiple days, show all the tasks that have accumulated over the unviewed days

**Deployment**
- Create docs for self-hosting (make it easy to self-host, possibly even just running an instance completely locally)
- Deploy the backend
- Possibly make a simple frontend website for easy mobile access across devices

## Using the Rical Frontend
### Starting out
Run the frontend (see development below) and make sure you're connected to the server.

To create an account, follow the instructions in the main menu.

To exit the app at any time, press `Ctrl+C`.

### Controls (calendar: month pane)
- `h/j/k/l`: navigate between dates
- `o`: "Open" a new task
- `Enter`: "Enter" into the tasks pane from the month pane
- `Ctrl+M`: log out to the "Menu"

### Controls (calendar: tasks pane)
- `Esc`: "Escape" out of the tasks pane into the month pane
- `j/k`: navigate down/up between dates and tasks
- `o`: "Open" a new task
- `e`: "Edit" a currently selected task
- `D` (`Shift`+`d`): "Delete" a task (cut it to your Rical clipboard)
- `p`: "Paste" a task from your rical clipboard into the currently selected date
- `x`: mark a task as done or not done (toggle)
- `Ctrl+M`: log out to the "Menu"

### Controls (input boxes/forms)
These should be what you're familiar with:
- `Backspace`: delete last character
- `Ctrl+Backspace`: delete last word
- `Left`: go to previous character
- `Home`: go to start of input
- `End`: go to end of input
- `Up` or `Shift`+`Tab`: go to previous input box
- `Down` or `Tab`: go to next input box
- `Enter`: submit a form

## Technologies
**Backend**
- Rust 🦀
- PostgreSQL
- Axum

**Frontend**
- Rust 🦀
- Crossterm TUI library (Ratatui is too high-level for this project; we require very specific details)

**Drical**
- Rust 🦀
- Libraries and technologies to be determined

## Development
**Starting the backend**
- Note: make sure to update the `.env` file and use the correct DB password/other info in these commands
- Run Postgres
```sh
cd backend/src

docker pull postgres:17.6
# In Windows, use 5433 instead of 5432 because of issue
# This is assuming running Docker in WSL
docker run --name rical-db -e POSTGRES_PASSWORD=passwordhere -e POSTGRES_DB=rical_db -e POSTGRES_USER=userhere -e PGPORT=5433 -d -p 5433:5433 postgres

# To go into the db using psql:
docker exec -it rical-db sh
psql rical_db userhere
```
- To start it if you've already ran it before:
```sh
docker container start rical-db
```
- If it was already running:
```sh
docker container stop rical-db
docker container rm rical-db
```
- Start the backend with `cargo run`. Assuming your `DATABASE_URL` is correct, schemas should be loaded into the database automatically via the build script.
- If you want to use the dockerfile, include the args
```sh
docker build . --tag 'rical_backend_test' --build-arg DATABASE_URL=yoururlhere --build-arg JWT_SECRET=yoursecrethere --build-arg PORT=3001
```

**Starting the frontend**
1. Clone the repository
2. Copy `rical_frontend/.env.example` into `rical_frontend/.env` and set the `API_URL` variable properly.
    - If you're planning on using the officially hosted backend, set it to that URL (coming soon)
    - If you're self-hosting, put the URL that you're running the backend on
3. Run or install by running these commands:
```sh
cd rical_frontend

# To just run the program once
cargo run

# OR, to install to your computer
cargo install --path .
```

## Deployment
- Using Railway, Postgres should be straightforward
    - <https://docs.railway.com/guides/postgresql>

## Etymology?
The acronym RICAL stands for:
- ***R***ical
- ***I***sn't a
- ***Cal***endar

Yes, it is in fact a date-based event/task management system, not a calendar.
