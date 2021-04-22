docker build -t rusttext .
docker run --name dev -it -v ${PWD}:/app/project rusttext