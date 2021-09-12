const mongoose=require('mongoose')

const blogSchema = mongoose.Schema({
    timeStamp: {
        type: String,
        default: ''
    },
    subject: {
        type: String,
        default: ''
    },
    title: {
        type: String,
        default: ''
    },
    preview: {
        type: String,
        default: ''
    },
    blogText: {
        type: String,
        default: ''
    }
})

module.exports=mongoose.model('Blog', blogSchema)
