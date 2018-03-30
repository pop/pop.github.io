FROM nginx:1.13.10-alpine

MAINTAINER Elijah Caine M. Voigt <Elijah.Caine.MV@gmail.com>

COPY . /site

RUN /sbin/apk -U add python3 \
 && /usr/bin/pip3 install -r /site/requirements.txt \
 && /usr/bin/pelican /site/content -o /usr/share/nginx/html -s /site/pelicanconf.py --relative-urls

EXPOSE 80

STOPSIGNAL SIGTERM

CMD ["nginx", "-g", "daemon off;"]
