FROM tiangolo/nginx-rtmp

RUN mkdir /recordings
RUN chown -R www-data:www-data /recordings
COPY nginx.conf /etc/nginx/nginx.conf
