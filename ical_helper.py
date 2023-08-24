from os import makedirs
from os.path import dirname
from icalendar import Calendar, Event, vCalAddress, vText
from datetime import datetime, timedelta
from dateutil.rrule import rrule, DAILY, WEEKLY, MONTHLY

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

#
# Files
#
# The calendars will be written to these separate files

ICAL_DIR        = 'test/ical'
DAILY_ICAL      = f'{ICAL_DIR}/daily.ics'
WEEKLY_ICAL     = f'{ICAL_DIR}/weekly.ics'
MONTHLY_ICAL    = f'{ICAL_DIR}/monthly.ics'

#
# Initial Datetime Offsets
#

# Daily
def first_day():
    return datetime.today().replace(day=1)

# Weekly
def first_monday(fst_day: datetime):
    fst_day.replace(day = 7)
    offset = -fst_day.weekday() # weekday = 0 means monday
    return fst_day + timedelta(offset)

def first_weekday(fst_day: datetime, weekday):
    return first_monday(fst_day) + timedelta(weekday)

def first_wednesday(fst_day: datetime): return first_weekday(fst_day, 2*1)
def first_friday(fst_day: datetime): return first_weekday(fst_day, 2*2)
def first_sunday(fst_day: datetime): return first_weekday(fst_day, 2*3)

# Monthly

# Calculate the next day given an offset
def days_of(fst_day: datetime, offset):
    return fst_day + timedelta(offset)

# Find the next occurence of a day
def n_day_of(fst_day: datetime, n: int):
    return days_of(fst_day, 7 * n)

#
# Offsets
#
# The offsets for all the starting events

fst = first_day()

offsets_events_daily = [
    first_monday(fst).replace(hour=0, minute=0, second=0),
    first_monday(fst).replace(hour=8, minute=0, second=0),
    first_monday(fst).replace(hour=16, minute=0, second=0),
]

offsets_events_weekly = [
    first_monday(fst).replace(hour=8, minute=0, second=0),
    first_wednesday(fst).replace(hour=8, minute=0, second=0),
    first_friday(fst).replace(hour=8, minute=0, second=0),
    first_sunday(fst).replace(hour=8, minute=0, second=0),
]

offsets_events_monthly = [
    n_day_of(fst, 0).replace(hour=8, minute=0, second=0),
    n_day_of(fst, 1).replace(hour=8, minute=0, second=0),
    n_day_of(fst, 2).replace(hour=8, minute=0, second=0),
    n_day_of(fst, 3).replace(hour=8, minute=0, second=0)
]

# Advance forward a year later
def advance_year_later(dt: datetime):
    dt.replace(year = dt.year + 1)
    return dt
#
# RRULES
#
# Define the recurring events

# Reoccur every day
def reoccurs_daily(dtstart: datetime):
    # return rrule(freq=DAILY, interval=1,dtstart=dtstart)
    return rrule(freq=DAILY, interval=1)

# Reoccur every week
def reoccurs_weekly(dtstart: datetime):
    return rrule(freq=WEEKLY, interval=1, wkst=0) # Start on monday

# Reoccur every month
def reoccurs_monthly(dtstart: datetime):
    return rrule(freq=MONTHLY, interval=1, bymonth=1)

#
# Calendar
#

def create_calendar():
    cal = Calendar()
    with_prodid(cal)
    return cal

def prodid(org, product, locale):
    return f'-//{org}//{product}//{locale}'

def with_prodid(cal):
    cal.add('prodid', prodid("jmdaemon", "ical_helper", "EN"))
    cal.add('version', '2.0')

#
# iCalendar Events
#
def with_organizer(event: Event):
    organizer = vCalAddress('MAILTO:organizer@example.com')
    organizer.params['cn'] = vText('Example Organizer')
    organizer.params['role'] = vText('CHAIR')
    event['organizer'] = organizer
    event['location'] = vText('British Columbia, Vancouver')
    event.add('priority', 5)

def with_rrule(event: Event, reoccur: rrule):
    # Format string for serialization
    s = str(reoccur) \
        .split('\n') \
        [1] \
        .replace('RRULE:', '')
    event['rrule'] = s

def create_event(summary: str, dtstart: datetime, dtend: datetime, dtstamp: datetime):
    event = Event()
    event.add('summary', summary)
    event.add('dtstart', dtstart)
    event.add('dtend', dtend)
    event.add('dtstamp', dtstamp)
    return event

#
# Calendar Events
#

# Generic Functions
def format_summary_msg(msg: str, number: int):
    return msg.format(number)

def create_calendar_event(msg: str, beg: datetime):
    beg = beg
    end = advance_year_later(beg)
    return create_event(msg, beg, end, beg)

# Specific Events

def create_daily_event(number: int, beg: datetime):
    msg = format_summary_msg('Example daily event #{}', number)
    return create_calendar_event(msg, beg)

def create_weekly_event(number: int, beg: datetime):
    msg = format_summary_msg('Example weekly event #{}', number)
    return create_calendar_event(msg, beg)

def create_monthly_event(number: int, beg: datetime):
    msg = format_summary_msg('Example monthly event #{}', number)
    return create_calendar_event(msg, beg)

#
# Main Entrypoint
#

def write_cal_to(file, cal):
    makedirs(dirname(file), exist_ok = True)
    with open(file, mode='w') as f:
        f.write(cal.to_ical().decode("utf-8"))

# Takes a higher order function to create a calendar
def create_calendar_fn(create_fn, offsets, rrule_fn):
    cal = create_calendar()

    # Populate events
    index = 1
    for offset in offsets:
        event = create_fn(index, offset)
        with_organizer(event)
        with_rrule(event, rrule_fn(offset))
        cal.add_component(event)
        index += 1
    return cal

def create_calendar_daily(): return create_calendar_fn(create_daily_event, offsets_events_daily, reoccurs_daily)
def create_calendar_weekly(): return create_calendar_fn(create_weekly_event, offsets_events_weekly, reoccurs_weekly)
def create_calendar_monthly(): return create_calendar_fn(create_monthly_event, offsets_events_monthly, reoccurs_monthly)

def main():
    write_cal_to(DAILY_ICAL, create_calendar_daily())
    write_cal_to(WEEKLY_ICAL, create_calendar_weekly())
    write_cal_to(MONTHLY_ICAL, create_calendar_monthly())
main()
