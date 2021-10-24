var searchIndex = JSON.parse('{\
"actix":{"doc":"","t":[6,6,0,0,0,5,5,0,3,12,11,11,11,5,11,11,11,11,11,11,3,12,11,11,11,11,11,11,11,11,11,3,11,11,11,11,12,11,11,12,11,11,11,11,11,3,17,3,3,11,11,11,11,11,11,11,11,12,12,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11],"n":["AirlineReq","HotelReq","airline","hotel","info_flight","main","reserve","stats_actor","Airline","addr_statistics","borrow","borrow_mut","from","from_file","handle","into","try_from","try_into","type_id","vzip","Hotel","addr_statistics","borrow","borrow_mut","from","handle","into","try_from","try_into","type_id","vzip","InfoFlight","borrow","borrow_mut","clone","clone_into","flight_reservation","from","into","start_time","to_owned","try_from","try_into","type_id","vzip","FinishMessage","STATS_FREQUENCY","Stat","StatsActor","add_stat","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","default","destinations","elapsed_time","flight_reservation","flights","from","from","from","get_avg_time","get_sum_time","get_top_destinations","get_total_count","handle","handle","into","into","into","new","print_operational_stats","print_top_routes","sum_time","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","vzip","vzip","vzip"],"q":["actix","","","","","","","","actix::airline","","","","","","","","","","","","actix::hotel","","","","","","","","","","","actix::info_flight","","","","","","","","","","","","","","actix::stats_actor","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["","","Handle airlines config","Handle airlines config","","","","","WebServer actor airline","","","","","Create a Airline Server for each available airline in …","Handle the message of InfoFlight and simulates to send it …","","","","","","","","","","","Handle the message of InfoFlight and simulates to send it …","","","","","","Message made to a Actor Airline or Hotel to start the …","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[0,0,0,0,0,0,0,0,0,1,1,1,1,0,1,1,1,1,1,1,0,2,2,2,2,2,2,2,2,2,2,0,3,3,3,3,3,3,3,3,3,3,3,3,3,0,0,0,0,4,4,5,6,4,5,6,4,4,6,6,4,4,5,6,4,4,4,4,4,4,4,5,6,4,4,4,4,4,5,6,4,5,6,4,5,6,4,5,6],"f":[null,null,null,null,null,[[]],[[["hashmap",3,["string","addr"]],["flightreservation",3],["addr",3,["hotel"]],["vec",3,["flightreservation"]],["hotel",3],["string",3],["addr",3,["airline"]]]],null,null,null,[[]],[[]],[[]],[[["str",15],["addr",3,["statsactor"]],["statsactor",3]],[["hashmap",3,["string","addr"]],["string",3],["addr",3,["airline"]]]],[[["infoflight",3]]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]],null,null,[[]],[[]],[[]],[[["infoflight",3]]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]],null,[[]],[[]],[[]],[[]],null,[[]],[[]],null,[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]],null,null,null,null,[[["u128",15],["string",3]]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],null,null,null,null,[[]],[[]],[[]],[[],["f64",15]],[[],["i64",15]],[[["usize",15]],["vec",3]],[[],["i64",15]],[[["finishmessage",3]]],[[["stat",3]]],[[]],[[]],[[]],[[],["statsactor",3]],[[]],[[["usize",15]]],null,[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[]],[[]],[[]]],"p":[[3,"Airline"],[3,"Hotel"],[3,"InfoFlight"],[3,"StatsActor"],[3,"FinishMessage"],[3,"Stat"]]},\
"common":{"doc":"","t":[17,17,0,0,0,0,3,12,11,11,11,11,11,12,11,11,11,5,11,12,12,11,12,11,11,11,11,11,11,13,13,4,6,13,11,11,11,11,11,11,5,11,5,11,11,11,11,11,17,17,17,5,5,17,5,5],"n":["AIRLINES_FILE","TEST_FLIGHTS_FILE","flight_reservation","logger","simulate_requests","utils","FlightReservation","airline","borrow","borrow_mut","clone","clone_into","deserialize","destination","fmt","fmt","from","from_file","get_route","hotel","id","into","origin","to_owned","to_string","try_from","try_into","type_id","vzip","FINISH","INFO","LogLevel","LoggerMsg","TRACE","borrow","borrow_mut","clone","clone_into","fmt","from","init","into","log","to_owned","try_from","try_into","type_id","vzip","MAX_TIME","MIN_TIME","RAND_SUCCESSFUL_REQUEST","simulate_airline","simulate_hotel","DEFAULT_RETRY_SECONDS","get_retry_seconds","read_file"],"q":["common","","","","","","common::flight_reservation","","","","","","","","","","","","","","","","","","","","","","","common::logger","","","","","","","","","","","","","","","","","","","common::simulate_requests","","","","","common::utils","",""],"d":["","","Flight Reservations Struct","","","Helper Functions","Struct","","","","","","","","","","","Read CSV file with all the flights requests and return a …","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Simulated request to a hypothetical airline web server","Simulated request to a hypothetical hotel web server","If the user doesn’t set the ENVVAR <code>RETRY_SECONDS</code> we …","","Read CSV file and return split content"],"i":[0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,0,1,1,1,1,1,1,1,1,1,1,1,2,2,0,0,2,2,2,2,2,2,2,0,2,0,2,2,2,2,2,0,0,0,0,0,0,0,0],"f":[null,null,null,null,null,null,null,null,[[]],[[]],[[]],[[]],[[],["result",4]],null,[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[["str",15]],[["flightreservation",3],["vec",3,["flightreservation"]]]],[[],["string",3]],null,null,[[]],null,[[]],[[],["string",3]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]],null,null,null,null,null,[[]],[[]],[[],["loglevel",4]],[[]],[[["formatter",3]],["result",6]],[[]],[[]],[[]],[[["loglevel",4],["string",3]]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]],null,null,null,[[],[["result",4,["str"]],["str",15]]],[[]],null,[[],["u64",15]],[[["str",15]],[["box",3,["error"]],["result",4,["vec","box"]],["vec",3,["vec"]]]]],"p":[[3,"FlightReservation"],[4,"LogLevel"]]},\
"informe":{"doc":"Informe","t":[5],"n":["main"],"q":["informe"],"d":[""],"i":[0],"f":[[[]]],"p":[]},\
"threads":{"doc":"AlGlobo - Simple HTTP server for reserving a flight","t":[3,0,12,0,11,11,11,11,11,11,11,11,0,12,5,11,3,0,12,11,11,11,11,11,11,11,11,6,5,5,5,5,17,17,5,3,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11],"n":["AppState","airlines","airlines","alglobo","borrow","borrow","borrow_mut","borrow_mut","from","from","into","into","keyboard","logger_sender","main","register","reservation","statistics","statistics","try_from","try_from","try_into","try_into","type_id","type_id","vzip","vzip","Airlines","from_file","reserve","send_to_airline","send_to_hotel","QUIT_COMMANDS","STAT_COMMANDS","keyboard_loop","Statistics","add_flight_reservation","borrow","borrow_mut","clone","clone_into","destinations","from","get_avg_time","get_sum_time","get_top_destinations","get_total_count","into","new","print_operational_stats","print_top_routes","sum_time","to_owned","try_from","try_into","type_id","vzip"],"q":["threads","","","","","","","","","","","","","","","","","","","","","","","","","","","threads::airlines","","threads::alglobo","","","threads::keyboard","","","threads::statistics","","","","","","","","","","","","","","","","","","","","",""],"d":["This is the shared state that will be shared across every …","Handle airlines config","","Make the reservations","","","","","","","","","","","The main function. It starts a thread for the keyboard …","","","Flight Stats","","","","","","","","","","Keep track of how many threads can each airline handle","Read from a CSV file with airlines and their max number …","We make a reservation by sending the request to the …","Function that makes the request to the airline","Function that makes the request to the hotel","Possible command strings that trigger the exit action","Possible command strings that trigger the show stats …","Listents to <code>s</code> (show stats) and <code>q</code> (quit) commands","Entity that holds the statistics of the flights","Adds request time to the accumulated sum, and the flights …","","","","","","","","","","","","","","","","","","","",""],"i":[0,0,1,0,1,2,1,2,1,2,1,2,0,1,0,2,0,0,1,1,2,1,2,1,2,1,2,0,0,0,0,0,0,0,0,0,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3],"f":[null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],null,null,[[]],[[["appservice",3]]],null,null,null,[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[]],[[]],null,[[["str",15]],[["hashmap",3,["string","arc"]],["string",3],["arc",3,["semaphore"]]]],[[["loggermsg",6],["sender",3,["loggermsg"]],["flightreservation",3],["arc",3,["semaphore"]],["statistics",3],["semaphore",3]]],[[["sender",3,["loggermsg"]],["arc",3],["flightreservation",3],["arc",3,["semaphore"]],["loggermsg",6],["semaphore",3]]],[[["arc",3],["flightreservation",3],["loggermsg",6],["sender",3,["loggermsg"]]]],null,null,[[["statistics",3],["sender",3]]],null,[[["instant",3],["string",3]],["i64",15]],[[]],[[]],[[]],[[]],null,[[]],[[],["f64",15]],[[],["i64",15]],[[["usize",15]],["vec",3]],[[],["i64",15]],[[]],[[],["statistics",3]],[[]],[[["usize",15]]],null,[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]]],"p":[[3,"AppState"],[3,"reservation"],[3,"Statistics"]]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};