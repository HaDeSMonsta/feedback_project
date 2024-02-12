FROM gradle:8.5-jdk21-alpine AS build

ENV GRADLE_USER_HOME /.gradle

RUN mkdir /.gradle && chmod 777 /.gradle

WORKDIR /home/gradle/project

COPY --chown=gradle:gradle build.gradle.kts /home/gradle/project
COPY --chown=gradle:gradle settings.gradle.kts /home/gradle/project/

RUN gradle dependencies --no-daemon

COPY --chown=gradle:gradle . /home/gradle/project

RUN gradle build --no-daemon --parallel

FROM openjdk:21-bullseye

WORKDIR /app

COPY --from=build /home/gradle/project/build/libs/FeedbackDataConnection.jar ./FeedbackDataConnection.jar

CMD ["java", "-jar", "./FeedbackDataConnection.jar"]