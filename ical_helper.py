import os
from os import makedirs
from icalendar import Calendar, Event, vCalAddress, vText
from datetime import date, datetime, time, timedelta
from dateutil.rrule import rrule, MONTHLY, WEEKLY, DAILY

import pytz

# Generate valid calendars to use for testing
#
# Daily:
# 1.Generate 3 calendar events with an 8-hour difference between each other
# 2. Each event reoccurs daily
# Weekly:
# 1. Generate 4 calendar events for (Monday, Wed, Fri, Sun)
# 2. Each event reoccurs weekly
# Monthly:
# 1. Generate 4 calendar events every week (Monday)
# 2. Each event reoccurs monthly

ICAL_DIR        = 'test/ical'
DAILY_ICAL      = 'daily.ical'
WEEKLY_ICAL     = 'weekly.ical'
MONTHLY_ICAL    = 'monthly.ical'

# Get the first day of the month given the current date
def first_day(dt):
    return dt.replace(day=1)

# Get the day of the first monday
# We will define this as the first week of the month
def first_monday(dt: datetime):
    dt.replace(day = 7)
    offset = -dt.weekday() # weekday = 0 means monday
    return dt + timedelta(offset)

def first_wednesday(dt: datetime):
    return first_monday(dt) + timedelta(2*1)

def first_friday(dt: datetime):
    return first_monday(dt) + timedelta(2*2)

def first_sunday(dt: datetime):
    return first_monday(dt) + timedelta(2*3)

def forward_year_later(time: datetime):
    time.replace(year = time.year + 1)
    return time

# Daily:
# We will have the events occur at 8:00, 16:00, and 24:00 (00:00)
# We start 3 daily events
DAILY_1 = dict(hour=0, minute=0, second=0)
DAILY_2 = dict(hour=8, minute=0, second=0)
DAILY_3 = dict(hour=16, minute=0, second=0)

DAILYS = [ DAILY_1, DAILY_2, DAILY_3 ]

# Weekly:
# We will have the events occur on Monday, Wed, Fri, Sun
# We start 4 weekly events
WEEKLY_1 = first_monday
WEEKLY_2 = first_wednesday
WEEKLY_3 = first_friday
WEEKLY_4 = first_sunday

WEEKLIES = [WEEKLY_1, WEEKLY_2, WEEKLY_3, WEEKLY_4, ]

#
# RRULES
#
# Define the recurring events

# Reoccur every day
def reoccurs_daily(dtstart: datetime):
    return rrule(freq=DAILY, interval=1,dtstart=dtstart)

# Reoccur every week
def reoccurs_weekly(dtstart: datetime):
    return rrule(freq=WEEKLY, interval=1, wkst=0, dtstart=dtstart) # Start on monday

# Reoccur every month
def reoccurs_monthly(dtstart: datetime):
    return rrule(freq=MONTHLY, interval=1, bymonth=1, dtstart=dtstart)

#
# Calendar
#

def prodid(org, product, locale):
    return f'-//{org}//{product}//{locale}'

def with_prodid(cal):
    cal.add('prodid', prodid("jmdaemon", "ical_helper", "EN"))
    cal.add('version', '2.0')

#
# Calendar Events
#
def with_organizer(event: Event):
    organizer = vCalAddress('MAILTO:organizer@example.com')
    organizer.params['cn'] = vText('Example Organizer')
    organizer.params['role'] = vText('CHAIR')
    event['organizer'] = organizer
    event['location'] = vText('British Columbia, Vancouver')
    event.add('priority', 5)

def create_event(summary: str, dtstart: datetime, dtend: datetime, dtstamp: datetime):
    event = Event()
    event.add('summary', summary)
    event.add('dtstart', dtstart)
    event.add('dtend', dtend)
    event.add('dtstamp', dtstamp)
    return event

def create_event_date_times(start: datetime, end: datetime):
    return (start, end, start)

def create_year_later_daily(time: dict):
    fst_mon = first_monday(first_day(datetime.today()))
    year_later = forward_year_later(first_monday(first_day(datetime.today())))
    dt = fst_mon.replace(**time)
    return (dt, year_later)

def create_event_summary(summary, time):
    (start, end) = create_year_later_daily(time)
    return create_event(summary, *create_event_date_times(start, end))

def create_daily_event(number: int, time):
    return create_event_summary(f'Example daily event #{number}', time)

# def create_weekly_event(time):
    # return create_event_summary('Example weekly event', time)
def create_year_later_weekly(day_fn, now):
    time = day_fn(now)
    year_later = forward_year_later(time)
    # dt = time.replace(**time)
    return (time, year_later)

def create_weekly_event_summary(summary, time):
    (start, end) = create_year_later_weekly(time, datetime.today())
    return create_event(summary, *create_event_date_times(start, end))

def create_weekly_summary(number: int, time):
    return create_weekly_event_summary(f'Example weekly event #{number}', time)

def create_monthly_event(time):
    return create_event_summary('Example monthly event', time)

#
# Main Entrypoint
#

def create_daily_calendar(file):
    cal = Calendar()
    with_prodid(cal)

    # Create dailies
    index = 1
    for time in DAILYS:
        event = create_daily_event(index, time)
        with_organizer(event)
        cal.add_component(event)
        index += 1

    with open(file, mode='w') as f:
        f.write(cal.to_ical().decode("utf-8"))

def create_weekly_calendar(file):
    cal = Calendar()
    with_prodid(cal)

    # Create dailies
    index = 1
    for day_fn in WEEKLIES:
        # time = day_fn(datetime.today())
        # year_later = day_fn(datetime.today())
        # year_later.replace(year = year_later.year + 1)

        # event = create_weekly_event_summary(index, time)
        # event = create_weekly_event_summary(index, day_fn)
        event = create_weekly_summary(index, day_fn)
        with_organizer(event)
        cal.add_component(event)
        index += 1

    if not os.path.exists(file):
        with open(file, mode='w') as f:
            f.write(cal.to_ical().decode("utf-8"))
    else:
        print(f'Calendar already written to {file}')

def main():
    makedirs(ICAL_DIR, exist_ok = True)
    create_daily_calendar(f'{ICAL_DIR}/{DAILY_ICAL}')
    create_weekly_calendar(f'{ICAL_DIR}/{WEEKLY_ICAL}')
main()


# def main():
    # fst_mon = first_monday(first_day(datetime.today()))
    # time_1 = fst_mon.replace(**DAILY_1)
    # time_2 = fst_mon.replace(**DAILY_2)
    # time_3 = fst_mon.replace(**DAILY_3)

    # print(reoccurs_daily(time_1))
    # print(reoccurs_daily(time_2))
    # print(reoccurs_daily(time_3))

# for time in DAILYS:
    # print(create_daily_event(time))
