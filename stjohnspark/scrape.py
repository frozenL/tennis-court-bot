import requests
from datetime import datetime, timedelta
import time
from random import random, seed
from .selenium_send_request import send_message, get_driver
import click

prev_iter = None

def scrape(target):
    date_now = datetime.now().date()
    date_next_week = date_now + timedelta(days=7)
    resp = requests.get(f'https://clubspark.lta.org.uk/v0/VenueBooking/StJohnsParkLondon/GetVenueSessions?resourceID=&startDate={date_now}&endDate={date_next_week}&roleId=')
    resp = resp.json()
    res = resp['Resources']
    all_availables = set()
    for r in res:
        court = r['Name']
        days = r['Days']
        for d in days:
            available_date = datetime.strptime(d['Date'], '%Y-%m-%dT%H:%M:%S').date()
            for s in d['Sessions']:
                capacity = s['Capacity']
                if capacity <= 0:
                    continue
                cost = s['Cost']
                fromHour = int(s['StartTime'] / 60)
                toHour = int(s['EndTime'] / 60)
                if available_date.weekday() >= 4:
                    all_availables.add((court, available_date, fromHour, toHour, cost))
                elif toHour > 18:
                    all_availables.add((court, available_date, fromHour, toHour, cost))
    global prev_iter
    if prev_iter != all_availables:
        prev_iter=all_availables
        print(prev_iter)
        if len(prev_iter) > 0:
            send_message(get_driver(), target, "hey, new updates: \n" + '\n'.join(f"{d[2]}:00 to {d[3]}:00 on {d[1]}, {d[0]}, cost: {d[4]}" for d in prev_iter) + f"\nbook here: https://clubspark.lta.org.uk/StJohnsParkLondon/Booking/BookByDate#?date={date_now}&role=guest")
        else:
            send_message(get_driver(), target, "all slots unavailable :(")

@click.command()
@click.option('--target', required=True)
def main(target):
    seed(datetime.now().microsecond)
    while True:
        # if datetime.now().hour >= 7 and datetime.now().hour <= 24:
        scrape(target)
        sleep_time = 60*5 + int(random() * 10) * 60
        print(f"sleep for {sleep_time}s ({sleep_time/60}mins)")
        time.sleep(sleep_time)

if __name__ == '__main__':
    main()