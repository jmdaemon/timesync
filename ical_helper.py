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

#
# Files
#
# The calendars will be written to these separate files

ICAL_DIR        = 'test/ical'
DAILY_ICAL      = f'{ICAL_DIR}/daily.ical'
WEEKLY_ICAL     = f'{ICAL_DIR}/weekly.ical'
MONTHLY_ICAL    = f'{ICAL_DIR}/monthly.ical'

#
# Initial Datetime Offsets
#
# For the first of all our events, we will construct our calendar to begin
#   from the first monday of every month
# Daily     : Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday
# Weekly    : Monday, Wednesday, Friday, Sunday
# Monthly   : Monday

# Algorithm:
# 1. let fst = fst_day()
# 2. let (fst_mon, fst_wed, fst_fri, fst_sun) = (fst_mon(fst), fst_wed(fst), fst_fri(fst), fst_sun(fst))
# 3. let daily_times = [
#       dict(hour=0, minute=0, second=0),
#       dict(hour=8, minute=0, second=0)
#       dict(hour=16, minute=0, second=0)
# ]
# 4. let daily_days = [ 
# ]

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

# Find every occurence of a day within a month
# def all_days(fst_day: datetime):

# Find the next occurence of a day
def n_day_of(fst_day: datetime, n: int):
    return days_of(fst_day, 7 * n)


#
# Offsets
#
# The offsets for all the starting events

fst = first_day()

offsets_events_daily = [
    # fst.replace(hour=0, minute=0, second=0),
    # fst.replace(hour=8, minute=0, second=0),
    # fst.replace(hour=16, minute=0, second=0),
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

# def fst_mon(fst: datetime):
    # fst.replace(day = 7)
    # offset = -fst.weekday() # weekday = 0 means monday
    # return fst + timedelta(offset)

# def fst_wed(fst: datetime): return first_monday(fst) + timedelta(2*1)
# def fst_fri(fst: datetime): return first_monday(fst) + timedelta(2*2)
# def fst_sun(fst: datetime): return first_monday(fst) + timedelta(2*3)





# Get the first day of the month given the current date
# def first_day(dt):
    # return dt.replace(day=1)

# # Get the day of the first monday
# # We will define this as the first week of the month
# def first_monday(dt: datetime):
    # dt.replace(day = 7)
    # offset = -dt.weekday() # weekday = 0 means monday
    # return dt + timedelta(offset)

# def first_wednesday(dt: datetime):
    # return first_monday(dt) + timedelta(2*1)

# def first_friday(dt: datetime):
    # return first_monday(dt) + timedelta(2*2)

# def first_sunday(dt: datetime):
    # return first_monday(dt) + timedelta(2*3)

# Advance forward a year later
def advance_year_later(dt: datetime):
    dt.replace(year = dt.year + 1)
    return dt

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

# def create_event_date_times(start: datetime, end: datetime):
    # return (start, end, start)

# def create_year_later_daily(time: dict):
    # fst_mon = first_monday(first_day(datetime.today()))
    # year_later = advance_year_later(first_monday(first_day(datetime.today())))
    # dt = fst_mon.replace(**time)
    # return (dt, year_later)

# def create_event_summary(summary, time):
    # (start, end) = create_year_later_daily(time)
    # return create_event(summary, *create_event_date_times(start, end))

# def create_daily_event(number: int, time):
    # return create_event_summary(f'Example daily event #{number}', time)

# # def create_weekly_event(time):
    # # return create_event_summary('Example weekly event', time)
# def create_year_later_weekly(day_fn, now):
    # time = day_fn(now)
    # year_later = advance_year_later(time)
    # # dt = time.replace(**time)
    # return (time, year_later)

# def create_weekly_event_summary(summary, time):
    # (start, end) = create_year_later_weekly(time, datetime.today())
    # return create_event(summary, *create_event_date_times(start, end))

# def create_weekly_summary(number: int, time):
    # return create_weekly_event_summary(f'Example weekly event #{number}', time)

# def create_monthly_event(time):
    # return create_event_summary('Example monthly event', time)

#
# Main Entrypoint
#

def write_cal_to(file, cal):
    makedirs(os.path.dirname(file), exist_ok = True)
    with open(file, mode='w') as f:
        f.write(cal.to_ical().decode("utf-8"))


# Takes a higher order function to create a calendar
def create_calendar_fn(fn):
    cal = create_calendar()

    # Populate events
    index = 1
    for offset in offsets_events_daily:
        event = fn(index, offset)
        with_organizer(event)
        cal.add_component(event)
        index += 1
    return cal

def create_calendar_daily(): return create_calendar_fn(create_daily_event)
def create_calendar_weekly(): return create_calendar_fn(create_weekly_event)
def create_calendar_monthly(): return create_calendar_fn(create_monthly_event)

# def create_daily_calendar(file):
    # cal = Calendar()
    # with_prodid(cal)

    # # Create dailies
    # index = 1
    # for time in DAILYS:
        # event = create_daily_event(index, time)
        # with_organizer(event)
        # cal.add_component(event)
        # index += 1

    # with open(file, mode='w') as f:
        # f.write(cal.to_ical().decode("utf-8"))

# def create_weekly_calendar(file):
    # cal = Calendar()
    # with_prodid(cal)

    # # Create dailies
    # index = 1
    # for day_fn in WEEKLIES:
        # # time = day_fn(datetime.today())
        # # year_later = day_fn(datetime.today())
        # # year_later.replace(year = year_later.year + 1)

        # # event = create_weekly_event_summary(index, time)
        # # event = create_weekly_event_summary(index, day_fn)
        # event = create_weekly_summary(index, day_fn)
        # with_organizer(event)
        # cal.add_component(event)
        # index += 1

    # if not os.path.exists(file):
        # with open(file, mode='w') as f:
            # f.write(cal.to_ical().decode("utf-8"))
    # else:
        # print(f'Calendar already written to {file}')

def main():
    write_cal_to(DAILY_ICAL, create_calendar_daily())
    write_cal_to(WEEKLY_ICAL, create_calendar_weekly())
    write_cal_to(MONTHLY_ICAL, create_calendar_monthly())

    # makedirs(ICAL_DIR, exist_ok = True)
    # create_daily_calendar(f'{ICAL_DIR}/{DAILY_ICAL}')
    # create_weekly_calendar(f'{ICAL_DIR}/{WEEKLY_ICAL}')
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
