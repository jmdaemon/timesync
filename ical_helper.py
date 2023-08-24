from icalendar import Calendar, Event
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

DAILY_ICAL      = 'daily.ical'
WEEKLY_ICAL     = 'weekly.ical'
MONTHLY_ICAL    = 'monthly.ical'

# Get the first day of the month given the current date
def first_day(dt):
    return dt.replace(day=1)

# Get the day of the first monday
# We will define this as the first week of the month
# def first_monday(year, month, day):
def first_monday(dt: datetime):
    # d = datetime(year, int(month), 7)
    # dt.day = 7
    # dt = datetime(year, int(month), 7)
    dt.replace(day = 7)
    offset = -dt.weekday() # weekday = 0 means monday
    return dt + timedelta(offset)
         # return (date_value.isocalendar()[1] - date_value.replace(day=1).isocalendar()[1] + 1)
    # dt.replace(day=1)
    # dt.weekday()

def forward_year_later(time: datetime):
    time.replace(year = time.year + 1)
    return time

# Daily:
# We will have the events occur at 8:00, 16:00, and 24:00 (00:00)
TIME_1 = dict(hour=0, minute=0, second=0)
TIME_2 = dict(hour=8, minute=0, second=0)
TIME_3 = dict(hour=16, minute=0, second=0)

TIMES = [ TIME_1, TIME_2, TIME_3 ]

# Define the recurring events

# Reoccur every day
def reoccurs_daily(dtstart: datetime):
    return rrule(freq=DAILY, dtstart=dtstart)

# Reoccur every week
def reoccurs_weekly(dtstart: datetime):
    return rrule(freq=WEEKLY, dtstart=dtstart)

# Reoccur every month
def reoccurs_monthly(dtstart: datetime):
    return rrule(freq=MONTHLY, dtstart=dtstart)

# Calendar

def prodid(org, product, locale):
    return f'-//{org}//{product}//{locale}'

def create_calendar(cal):
    cal.add('prodid', prodid("jmdaemon", "ical_helper", "EN"))
    cal.add('version', '2.0')

## Calendar Events

def create_event(summary: str, dtstart: datetime, dtend: datetime, dtstamp: datetime):
    event = Event()
    event.add('summary', summary)
    event.add('dtstart', dtstart)
    event.add('dtend', dtend)
    event.add('dtstamp', dtstamp)
    return event

def create_event_date_times(start: datetime, end: datetime):
    return (start, end, start)

def create_year_later(time: dict):
    fst_mon = first_monday(first_day(datetime.today()))
    year_later = forward_year_later(first_monday(first_day(datetime.today())))
    dt = fst_mon.replace(**time)
    return (dt, year_later)

def create_event_summary(summary, time):
    (start, end) = create_year_later(time)
    return create_event(summary, *create_event_date_times(start, end))

def create_daily_event(time):
    return create_event_summary('Example daily event', time)

def create_weekly_event(time):
    return create_event_summary('Example weekly event', time)

def create_monthly_event(time):
    return create_event_summary('Example monthly event', time)


# def create_weekly_event(start: datetime, end: datetime):
    # return create_event('Example weekly event', start, end, start)

# def create_weekly_event(start: datetime, end: datetime):
    # return create_event('Example weekly event', start, end, start)



# def create_daily_event_year_later(time: dict):
    # print(create_daily_event(*create_year_later(time)))

# def create_weekly_event_year_later(time: dict):
    # print(create_weekly_event(*create_year_later(time)))


def main():
    fst_mon = first_monday(first_day(datetime.today()))
    time_1 = fst_mon.replace(**TIME_1)
    time_2 = fst_mon.replace(**TIME_2)
    time_3 = fst_mon.replace(**TIME_3)

    print(reoccurs_daily(time_1))
    print(reoccurs_daily(time_2))
    print(reoccurs_daily(time_3))

for time in TIMES:
    print(create_daily_event(time))
