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

We will make a Rust program to call to this API and parse the results,
and then store present those in a useful format. Maybe providing a weather
service on our local network.
