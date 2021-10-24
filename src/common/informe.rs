//! Informe
//! ---
//! [Leer en PDF](http://...)
//!
//! [Leer en HTML](http://...)
//! 
//! # Parte A
//! La resolución de la primera parte del Trabajo Práctico se encuentra en la carpeta `src/threads` y se podrá ejecutar con `cargo run --bin threads`.
//! 
//! ## Enunciado
//! *Implementar la aplicación utilizando las herramientas de concurrencia de la biblioteca standard de Rust vistas en clase: Mutex, RwLock, Semáforos (del crate std-semaphore), Channels, Barriers y Condvars.*
//! 
//! ## Resolución
//! 
//! ### Estructuras
//! 
//! #### Flight Reservation
//! En primer lugar, se crea una estructura que representa una reserva de vuelo. A cada vuelo ingresado por consola, se le asignara un ID para ayudarnos a identificarlo.
//! Ademas, la estructura cuenta con la información necesaria para que el vuelo se pueda reservar con las configuraciones pedidas. Se almacenara su origen y destino, la aerolinea correspondiente a la que se le realizara el request y si el pedido incluye o no la reserva de hotel.
//! 
//! ```
//! pub struct FlightReservation {
//!    pub id: i32,
//!    pub origin: String,
//!    pub destination: String,
//!    pub airline: String,
//!    pub hotel: bool,
//!}
//!```
//! 
//! #### Statistics
//! Estructura que contiene las estadísticas de la aplicación. Por un lado, contamos con un acumulador de tiempo para poder estimar el tiempo promedio que toma una reserva desde que ingresa el pedido hasta que es finalmente aceptada. Por otro lado, un `HashMap` en donde se iran guardando todas las rutas (origen - destino) realizadas para poder llevar una estadística de las rutas mas frecuentes.
//! ```
//! pub struct Statistics {
//!    sum_time: Arc<RwLock<i64>>,
//!    destinations: Arc<RwLock<HashMap<String, i64>>>,
//! }
//! ```
//! 
//! Como se puede ver en la estructura, ambas estructuras son `Arc` para que se puedan usar en varios threads. Ademas, se usa `RwLock` para proveer seguridad a la hora de leer y escribir en las mismas. Esto se debe a que todos los pedidos que ingresan al sistema van a estar intentando acceder a los recursos de estadisticas, es por eso que es necesario el uso de un mecanismo de sincronismo para que no haya conflictos. `RwLock` nos va a permitir tener un escritor (lock exlusivo) o varios lectores a la vez(lock compartido).
//! 
//! #### AppState
//! Esta última estructura se trata del estado compartido que se compartirá en cada thread que escuche nuevas solicitudes. 
//! La estrcutura contiene:
//! - Las aereolinas del tipo `Airlines`, que se trata de un mapa de todas las Aereolineas con webservice disponibles en nuestro sistema. `Airlines` es un `HashMap` de tipo `<String, Arc<Semaphore>>`, en donde la clave es el nombre de la aereolinea. Y el valor es lo que simula ser el webservice, en este caso, un `Semaphore` que nos permitirá controlar la cantidad de solicitudes que se pueden realizar a cada webservice, teniendo en cuenta que cada aereolinea cuenta con un `rate_limit`. 
//!  Este mapa se popula a partir de un archivo `src/configs/airlines.txt`, el cual indica todos los nombres de las aereolineas junto a los N pedidos que puede responder de forma concurrente.
//! - La estructura de estadísticas `Statistics` para poder acceder y agregar estadísticas a la aplicación.
//! - El `logger_sender` para poder enviar mensajes al canal de logs desde cada thread. Para lograr este pasaje de mensajes al canal de logs, se usa un `Sender` que permite enviar mensajes al otro lado del canal (multiples consumidores y un solo productor).
//!
//! ```
//! struct AppState {
//!     airlines: Airlines,
//!     statistics: Statistics,
//!     logger_sender: Sender<LoggerMsg>,
//! }
//! ```
//! 
//! ### Implementación
//! 
//! #### Inicialización
//! 
//! El main esta compuesto por una serie de threads con diferentes tareas: 
//! 
//! - `logger`: El primer thread que se abre es el logger, que se encarga de escribir tanto por consola como en el archivo de loggueo, los mensajes que se van a ir recibiendo. Como se explico previamente, esto esta implementado con un `mpsc::channel` con el objetivo que desde cualquier lugar de la aplicación se pueda enviar mensajes con el `Sender` y desde el thread `logger` los mensajes sean leidos por el `Receiver`.
//! - `http-server`: Se hace uso de Actix web para recibir requests reales por consola, por lo que este thread crea la App con el estado de la aplicación `AppState` y se queda a la espera de requests para resolverlos.
//! - `keyboard-loop`: Por último, por detras tenemos al keyboard que se encarga de recibir dos posibles comandos: 'S'/'STATS' que nos permitirá mostrar las estadísticas de la aplicación, y 'Q'/'QUIT' que nos permitirá salir de la aplicación.
//! 
//! #### Aplicación
//! 
//! Una vez que ya tenemos todo el sistema inicializado, nuestro sistema ya esta listo para recibir nuevos requests. Si todos los parametros ingresados son correctos, se procede a realizar la reserva. Si no, se muestra un mensaje de error.
//! 
//! La lógica de la aplicación se encuentra en el archivo `src/threads/alglobo.rs` . En primer lugar se abre un nuevo thread para poder ejecutar concurrentemente el request a la aereolinea por un lado y por el otro lado el request al hotel si el mismo lo requiere(en caso de que el pedido incluya el modo de paquete completo).
//! 
//! En el caso de que la reserva sea por paquete, el pedido se envia al webservice del hotel y como sus reservas nunca se rechazan, el tiempo que tarda en procesar la respuesta simplemente se simula con un sleep de un tiempo random.
//! 
//! De forma simultanea, se envia el pedido a la aereolinea correspondiente. El tiempo que demora en realizar el request va a depender de la cantidad de request que soporta la aereolinea concurrentemente, ya que si la misma esta respondiendo la cantidad maxima de pedidos, el request de la aereolinea se va a bloquear y debera esperar a que un pedido anterior termine. Esto esta resuelto por el mismo semaforo que solo le va a permitir acceso a los pedidos si su contador interno es positivo, cada pedido que ingresa adquiere el semaforo decrementando en uno el contador, una vez que finaliza el pedido se incrementa el contador desbloqueando un hilo. 
//! Ademas la aerolínea puede aceptar el pedido o recharzarlo (se simula con un random booleano). Si es rechazado, el sistema espera `retry_seconds` segundos para reintentar el pedido. La cantidad de segundos para reintentar es configurable via variable de entorno `RETRY_SECONDS`. Por ultimo va a depender del tiempo que tarda en procesar el request que tambien es simulado simplemente con un sleep de tiempo random.
//! 
//! El resultado final de la reserva entonces necesitará que ambos pedidos (hotel y aerolinea) se completen en el caso de ser paquete o unicamente el pedido a la aereolinea. Es decir que no se puede agregar las estadisticas ni finalizar el request hasta que ambos threads hayan finalizado, y eso se resuelve a partir de monitores, esta herramienta nos brinda la posibilidad de esperar hasta que se cumpla una condición, en este caso si se reserva un paquete se debe esperar que ambos pedidos sean completados y sino solamente el request a la aereolinea.
//! 
//! Una vez que se completa el request a la aereolinea, se procede a agregar las estadísticas correspondientes, se suma el tiempo total que tardo en procesarse el pedido de principio a fin y se agrega la ruta solicitada(para agregar estas estadisticas, sera necesario obtener el lock para poder leer el estado actual de las estadisticas y agregar las nuevas).
//! 
//! ![Threads](../../../../img/threads.jpg)
//! 
//! # Parte B
//! La resolución de la primera parte del Trabajo Práctico se encuentra en la carpeta `src/atix` y se podrá ejecutar con `cargo run --bin atix`.
//! 
//! ## Enunciado
//! *Implementar la aplicación basada en el modelo de Actores, utilizando el framework Actix.*
//! 
//! ## Resolución
//! 
//! ### Actores
//! 
//! #### StatActor
//! 
//! El actor `StatActor` se encarga de manejar las estadísticas de la aplicación. La estructura del actor cuenta con la acumulacion de los tiempos que toman los request, un `HashMap` con las rutas solicitadas y un `HashMap` con los IDs de los request junto con un contador para saber si finalizo su procesamiento.
//! 
//! ```
//! pub struct StatsActor {
//! sum_time: i64,
//! destinations: HashMap<String, i64>,
//! flights: HashMap<i32, i32>,
//! }
//! ```
//! 
//! Este actor puede recibir un mensaje a la vez del tipo `Stat`. Al recibir este tipo de mensajes, si el request esta finalizado(es decir que si se trata de un paquete, finalizo tanto el pedido del hotel como el de la aereolinea), entonces se procede a sumar el tiempo de procesamiento al contador de tiempos totales y se agrega la ruta al `HashMap` de rutas frecuentes. Además imprimer por consola las estadisticas hasta el momento que incluyen la cantidad de vuelos, el tiempo total de procesamiento, el tiempo promedio de procesamiento y las 3 rutas más frecuentes.
//! 
//! Por otro lado, puede recibir un mensaje del tipo `FinishMessage` que indica que ya no quedan requests por procesar, por lo que se procede a finalizar la aplicación.
//! 
//! #### Airline
//!
//! El actor `Airline` simula el webservice de la aereolinea. La estructura unicamente cuenta con la referencia al `StatActor` para poder enviarle los mensajes de estadisticas una vez que termina de procesar el request.
//! 
//! A diferencia de `StatActor`, este actor se implementa con un `SyncContext` y esto se debe a que este actor se ejecuta en un `SyncArbitrer` que permite ejecutar `rate_limit` actores simultaneamente. Por lo que, por cada aereolinea, se tiene un `SyncArbitrer` que permite ejecutar N `Airline` simultaneamente acorde a su `rate_limit` establecido en el archivo `src/config/ariline.txt`.
//! 
//! Este actor recibe unicamente mensajes del tipo `InfoFlight` y el actor va a simular el procesamiento del request, es decir, va a simular el tiempo que tarda en procesar el request. Este tiempo estare compuesto de la misma manera que esta explicado en la parte A del Trabajo Práctico, es decir que el tiempo va a depender de: cuantos request se pueden procesar simultaneamente, el tiempo que tarda en procesar un request(sleep con duración random) y como puede rechazar los pedidos, se esperaran `retry_seconds` segundos si se rechaza para reintentar el pedido, hasta que se acepte.
//! 
//! Una vez que completa el request, realiza un `try_send` al `StatActor` para enviarle el mensaje de estadisticas correspondiente con el tiempo que tardo en procesar el pedido.
//! 
//! #### Hotel
//! 
//! El actor `Hotel` simula el webservice del hotel. Al igual que la aereolinea, la estructura unicamente cuenta con la referencia al `StatActor` para poder enviarle los mensajes de estadisticas una vez que termina de procesar el request.
//! 
//! El Hotel también es ejecutado en un `SyncArbitrer` que permite ejecutar todos los request en simultaneo.
//! 
//! Este actor recibe mensajes del tipo `InfoFlight` y a los mismos responde simulando el procesamiento del request, es decir, va a simular el tiempo que tarda en procesar el request. Pasado el tiempo de procesamiento (sleep de duracion random), se enviara un mensaje al `StatActor` para avisarle que se completo el request y se le mandaran las estadisticas correspondientes con el tiempo que tardo en procesar el pedido.
//! 
//! ### Mensajes
//! 
//! #### InfoFlight
//! 
//! Mensaje que se envía a los actores `Airline` y `Hotel` para indicar que se recibio un request de vuelo. Esta compuesto por la información del vuelo y el tiempo que comenzo a procesarse el request. La respuesta esperada para este tipo de mensajes es vacia.
//! 
//! ```
//! pub struct InfoFlight {
//! pub flight_reservation: FlightReservation,
//! pub start_time: std::time::Instant,
//! }
//! ```
//! 
//! #### Stat
//! 
//! Mensaje que se envía al actor `StatsActor` para indicar que finalizo de procesarse el request de vuelo. Esta compuesto por el tiempo de procesamiento de un request y `FlightReservation` para conocer la informacion del vuelo. La respuesta esperada para este tipo de mensajes es vacia.
//! 
//! ```
//! pub struct Stat {
//! pub elapsed_time: u128,
//! pub flight_reservation: FlightReservation,
//! }
//! ```
//! 
//! ## Testing
//! 
//! - Para la parte A, se realizan pruebas de volumen gracias a el uso de Atix web, en donde con mayor facilidad se logro enviar muchos pedidos en simultaneo para validar el funcionamiento del programa.
//! - Se realizan pruebas automatizadas en donde se realizan varias pruebas de una vez, para validar el funcionamiento del programa, implementando nuevamente aquellos métodos que no son deterministicos.
//! 
//! ## Post Mortem
//! 
//! - try_send()
//! - condvar por barrieres
//! - loom
//! - atixweb en actroes
//! - exxplicar porque no usamos stdout para el log (las stats te lo cagan)
//!
//!
//! Ideas de Todos:
//!
//! Hablar de correctitud, estado mutable compartido, por que no es fork join, barriers y semaforos
//!
//!
//! Una explicación del diseño y de las decisiones tomadas para la implementación de la solución.
//!
//! Detalle de resolución de la lista de tareas anterior.
//!
//! Diagrama que refleje los threads, el flujo de comunicación entre ellos y los datos que intercambian.
//!
//! 
//! Clavar un par de screenshots de htop
//!
//! Diagramas de entidades realizados (structs y demás).
