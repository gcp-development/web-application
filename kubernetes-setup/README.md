
```bash
minikube start -p demo
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
kubectl apply -f 6_postgres-service.yml
```

<hr>
References:<br>

[Kubernetes](https://kubernetes.io/docs/home/)<br>
