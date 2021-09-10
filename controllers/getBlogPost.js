const Blog=require('../database/blog')
const getBlogPost=(req,res,next)=>{
    const {id}=req.body;
    console.log(id)
    if (!id){
        const getAll= async ()=>{
            var query= await Blog.find()
            return res.status(200).send({results: query})
        }
        getAll()
    }else{
        const getOne=async ()=>{
            var query= await Blog.find({_id:id})
            return res.status(200).send({results: query})
        }
        getOne()
    }
}
module.exports=getBlogPost
