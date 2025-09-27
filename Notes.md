# Project Notes

## Primality testing

This is a simple, single-threaded numerical benchmark. It first checks if a number `n` is prime,
and if it is prime uses Lucas-Lehmer to check if `2^n - 1` is a Mersenne prime.

See [GIMPS](https://www.mersenne.org/primes/) for a list of known Mersenne primes.

## Weather

API call for local weather (Blacksburg, VA):

```
https://api.openweathermap.org/data/2.5/weather?q=blacksburg,va,us&appid={API key}
```

The API key is in the uncommitted file `OPENWEATHERMAP_KEY`.

The `src/bin/weather.rs` program makes a call to this API, extracts part
of the resulting weather data, and stores some of that data in a sqlite
database. It's great that cross-compilation just works for all the
dependencies we're using, including `sqlx` with `sqlite`.

__Testing on the Pi:__

```bash
sudo apt install sqlite3
sqlite3 database.sqlite3
```

```sql
sqlite> .mode table
sqlite> select * from weather_reading;
```

__Next plan:__

We will use something like the `daemonize` crate to make this a daemon that
we can keep running on our Raspberry Pi server, and we will add an API that
we can query on our local network to get weather information.

Really the point of this is to see what the Pi 3B+ can do, and what we can
do with it. And for fun, of course.
