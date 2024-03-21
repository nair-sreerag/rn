const app = require("express")();

const PORT = process.env.PORT || 35577;

app.get("/hello-world", async (req, res) => {
  console.log("got a request");

  setTimeout(() => {
    return res.json({
      message: "Hello, world!",
      ts: Date.now(),
    });
  }, 2000);
});

// app.get("*", (req, res) => {

//     console.log("here ->>> ");

//     return res.json({

//         message: "hit this"
//     });
// })

app.listen(PORT, () => {
  console.log(`Server running at port ${PORT}`);
});
