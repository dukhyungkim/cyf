curl -i 'http://localhost:8080/200'
echo '\n'

curl -i 'http://localhost:8080/500'
echo '\n'

curl -i 'http://localhost:8080/404'
echo '\n'

curl -i 'http://localhost:8080/'
echo '\n'

curl -i -d "<em>Hi</em>" 'http://localhost:8080/'
echo '\n'

curl -i 'http://localhost:8080?foo=<strong>bar</strong>'
echo '\n'
