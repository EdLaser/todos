docker run --rm -p 8089:8089 -v $PWD:/mnt/locust locustio/locust -f /mnt/locust/locustfile.py --headless -u 2 -r 1 -t 30 --host http://localhost:8000
```