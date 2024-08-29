import requests
from pprint import pprint
import datetime
from dateutil import parser
import math


FORECAST_TIME=100;
SITE=5502 # To be changed to somethign from user.
SL_FETCH_URL='https://transport.integration.sl.se/v1/sites/'+str(SITE)+'/departures?forecast='+str(FORECAST_TIME)



def findInfoForLocation(location): 
    response = requests.get(SL_FETCH_URL)
    data=response.json()
    
    #Handle departures from ""
    for i in range(len(data['departures'])):
        times = data['departures'][i]['expected']
        yourdate=datetime.datetime.fromisoformat(times);
        #print(yourdate)
        timeNow= datetime.datetime.now()
        timediff=(yourdate-timeNow)
        minutesLeft=math.floor(timediff.seconds/60)
        direction=data['departures'][i]['direction']
        route=(data['departures'][i]['line']['designation'])
        
        if minutesLeft >3 and minutesLeft <100:
        
            print("Bus", route, " towards", direction, "in", minutesLeft," minutes")
        
    
def main():
    findInfoForLocation(1)
    
    
    
main()


'''
Todo :

1) Use SL data to find a route between A to B with least wait times outside in cold and shortest times.

'''