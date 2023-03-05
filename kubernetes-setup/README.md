# Kubernetes Setup

It's assumed that these software are installed and running:

<ul>
  <li><a href="https://docs.docker.com/engine/install/ubuntu/" target="_blank">docker</a></li>
  <li><a href="https://minikube.sigs.k8s.io/docs/start/" target="_blank">minikube</a></li>
  <li><a href="https://kubernetes.io/docs/tasks/tools/install-kubectl-linux/" target="_blank">kubectl</a></li>
</ul>
<hr>

## Table of Contents<br>
<ul>
  <li><a href="https://github.com/gcp-development/web-application/tree/main/kubernetes-setup#minikube-setup" target="_self">minikube setup</a></li>
  <li><a href="https://github.com/gcp-development/web-application/tree/main/kubernetes-setup#dockerfilelibrary-service" target="_self">Dockerfile(library-service)</a></li>
  <li><a href="https://github.com/gcp-development/web-application/blob/main/kubernetes-setup/README.md#smoke-tests" target="_self">Smoke tests</a></li>
    <li><a href="https://github.com/gcp-development/web-application/tree/main/kubernetes-setup#kubernetes-manifests" target="_self">Kubernetes manifests</a></li>
</ul>
<hr>

## minikube setup

minikube version

![image](https://user-images.githubusercontent.com/76512851/222912565-1742b8a7-2b23-45f2-9007-bb1ade990be1.png)

Create a [cluster](https://minikube.sigs.k8s.io/docs/commands/profile/)
```bash
minikube start --driver=docker -p demo
```
![image](https://user-images.githubusercontent.com/76512851/222913292-c33b7a20-b00f-49f8-a8df-3bca70837d51.png)

<hr>

## Dockerfile(library-service)

Setup the connection string to reflect the kubernetes setup.

![image](https://user-images.githubusercontent.com/76512851/222966080-9e1485a7-881c-40c1-b816-84b6dcdd8c5a.png)

Create the sqlx-data.json file.
```bash
cargo sqlx prepare
```
Note : to install the [sql-cli](https://crates.io/crates/sqlx-cli) only for postgres, just run "cargo install sqlx-cli --no-default-features --features native-tls,postgres"

![image](https://user-images.githubusercontent.com/76512851/222965937-03ce8a0e-0b0a-46e5-a84e-bafa57fc058f.png)

Create the library-service image.
```bash
docker build -f /library-service-dockerfile.dev -t library-service:1.0 .
```
Note : to list images just run "docker image ls"

Tag our image to the docker-hub repository.
```bash
docker tag library-service:1.0 {docker.hub}/library-service:1.0
```
Push the image to the docker-hub repository.
```bash
docker push {docker.hub}/library-service:1.0
```

<hr>

## Kubernetes manifests

Create a namespace.
```bash
kubectl apply -f 1_namespace.yml
```
![image](https://user-images.githubusercontent.com/76512851/222913681-a1d8f917-2fed-4ffa-8ded-90470a354d43.png)

Create environment variables.
```bash
kubectl apply -f 2_postgres-configmap.yml
```
Note:This is only for demonstration purposes. [ConfigMaps](https://kubernetes.io/docs/concepts/configuration/configmap/) does not provide secrecy or encryption. To store confidential data we should use [Secrets](https://kubernetes.io/docs/concepts/configuration/secret/).

![image](https://user-images.githubusercontent.com/76512851/222915388-f776f3bb-8ff3-478d-b3cf-a67b57582970.png)

Create a [persistent volume](https://kubernetes.io/docs/concepts/storage/persistent-volumes/) for [postgres](https://www.postgresql.org/).
```bash
kubectl apply -f 3_postgres-pv.yml
```

![image](https://user-images.githubusercontent.com/76512851/222915466-8ed1e363-85b4-46b6-92fd-70a4730ae031.png)

Create a persistent volume claim for postgres.
```bash
kubectl apply -f 4_postgres-pvc.yml
```

![image](https://user-images.githubusercontent.com/76512851/222915553-4752d118-04c8-4169-be83-260fd5d5564b.png)

Deploy postgres.
```bash
kubectl apply -f 5_postgres-deployment.yml
```
![image](https://user-images.githubusercontent.com/76512851/222915679-ebaec9de-6c94-4854-955e-ba37e7ca10e4.png)

Create a [load balancer service](https://kubernetes.io/docs/concepts/services-networking/service/#loadbalancer) for postgres.
```bash
kubectl apply -f 6_postgres-service.yml
```
![image](https://user-images.githubusercontent.com/76512851/222916006-6c6dd2f5-47ca-4198-b8a7-3149fd9f67f7.png)

Deploy library-service application.
```bash
kubectl apply -f 7_library-deployment.yml
```
![image](https://user-images.githubusercontent.com/76512851/222916452-2f891197-f1ef-4fda-89b6-9e6b158427ca.png)

Verifying the [pods](https://kubernetes.io/docs/concepts/workloads/pods/).
```bash
kubectl get pods --namespace=web-application
```
![image](https://user-images.githubusercontent.com/76512851/222916601-584276d3-9cc1-4cd7-8f29-150cb1a01613.png)

Log into the [container](https://kubernetes.io/docs/concepts/containers/).
```bash
kubectl logs -f library-754659bc5d-j6jx2 --namespace=web-application
```
![image](https://user-images.githubusercontent.com/76512851/222965759-62568463-ca39-4850-ab64-1cb53a0504f9.png)

Create a [load balancer service](https://kubernetes.io/docs/concepts/services-networking/service/#loadbalancer) for library-service.
```bash
kubectl apply -f 8_library-service.yml
```
![image](https://user-images.githubusercontent.com/76512851/222916925-22fc37a4-31c7-48e6-8a2a-dabdda858e63.png)

Start the minikube [load balancer](https://minikube.sigs.k8s.io/docs/handbook/accessing/#loadbalancer-access)
```bash
minikube tunnel -p demo
```
![image](https://user-images.githubusercontent.com/76512851/222916983-2ba9841a-32af-431e-b1f4-486f0ebfb031.png)

![image](https://user-images.githubusercontent.com/76512851/222917137-6a54dc5b-80f8-43b2-aaf1-d30603a270cb.png)

Get the [minikube ip](https://minikube.sigs.k8s.io/docs/commands/ip/) for the node.

```bash
minikube ip -p demo
```
![image](https://user-images.githubusercontent.com/76512851/222918074-f1a14a91-192e-4a80-809d-b57eac2dd91d.png)

Add a host alias record in our /etc/hosts file.

![image](https://user-images.githubusercontent.com/76512851/222917956-a33797a8-1061-460e-a8ab-82f928d9ddaa.png)

<hr>

## Smoke tests

Test the [postgres database](https://www.postgresql.org/) using [pgAdmin](https://www.pgadmin.org/download/)

![image](https://user-images.githubusercontent.com/76512851/222918441-29c02a2b-c556-46b9-9fe1-abe820a39527.png)

Connecting to the library database.

![image](https://user-images.githubusercontent.com/76512851/222918644-b17640c8-7c9d-451a-b407-a8a058a0d3b8.png)

Execute this [sql script](https://github.com/gcp-development/web-application/blob/main/library-service/src/model/sql_scripts/1_tbBooks.sql).

![image](https://user-images.githubusercontent.com/76512851/222968007-704da7cc-0d64-437e-ba5a-ecb5fff981fb.png)

Test the library-service using [Postman](https://www.postman.com/downloads/)

![image](https://user-images.githubusercontent.com/76512851/222967580-819bba06-fc85-476f-bd18-6cf88cf0bded.png)

<hr>
References:<br>

[Dockerfile reference](https://docs.docker.com/engine/reference/builder/)<br>
[minikube](https://minikube.sigs.k8s.io/docs/)<br>
[Kubernetes Documentation](https://kubernetes.io/docs/home/)<br>
[Rust Official Image](https://hub.docker.com/_/rust)
