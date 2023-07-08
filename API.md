# Question Server API
Bart Massey and Casey Bailey 2023

* `GET` `/`: Front page.
* `GET` `/questions`: Get list of questions.

  Each question has an integer `id`, a string `title`, some
  string `content` and optionally a list of `tag` strings.
  
  Example response:

      [
          {
              "id": 7,
              "title": "Sample Question",
              "content": "What is the question?",
              "tag": ["dumb", "sample"]
          }
      ]

* `GET` `/question?question_id=<id>`: Get question by id.

  This returns a question in the format described by `GET` `questions`.

  Example query: `https://example.org/question?question_id=7`

  Example response:

      {
          "id": 7,
          "title": "Sample Question",
          "content": "What is the question?",
          "tag": ["dumb", "sample"]
      }

* `POST` `/question`: Add a question.

  This accepts a JSON question consisting of a string
  `title`, some string `content`, and optionally a list of
  string `tags`. The result is the question together with a
  new integer question `id`.


  Example POST query: `https://example.org/question`
  
      {
          "title": "Sample Question",
          "content": "What, another sample question?",
          "tag": ["dumb", "sample"]
      }

  Example response:

      {
          "title": "Sample Question",
          "content": "What, another sample question?",
          "tag": ["dumb", "sample"]
      }

* `DELETE` `/question?question_id=<id>`: Delete question by id, if present.

  Example DELETE query: `https://example.org/question?7`
