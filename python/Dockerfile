FROM python:3.10-slim

LABEL MAINTAINER="jw59615@gmail.com"
ENV WORK_DIR="/opt/python-rust-app/"
SHELL ["/bin/bash", "-lc"]

RUN apt update && apt-get upgrade -y && apt install -y curl git

WORKDIR /root
# download miniconda installer
RUN curl -sL \
"https://repo.anaconda.com/miniconda/Miniconda3-latest-Linux-x86_64.sh" > \
"miniconda3.sh" && chmod +x ./miniconda3.sh

# run conda installer and init shell
RUN ./miniconda3.sh -b
RUN source ./miniconda3/bin/activate && ./miniconda3/bin/conda init 

# make new folder to store project
RUN mkdir ${WORK_DIR}

# copy over source and environment spec
COPY ./environment.yml ${WORK_DIR}environment.yml
COPY ./src ${WORK_DIR}src

# chdir to workdir 
WORKDIR ${WORK_DIR}

# set configuration for conda
RUN conda config --add channels conda-forge
RUN conda config --set channel_priority strict

# create conda environment
RUN conda env create --file ${WORK_DIR}environment.yml 

RUN conda activate ds

CMD ["python", "./src/build.py"]

