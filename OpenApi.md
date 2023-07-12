---
title: backend v0.1.0
language_tabs:
  - rust: Rust
toc_footers: []
includes: []
search: true
highlight_theme: darkula
headingLevel: 2

---

<!-- Generator: Widdershins v4.0.1 -->

<h1 id="backend">backend v0.1.0</h1>

> Scroll down for code samples, example requests and responses. Select a language for code samples from the tabs above or the mobile navigation menu.

 License: 

<h1 id="backend-handlers">handlers</h1>

## create_question

<a id="opIdcreate_question"></a>

`POST /question/`

> Example responses

> 200 Response

```json
{
  "type": "array",
  "items": {
    "type": "object",
    "required": [
      "id",
      "title",
      "content"
    ],
    "properties": {
      "content": {
        "type": "string"
      },
      "id": {
        "type": "integer",
        "minimum": 0
      },
      "tags": {
        "type": "array",
        "items": {
          "type": "string"
        },
        "nullable": true
      },
      "title": {
        "type": "string"
      }
    }
  }
}
```

<h3 id="create_question-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|Create new question|Inline|

<h3 id="create_question-responseschema">Response Schema</h3>

Status Code **200**

|Name|Type|Required|Restrictions|Description|
|---|---|---|---|---|
|*anonymous*|[[Question](#schemaquestion)]|false|none|none|
|» content|string|true|none|none|
|» id|[QuestionId](#schemaquestionid)|true|none|none|
|» tags|[string]¦null|false|none|none|
|» title|string|true|none|none|

<aside class="success">
This operation does not require authentication
</aside>

## delete_question

<a id="opIddelete_question"></a>

`DELETE /question/`

> Example responses

> 200 Response

```json
{
  "type": "array",
  "items": {
    "type": "object",
    "required": [
      "id",
      "title",
      "content"
    ],
    "properties": {
      "content": {
        "type": "string"
      },
      "id": {
        "type": "integer",
        "minimum": 0
      },
      "tags": {
        "type": "array",
        "items": {
          "type": "string"
        },
        "nullable": true
      },
      "title": {
        "type": "string"
      }
    }
  }
}
```

<h3 id="delete_question-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|Delete question with given ID|Inline|

<h3 id="delete_question-responseschema">Response Schema</h3>

Status Code **200**

|Name|Type|Required|Restrictions|Description|
|---|---|---|---|---|
|*anonymous*|[[Question](#schemaquestion)]|false|none|none|
|» content|string|true|none|none|
|» id|[QuestionId](#schemaquestionid)|true|none|none|
|» tags|[string]¦null|false|none|none|
|» title|string|true|none|none|

<aside class="success">
This operation does not require authentication
</aside>

## get_question_by_id

<a id="opIdget_question_by_id"></a>

`GET /question/:question_id`

> Example responses

> 200 Response

```json
{
  "type": "array",
  "items": {
    "type": "object",
    "required": [
      "id",
      "title",
      "content"
    ],
    "properties": {
      "content": {
        "type": "string"
      },
      "id": {
        "type": "integer",
        "minimum": 0
      },
      "tags": {
        "type": "array",
        "items": {
          "type": "string"
        },
        "nullable": true
      },
      "title": {
        "type": "string"
      }
    }
  }
}
```

<h3 id="get_question_by_id-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|Retrieve question with given ID|Inline|

<h3 id="get_question_by_id-responseschema">Response Schema</h3>

Status Code **200**

|Name|Type|Required|Restrictions|Description|
|---|---|---|---|---|
|*anonymous*|[[Question](#schemaquestion)]|false|none|none|
|» content|string|true|none|none|
|» id|[QuestionId](#schemaquestionid)|true|none|none|
|» tags|[string]¦null|false|none|none|
|» title|string|true|none|none|

<aside class="success">
This operation does not require authentication
</aside>

## get_questions

<a id="opIdget_questions"></a>

`GET /questions`

> Example responses

> 200 Response

```json
{
  "type": "array",
  "items": {
    "type": "object",
    "required": [
      "id",
      "title",
      "content"
    ],
    "properties": {
      "content": {
        "type": "string"
      },
      "id": {
        "type": "integer",
        "minimum": 0
      },
      "tags": {
        "type": "array",
        "items": {
          "type": "string"
        },
        "nullable": true
      },
      "title": {
        "type": "string"
      }
    }
  }
}
```

<h3 id="get_questions-responses">Responses</h3>

|Status|Meaning|Description|Schema|
|---|---|---|---|
|200|[OK](https://tools.ietf.org/html/rfc7231#section-6.3.1)|List all questions successfully|Inline|

<h3 id="get_questions-responseschema">Response Schema</h3>

Status Code **200**

|Name|Type|Required|Restrictions|Description|
|---|---|---|---|---|
|*anonymous*|[[Question](#schemaquestion)]|false|none|none|
|» content|string|true|none|none|
|» id|[QuestionId](#schemaquestionid)|true|none|none|
|» tags|[string]¦null|false|none|none|
|» title|string|true|none|none|

<aside class="success">
This operation does not require authentication
</aside>

# Schemas

<h2 id="tocS_CreateQuestion">CreateQuestion</h2>
<!-- backwards compatibility -->
<a id="schemacreatequestion"></a>
<a id="schema_CreateQuestion"></a>
<a id="tocScreatequestion"></a>
<a id="tocscreatequestion"></a>

```json
{
  "type": "object",
  "required": [
    "title",
    "content"
  ],
  "properties": {
    "content": {
      "type": "string"
    },
    "tags": {
      "type": "array",
      "items": {
        "type": "string"
      },
      "nullable": true
    },
    "title": {
      "type": "string"
    }
  }
}

```

### Properties

|Name|Type|Required|Restrictions|Description|
|---|---|---|---|---|
|content|string|true|none|none|
|tags|[string]¦null|false|none|none|
|title|string|true|none|none|

<h2 id="tocS_GetQuestionById">GetQuestionById</h2>
<!-- backwards compatibility -->
<a id="schemagetquestionbyid"></a>
<a id="schema_GetQuestionById"></a>
<a id="tocSgetquestionbyid"></a>
<a id="tocsgetquestionbyid"></a>

```json
{
  "type": "object",
  "required": [
    "question_id"
  ],
  "properties": {
    "question_id": {
      "type": "integer",
      "minimum": 0
    }
  }
}

```

### Properties

|Name|Type|Required|Restrictions|Description|
|---|---|---|---|---|
|question_id|integer|true|none|none|

<h2 id="tocS_Question">Question</h2>
<!-- backwards compatibility -->
<a id="schemaquestion"></a>
<a id="schema_Question"></a>
<a id="tocSquestion"></a>
<a id="tocsquestion"></a>

```json
{
  "type": "object",
  "required": [
    "id",
    "title",
    "content"
  ],
  "properties": {
    "content": {
      "type": "string"
    },
    "id": {
      "type": "integer",
      "minimum": 0
    },
    "tags": {
      "type": "array",
      "items": {
        "type": "string"
      },
      "nullable": true
    },
    "title": {
      "type": "string"
    }
  }
}

```

### Properties

|Name|Type|Required|Restrictions|Description|
|---|---|---|---|---|
|content|string|true|none|none|
|id|[QuestionId](#schemaquestionid)|true|none|none|
|tags|[string]¦null|false|none|none|
|title|string|true|none|none|

<h2 id="tocS_QuestionId">QuestionId</h2>
<!-- backwards compatibility -->
<a id="schemaquestionid"></a>
<a id="schema_QuestionId"></a>
<a id="tocSquestionid"></a>
<a id="tocsquestionid"></a>

```json
{
  "type": "integer",
  "minimum": 0
}

```

### Properties

|Name|Type|Required|Restrictions|Description|
|---|---|---|---|---|
|*anonymous*|integer|false|none|none|

