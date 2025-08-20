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
- Note: make sure to update the `.env` file and use the correct password/other info in these commands
- Run Postgres
```
cd backend/src

docker pull postgres:17.6
# In Windows, use 5433 instead of 5432 because of issue
# This is assuming running Docker in WSL
docker run --name rical-db -e POSTGRES_PASSWORD=passwordhere -e POSTGRES_DB=rical_db -e POSTGRES_USER=userhere -e PGPORT=5433 -d -p 5433:5433 postgres

# To go into the db using psql:
docker exec -it rical-db sh
psql rical_db userhere
```
- If it was already running:
```
docker container stop rical-db
docker container rm rical-db
```
- Start the backend with `cargo run`. Assuming your `DATABASE_URL` is correct, schemas should be loaded into the database automatically via the build script.

## Deployment
- Using Railway, Postgres should be straightforward
    - <https://docs.railway.com/guides/postgresql>

## Etymology?
The acronym RICAL stands for:
- ***R***ical
- ***I***sn't a
- ***Cal***endar

Yes, it is in fact a date-based event/task management system, not a calendar.
