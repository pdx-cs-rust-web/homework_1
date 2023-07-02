


> Homework 1 Bug Report  ("Request Detectives")
> Farani Lucero (rlucero@pdx.edu)


**Bug 1**
- *API route:* /questions
- *Args/Params:* (none)
- *Method:* GET
- *Handler:* fn get_questions
- *Summary:* Accessing this API endpoint should provide all the questions in the mock database. Instead, however, a default question is created/saved into the mock database each time this endpoint is called. So, duplicate calls will produce multiple default questions in the mock database.
- *Discovery:* I found this bug by using Postman. I also added code to the client portion of the application to verify I was receiving the same results.
- *Why:* I believe the bug is occurring primarily based on the code on lines 18-26 (of backend/src/handlers.rs).
- *How to Reproduce:* After starting up the backend API, simplest way to reproduce is to point a web browser to http://localhost:8088/questions. The default question should be displayed in JSON format. Refreshing this screen in the browser should display more results (duplicates of the default question) in JSON.



**Bug 2**
- *API route:* /question
- *Args/Params:* question_id (/question?question_id=[value])
- *Method:* DELETE
- *Handler:* fn delete_question
- *Summary:* Accessing this API endpoint should delete the question specified by the question_id in the Query param. Instead, however, all questions are deleted except the one specified by the question_id in the Query param.
- *Discovery:* I found this bug by using Postman. I also added code to the client portion of the application to verify I was receiving the same results.
- *Why:* I believe the bug is occurring primarily based on the code on lines 82 (of backend/src/handlers.rs).
- *How to Reproduce:* After starting up the backend API, simplest way to reproduce is to use Postman.
	1. Create a new request in Postman.
	2. Change the default method from GET to DELETE.
	3. Enter http://localhost:8088/question into the URL field
	4. Under Params, create a Query Param called "question_id" under Key
	5. Under Value, enter the question_id you wish to delete
	6. Click Send
