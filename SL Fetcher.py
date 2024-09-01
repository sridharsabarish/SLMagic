import requests
from pprint import pprint
import datetime
from dateutil import parser
import math
import Constants



FORECAST_TIME=100;
SITE=5502 # To be changed to somethign from user.
SL_FETCH_URL='https://transport.integration.sl.se/v1/sites/'+str(SITE)+'/departures?forecast='+str(FORECAST_TIME)
EXAMPLE_PLANNER_URL='https://journeyplanner.integration.sl.se/v1/TravelplannerV3_1/journeydetail.&lt;FORMAT&gt;?key=&lt;'+str(Constants.APIKEY)+'&gt;&amp;id=1|3598|0|74|13062017'
STOP_SEARCH_URL='https://journeyplanner.integration.sl.se/v1/typeahead.json?searchstring=Edsberg&stationsonly=true&maxresults=5&key='+Constants.APIKEY

def doAPICall(URL): 
    response = requests.get(URL)
    print(response.text)
   

def findInfoForLocation(location): 
    data=doAPICall(SL_FETCH_URL)
    
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
        
    
def findConnectionFromLocation(location):
    doAPICall(STOP_SEARCH_URL)
def main():
    #findInfoForLocation(1)
    findConnectionFromLocation(1)
main()


'''
Todo :

1) Use SL data to find a route between A to B with least wait times outside in cold and shortest times.

'''


    


