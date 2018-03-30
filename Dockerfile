FROM nginx:1.13.10-alpine

RUN /sbin/apk -U add python3

COPY requirements.txt /tmp/requirements.txt

RUN /usr/bin/pip3 install -r /tmp/requirements.txt

COPY . /site

RUN /usr/bin/pelican /site/content -o /usr/share/nginx/html -s /site/pelicanconf.py --relative-urls

EXPOSE 80

STOPSIGNAL SIGTERM

CMD ["nginx", "-g", "daemon off;"]
