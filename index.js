const express=require('express')
const app = express()
const connectDB = require('./database/db')
const bodyParser = require('body-parser')
const port = 5000
const Blog=require('./database/blog')
const cors=require('cors')
const blogPost =require("./controllers/blogPost")
const getBlogPost =require("./controllers/getBlogPost")
require('dotenv').config()


app.use(bodyParser.json())
app.use(bodyParser.urlencoded({ extended: true }))
app.use(cors())


const start = async () => {
  try {
    await connectDB(process.env.MONGO_URI)
    app.listen(process.env.PORT || port, console.log(`http://localhost:${port}`))
  } catch (error) {
    console.log(error)
  }
}

app.post("/",blogPost)
app.post("/getblogpost",getBlogPost)


start()
