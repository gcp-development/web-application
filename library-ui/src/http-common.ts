import axios from "axios";

export default axios.create({
  baseURL: "http://demo:31726/",
  headers: {
    "Content-type": "application/json"
  }
});