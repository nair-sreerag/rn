const axios = require("axios").default;

const TOTAL_REQUESTS_TO_MAKE = 100;
const url = "http://localhost:9000/hello-world";

async function main() {
  console.time();

  const arr = [];

  for (let i = 0; i < TOTAL_REQUESTS_TO_MAKE; i++) {
    arr.push(axios.get(url));
  }

  const responses = await Promise.all(arr);

  responses.forEach((r) => {
    console.log("response data", r.data);
  });

  console.timeEnd();
}

main()
  .then(() => {
    console.log("donezo!");
  })
  .catch((err) => {
    console.error("got error", err);
  });
