const Blog=require('../database/blog')
const blogPost=(req,res,next)=>{
    const {timeStamp,title,preview,blogText}=req.body
    if (!timeStamp || !title || !preview || !blogText){
        return res.status(400).send({
                success: false,
                message: 'Error: All field must be filled cannot be blank.'
        })
    }else{
        const blog= async ()=>{
            try{
                const newBlogPost= new Blog();
                newBlogPost.timeStamp=timeStamp;
                newBlogPost.title=title;
                newBlogPost.preview=preview;
                newBlogPost.blogText=blogText;
                const savedBlog= await newBlogPost.save();
                return res.status(201).send({
                    success: true,
                    message: "Sent New Blog Post"
                })
            }catch(error){
                return res.status(500).send({
                    success: false,
                    message: "Error: Server Error"
                })
            }
        }
        blog()
    }

}
module.exports=blogPost
