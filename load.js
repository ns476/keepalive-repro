// Install k6:
// k6 run --vus 30 --duration 30s scripts/load.js
import { URL } from 'https://jslib.k6.io/url/1.0.0/index.js';
import http from 'k6/http';

export default function () {
    http.get('http://localhost:8080/');
}
