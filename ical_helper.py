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
def first_monday(year, month, day):
    # d = datetime(year, int(month), 7)
    # dt.day = 7
    dt = datetime(year, int(month), 7)
    offset = -dt.weekday() # weekday = 0 means monday
    return dt + timedelta(offset)
         # return (date_value.isocalendar()[1] - date_value.replace(day=1).isocalendar()[1] + 1)
    # dt.replace(day=1)
    # dt.weekday()

# Daily:
# We will have the events occur at 8:00, 16:00, and 24:00 (00:00)
# TIME_1 = time(hour=0, minute=0, second=0)
# TIME_2 = time(hour=8, minute=0, second=0)
# TIME_3 = time(hour=16, minute=0, second=0)
TIME_1 = dict(hour=0, minute=0, second=0)
TIME_2 = dict(hour=8, minute=0, second=0)
TIME_3 = dict(hour=16, minute=0, second=0)


# Define the recurring events
def reoccurs_daily(dtstart: datetime):
    return rrule(freq=DAILY, count=4, dtstart=dtstart)

def prodid(org, product, locale):
    return f'-//{org}//{product}//{locale}'

def make_daily():
    today = datetime.today()





# def create_event(time_frame):
    # event_time_frame = datetime.today()
    # msg = ''
    # match time_frame:
        # case 'daily': pass
        # case 'weekly': event_time_frame += timedelta(weeks = 7)
        # case 'monthly': 

    # event = Event()

# def create_event(time_frame):

# cal = Calendar()

def create_calendar(cal):
    cal.add('prodid', prodid("jmdaemon", "ical_helper", "EN"))
    cal.add('version', '2.0')

def main():
    # time(hour = , 
    today = datetime.today()
    # dt = datetime(time=TIME_1)
    dt = datetime.today().replace(**TIME_1)
    # dt.time=
    # print(list(reoccurs_daily(datetime.today())))
    print(list(reoccurs_daily(dt)))
main()
