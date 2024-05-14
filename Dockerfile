FROM ubuntu:latest
MAINTAINER your_name <email_address>

# Оновлюємо стандартні пакети
# Оновлюємо стандартні пакети
RUN apt-get update && apt-get upgrade -y && \
    apt-get install -y curl --fix-missing
# Копіюємо виконуваний файл у контейнер
COPY ./ /app/

# Перевіряємо робочу директорію
WORKDIR /app

# Виконуємо виконуваний файл
CMD ["./easy_english"]
