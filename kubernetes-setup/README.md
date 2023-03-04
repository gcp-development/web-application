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
  <li><a href="https://github.com/gcp-development/peer-to-peer/blob/main/kubernetes-setup/README.md#minikube-setup" target="_self">minikube setup</a></li>
  <li><a href="https://github.com/gcp-development/peer-to-peer/blob/main/kubernetes-setup/README.md#scenario-p2p-app" target="_self">Scenario p2p-app</a></li>
  <ul>
    <li><a href="https://github.com/gcp-development/peer-to-peer/tree/main/p2p-setup#dockerfile" target="_self">Dokerfile</a></li>
    <li><a href="https://github.com/gcp-development/peer-to-peer/tree/main/p2p-setup#pods" target="_self">Pods</a></li>
  </ul>
  <li><a href="https://github.com/gcp-development/peer-to-peer/blob/main/kubernetes-setup/README.md#scenario-mdns-app">Scenario mdns-app</a></li>
  <ul>
    <li><a href="https://github.com/gcp-development/peer-to-peer/blob/main/kubernetes-setup/README.md#dockerfile" target="_self">Dokerfile</a></li>
    <li><a href="https://github.com/gcp-development/peer-to-peer/blob/main/kubernetes-setup/README.md#pods" target="_self">Pods</a></li>
  </ul>
  <li><a href="https://github.com/gcp-development/peer-to-peer/blob/main/kubernetes-setup/README.md#scenario-dht-app" target="_self">Scenario dht-app</a></li>
  <ul>
    <li><a href="https://github.com/gcp-development/peer-to-peer/blob/main/kubernetes-setup/README.md#dockerfile-1" target="_self">Dokerfile</a></li>
    <li><a href="https://github.com/gcp-development/peer-to-peer/blob/main/kubernetes-setup/README.md#pods-1" target="_self">Pods</a></li>
  </ul>
</ul>
<hr>

## minikube setup

minikube version

```bash
minikube start --driver=docker -p demo
```

```bash
kubectl apply -f 1_namespace.yml
```

```bash
kubectl apply -f 2_postgres-configmap.yml
```

```bash
kubectl apply -f 3_postgres-pv.yml
```

```bash
kubectl apply -f 4_postgres-pvc.yml
```

```bash
kubectl apply -f 5_postgres-deployment.yml
```

```bash
minikube tunnel -p demo
```

```bash
kubectl apply -f 6_postgres-service.yml
```

## library-service

### Dockerfile

Create the sqlx-data.json file.
```bash
cargo sqlx prepare
```
Note : to list images just run "docker image ls"

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

<hr>
References:<br>
[Dockerfile reference](https://docs.docker.com/engine/reference/builder/)<br>
[minikube](https://minikube.sigs.k8s.io/docs/)<br>
[Kubernetes Documentation](https://kubernetes.io/docs/home/)<br>
[Rust Official Image](https://hub.docker.com/_/rust)
