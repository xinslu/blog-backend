const Blog=require('../database/blog')
const getBlogPost=(req,res,next)=>{
    const {id}=req.body;
    if (!id){
        const getAll= async ()=>{
            var query= await Blog.find()
            return res.status(200).send({results: query})
        }
        getAll()
    }else{
        const getOne=async ()=>{
            try{
                var query= await Blog.find({_id:id})
                return res.status(200).send({
                    success: true,
                    results: query})
            }catch(error){
                console.log("in error")
                return res.status(200).send({
                    success: false})
            }

        }
        getOne()
    }
}
module.exports=getBlogPost
