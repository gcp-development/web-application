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
  <li><a href="" target="_self">library-service</a></li>
  <ul>
    <li><a href="" target="_self">Dokerfile</a></li>
    <li><a href="" target="_self">Pods</a></li>
  </ul>
</ul>
<hr>

## library-service

### Dockerfile

Create the sqlx-data.json file.
```bash
cargo sqlx prepare
```
Note : to install the [sql-cli](https://crates.io/crates/sqlx-cli) only for postgres, just run "cargo install sqlx-cli --no-default-features --features native-tls,postgres"

```bash
docker build -f /library-service-dockerfile.dev -t library-service:1.0 .
```
Note : to list images just run "docker image ls"

```bash
docker tag p2p-pod-a:1.0 {docker.hub}/library-service:1.0
```

```bash
docker push {docker.hub}/library-service:1.0
```
## minikube setup

minikube version

![image](https://user-images.githubusercontent.com/76512851/222912565-1742b8a7-2b23-45f2-9007-bb1ade990be1.png)

Create a cluster
```bash
minikube start --driver=docker -p demo
```
![image](https://user-images.githubusercontent.com/76512851/222913292-c33b7a20-b00f-49f8-a8df-3bca70837d51.png)

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

Verifying pods.
```bash
kubectl get pods --namespace=web-application
```
![image](https://user-images.githubusercontent.com/76512851/222916601-584276d3-9cc1-4cd7-8f29-150cb1a01613.png)

Log into the container.
```bash
kubectl logs -f library-754659bc5d-j6jx2 --namespace=web-application
```
![image](https://user-images.githubusercontent.com/76512851/222916517-ff61b8a4-a7bb-4ef5-9e23-613a1415b362.png)

Create a [load balancer service](https://kubernetes.io/docs/concepts/services-networking/service/#loadbalancer) for library-service.
```bash
kubectl apply -f 8_library-service.yml
```

```bash
minikube tunnel -p demo
```


<hr>
References:<br>

[Dockerfile reference](https://docs.docker.com/engine/reference/builder/)<br>
[minikube](https://minikube.sigs.k8s.io/docs/)<br>
[Kubernetes Documentation](https://kubernetes.io/docs/home/)<br>
[Rust Official Image](https://hub.docker.com/_/rust)
