import http.client

url = ''
h1 = http.client.HTTPConnection(url)
h1.request('GET')
response = h1.getresponse()
print(response.read())
