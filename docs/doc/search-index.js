var searchIndex = JSON.parse('{\
"actix":{"doc":"AlGlobo - Actor system to process flights","t":[0,0,0,0,5,0,3,12,11,11,11,11,11,11,11,11,11,3,3,3,12,11,11,11,11,11,11,11,11,11,5,11,11,12,12,11,11,11,12,12,11,11,11,11,11,11,11,11,11,11,11,11,3,12,11,11,11,11,11,11,11,11,11,3,12,11,11,11,11,12,11,11,12,12,11,11,11,11,11,17,3,3,11,11,11,11,11,12,12,12,12,12,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11],"n":["airline","airline_manager","hotel","info_flight","main","stats_actor","Airline","addr_statistics","borrow","borrow_mut","from","handle","into","try_from","try_into","type_id","vzip","AirlineManager","FinishRequest","NewRequest","addr_airline","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","from","from","from","from_file","handle","handle","info_flight","info_flight","into","into","into","pending_requests","requests_per_airline","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","vzip","vzip","vzip","Hotel","addr_statistics","borrow","borrow_mut","from","handle","into","try_from","try_into","type_id","vzip","InfoFlight","addr_manager","borrow","borrow_mut","clone","clone_into","flight_reservation","from","into","is_retry","start_time","to_owned","try_from","try_into","type_id","vzip","STATS_FREQUENCY","Stat","StatsActor","add_stat","borrow","borrow","borrow_mut","borrow_mut","destinations","elapsed_time","flight_reservation","flights","flights_to_process","from","from","get_avg_time","get_sum_time","get_top_destinations","get_total_count","handle","into","into","new","print_operational_stats","print_top_routes","sum_time","try_from","try_from","try_into","try_into","type_id","type_id","vzip","vzip"],"q":["actix","","","","","","actix::airline","","","","","","","","","","","actix::airline_manager","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","actix::hotel","","","","","","","","","","","actix::info_flight","","","","","","","","","","","","","","","","actix::stats_actor","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["Airline request actor","","Hotel request actor","Flight information message to be sent accross Hotel and …","Main function of the actor system","Actor that handles flight stats","","Ref to the stats actor","","","","Handle the message of InfoFlight and simulates to send it …","","","","","","","","","","","","","","","","","","","Create an Airline actor for each available airline in file","","","","","","","","","","","","","","","","","","","","","","","Ref to the stats actor","","","","Handle the message of InfoFlight and simulates to send it …","","","","","","Message made to a Actor Airline or Hotel to start the …","","","","","","The flight itself","","","","When the flight started to being processed by the program","","","","","","We print the stats every N flights proccesed","A request stat that can be either a hotel or an airline …","","Adds a flight to the stats entity","","","","","Every route and the number of flights taken so that we …","How much time has the request taken","Which flight this request belongs to","Every flight currently being run and their number of …","Number of flights to be processed","","","Returns the average response time","Returns the number of seconds spent processing","Returns the N top routes taken","Returns the number of flights taken overall","Request stat handler","","","","Prints the operational stats (every N flights)","Prints the top routes (every N flights)","Total number of seconds spent handling requests, to then …","","","","","","","",""],"i":[0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,0,0,0,2,2,3,4,2,3,4,2,3,4,0,2,2,3,4,2,3,4,2,2,2,3,4,2,3,4,2,3,4,2,3,4,0,5,5,5,5,5,5,5,5,5,5,0,6,6,6,6,6,6,6,6,6,6,6,6,6,6,6,0,0,0,7,7,8,7,8,7,8,8,7,7,7,8,7,7,7,7,7,7,8,7,7,7,7,7,8,7,8,7,8,7,8],"f":[null,null,null,null,[[]],null,null,null,[[]],[[]],[[]],[[["infoflight",3]]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]],null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[["str",15],["statsactor",3],["addr",3,["statsactor"]]],[["addr",3,["airlinemanager"]],["airlinemanager",3]]],[[["newrequest",3]]],[[["finishrequest",3]]],null,null,[[]],[[]],[[]],null,null,[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[]],[[]],[[]],null,null,[[]],[[]],[[]],[[["infoflight",3]]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]],null,null,[[]],[[]],[[]],[[]],null,[[]],[[]],null,null,[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]],null,null,null,[[["u128",15],["string",3]]],[[]],[[]],[[]],[[]],null,null,null,null,null,[[]],[[]],[[],["f64",15]],[[],["i64",15]],[[["usize",15]],["vec",3]],[[],["i64",15]],[[["stat",3]]],[[]],[[]],[[["usize",15]],["statsactor",3]],[[]],[[["usize",15]]],null,[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[]],[[]]],"p":[[3,"Airline"],[3,"AirlineManager"],[3,"NewRequest"],[3,"FinishRequest"],[3,"Hotel"],[3,"InfoFlight"],[3,"StatsActor"],[3,"Stat"]]},\
"common":{"doc":"Common constants, structures and functions between both …","t":[17,17,17,17,0,0,0,3,12,11,11,11,11,11,12,11,11,11,5,11,12,12,11,12,11,11,11,11,11,11,13,13,4,6,13,11,11,11,11,11,11,5,11,5,11,11,11,11,11,17,17,5,5,5],"n":["AIRLINES_FILE","MAX_TIME","MIN_TIME","TEST_FLIGHTS_FILE","flight_reservation","logger","utils","FlightReservation","airline","borrow","borrow_mut","clone","clone_into","deserialize","destination","fmt","fmt","from","from_file","get_route","hotel","id","into","origin","to_owned","to_string","try_from","try_into","type_id","vzip","FINISH","INFO","LogLevel","LoggerMsg","TRACE","borrow","borrow_mut","clone","clone_into","fmt","from","init","into","log","to_owned","try_from","try_into","type_id","vzip","DEFAULT_RETRY_SECONDS","SUCCESFUL_RATE","get_retry_seconds","read_file","toin_coss"],"q":["common","","","","","","","common::flight_reservation","","","","","","","","","","","","","","","","","","","","","","","common::logger","","","","","","","","","","","","","","","","","","","common::utils","","","",""],"d":["Airlines CSV config file","The maximum amount of seconds a simulated request takes","The minimum amount of seconds a simulated request takes","Flight CSV file to default if it wasn’t specified as CLA","Flight Reservations Struct","Logging functions","Helper Functions","Struct","Airline","","","","","","Destination airport","","","","Reads the CSV file with all the flights requests and …","Returns the flights route (origin + destination)","True if the flight is a package, false if it’s just a …","The id is set by the program, not the request","","Origin airport","","","","","","","FINISH logs are used to signalize that we want to stop …","INFO logs are printed to the console and are useful for …","Simple logging levels for our logger","A log consists of a message and their log level","TRACE logs refer to system initialization and nothing …","","","","","","","Creates or overwrites the log file and logs a START …","","Logs the message to the file and, if the level is INFO, …","","","","","","If the user doesn’t set the ENVVAR <code>RETRY_SECONDS</code> we …","The probability of a request to being successful","Returns either the ENV VAR value or the default one","Read CSV file and return split content under a Rust Vec","Throws a coin and returns a Result"],"i":[0,0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,0,1,1,1,1,1,1,1,1,1,1,1,2,2,0,0,2,2,2,2,2,2,2,0,2,0,2,2,2,2,2,0,0,0,0,0],"f":[null,null,null,null,null,null,null,null,null,[[]],[[]],[[]],[[]],[[],["result",4]],null,[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[["str",15]],[["flightreservation",3],["vec",3,["flightreservation"]]]],[[],["string",3]],null,null,[[]],null,[[]],[[],["string",3]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]],null,null,null,null,null,[[]],[[]],[[],["loglevel",4]],[[]],[[["formatter",3]],["result",6]],[[]],[[]],[[]],[[["loglevel",4],["string",3]]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]],null,null,[[],["u64",15]],[[["str",15]],[["box",3,["error"]],["vec",3,["vec"]],["result",4,["vec","box"]]]],[[],[["result",4,["str"]],["str",15]]]],"p":[[3,"FlightReservation"],[4,"LogLevel"]]},\
"informe":{"doc":"Informe","t":[5],"n":["main"],"q":["informe"],"d":[""],"i":[0],"f":[[[]]],"p":[]},\
"threads":{"doc":"AlGlobo - Simple HTTP server for reserving a flight","t":[3,0,12,0,11,11,11,11,11,11,11,11,0,12,5,11,3,0,12,11,11,11,11,11,11,11,11,6,5,5,5,5,5,17,17,5,3,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11],"n":["AppState","airlines","airlines","alglobo","borrow","borrow","borrow_mut","borrow_mut","from","from","into","into","keyboard","logger_sender","main","register","reservation","statistics","statistics","try_from","try_from","try_into","try_into","type_id","type_id","vzip","vzip","Airlines","from_file","reserve","send_to_airline","send_to_hotel","simulate_request","QUIT_COMMANDS","STAT_COMMANDS","keyboard_loop","Statistics","add_flight_reservation","borrow","borrow_mut","clone","clone_into","destinations","from","get_avg_time","get_sum_time","get_top_destinations","get_total_count","into","new","print_operational_stats","print_top_routes","sum_time","to_owned","try_from","try_into","type_id","vzip"],"q":["threads","","","","","","","","","","","","","","","","","","","","","","","","","","","threads::airlines","","threads::alglobo","","","","threads::keyboard","","","threads::statistics","","","","","","","","","","","","","","","","","","","","",""],"d":["This is the shared state that will be shared across every …","Handle airlines config file","Each airline supported by the app, with the number of …","Make the reservations","","","","","","","","","Keyboard loop waiting for commands","A reference to the logger, so that each thread can log …","The main function. It spawns different threads and starts …","","","Thread-safe flight statistics structure","The flights stats","","","","","","","","","Keep track of how many threads can each airline handle","Read from a CSV file with airlines and their max number …","We make a reservation by sending the request to the …","Function that makes the request to the airline","Function that makes the request to the hotel","","Possible command strings that trigger the graceful …","Possible command strings that trigger the show stats …","Listents to <code>s</code> (show stats) and <code>q</code> (quit) commands","Entity that holds the statistics of the flights","Adds request time to the accumulated sum, and the flights …","","","","","Every route and the number of flights taken so that we …","","Returns the avg flight process time","Returns the number of seconds spent handling requests","Returns the top N routes taken","Returns the number of total flights processed","","","Prints the operational stats","Prints the top N routes","Total number of seconds spent handling requests, to then …","","","","",""],"i":[0,0,1,0,1,2,1,2,1,2,1,2,0,1,0,2,0,0,1,1,2,1,2,1,2,1,2,0,0,0,0,0,0,0,0,0,0,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3],"f":[null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],null,null,[[]],[[["appservice",3]]],null,null,null,[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[]],[[]],null,[[["str",15]],[["string",3],["hashmap",3,["string","arc"]],["arc",3,["semaphore"]]]],[[["arc",3,["semaphore"]],["flightreservation",3],["statistics",3],["semaphore",3],["sender",3,["loggermsg"]],["loggermsg",6]]],[[["arc",3,["semaphore"]],["flightreservation",3],["loggermsg",6],["semaphore",3],["arc",3],["sender",3,["loggermsg"]]]],[[["flightreservation",3],["loggermsg",6],["arc",3],["sender",3,["loggermsg"]]]],[[]],null,null,[[["statistics",3],["sender",3]]],null,[[["instant",3],["string",3]],["i64",15]],[[]],[[]],[[]],[[]],null,[[]],[[],["f64",15]],[[],["i64",15]],[[["usize",15]],["vec",3]],[[],["i64",15]],[[]],[[],["statistics",3]],[[]],[[["usize",15]]],null,[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]]],"p":[[3,"AppState"],[3,"reservation"],[3,"Statistics"]]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};