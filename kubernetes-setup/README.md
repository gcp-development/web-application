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

Create environment variables.[ConfigMaps](https://kubernetes.io/docs/concepts/configuration/configmap/)
```bash
kubectl apply -f 2_postgres-configmap.yml
```
Note:This is only for demonstration purposes.ConfigMap does not provide secrecy or encryption. To store confidential data we should use [Secrets](https://kubernetes.io/docs/concepts/configuration/secret/).

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


<hr>
References:<br>

[Dockerfile reference](https://docs.docker.com/engine/reference/builder/)<br>
[minikube](https://minikube.sigs.k8s.io/docs/)<br>
[Kubernetes Documentation](https://kubernetes.io/docs/home/)<br>
[Rust Official Image](https://hub.docker.com/_/rust)
