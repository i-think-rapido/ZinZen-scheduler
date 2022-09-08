import { schedule } from "../js-api/scheduler.js";
import { assertEquals } from "https://deno.land/std@0.141.0/testing/asserts.ts";

Deno.test("realistic schedule", () => {
  assertEquals(
      schedule(
    {
      "startDate": "2022-09-01T00:00:00",
      "endDate": "2022-10-01T00:00:00",
      "goals": [
        {
          "id": 1,
          "title": "walk",
          "duration": 1,
          "repetition": "daily",
          "after_time": 17,
          "before_time": 20 
        },
        {
          "id": 2,
          "title": "read",
          "duration": 1,
          "repetition": "daily",
          "after_time": 19,
          "before_time": 22 
        },
        {
          "id": 3,
          "title": "piano practice",
          "duration": 2,
          "repetition": "wednesdays",
          "after_time": 12,
          "before_time": 14 
        },
        {
          "id": 4,
          "title": "gym",
          "duration": 2,
          "repetition": "daily",
          "after_time": 5,
          "before_time": 7 
        },
        {
          "id": 5,
          "title": "work",
          "duration": 8,
          "repetition": "daily",
          "start": "2022-09-01T00:00:00",
          "deadline": "2022-09-07T00:00:00",
          "after_time": 9,
          "before_time": 21 
        },
        {
          "id": 6,
          "title": "dentist",
          "duration": 1,
          "start": "2022-09-14T00:00:00",
          "deadline": "2022-09-15T00:00:00",
          "after_time": 10,
          "before_time": 11 
        },
        {
          "id": 7,
          "title": "shopping",
          "duration": 2,
          "start": "2022-09-10T00:00:00",
          "deadline": "2022-09-11T00:00:00",
          "after_time": 12,
          "before_time": 18 
        },
        {
          "id": 8,
          "title": "repair sink",
          "duration": 3,
          "start": "2022-09-25T00:00:00",
          "deadline": "2022-09-26T00:00:00"
        }
    ]
}
      ),
 [
      {
        "taskid": 60,
        "goalid": 6,
        "title": "dentist",
        "duration": 1,
        "start": "2022-09-14T10:00:00",
        "deadline": "2022-09-14T11:00:00"
      },
      {
        "taskid": 429,
        "goalid": 4,
        "title": "gym",
        "duration": 2,
        "start": "2022-09-30T05:00:00",
        "deadline": "2022-09-30T07:00:00"
      },
      {
        "taskid": 428,
        "goalid": 4,
        "title": "gym",
        "duration": 2,
        "start": "2022-09-29T05:00:00",
        "deadline": "2022-09-29T07:00:00"
      },
      {
        "taskid": 427,
        "goalid": 4,
        "title": "gym",
        "duration": 2,
        "start": "2022-09-28T05:00:00",
        "deadline": "2022-09-28T07:00:00"
      },
      {
        "taskid": 426,
        "goalid": 4,
        "title": "gym",
        "duration": 2,
        "start": "2022-09-27T05:00:00",
        "deadline": "2022-09-27T07:00:00"
      },
      {
        "taskid": 425,
        "goalid": 4,
        "title": "gym",
        "duration": 2,
        "start": "2022-09-26T05:00:00",
        "deadline": "2022-09-26T07:00:00"
      },
      {
        "taskid": 424,
        "goalid": 4,
        "title": "gym",
        "duration": 2,
        "start": "2022-09-25T05:00:00",
        "deadline": "2022-09-25T07:00:00"
      },
      {
        "taskid": 423,
        "goalid": 4,
        "title": "gym",
        "duration": 2,
        "start": "2022-09-24T05:00:00",
        "deadline": "2022-09-24T07:00:00"
      },
      {
        "taskid": 422,
        "goalid": 4,
        "title": "gym",
        "duration": 2,
        "start": "2022-09-23T05:00:00",
        "deadline": "2022-09-23T07:00:00"
      },
      {
        "taskid": 421,
        "goalid": 4,
        "title": "gym",
        "duration": 2,
        "start": "2022-09-22T05:00:00",
        "deadline": "2022-09-22T07:00:00"
      },
      {
        "taskid": 420,
        "goalid": 4,
        "title": "gym",
        "duration": 2,
        "start": "2022-09-21T05:00:00",
        "deadline": "2022-09-21T07:00:00"
      },
      {
        "taskid": 419,
        "goalid": 4,
        "title": "gym",
        "duration": 2,
        "start": "2022-09-20T05:00:00",
        "deadline": "2022-09-20T07:00:00"
      },
      {
        "taskid": 418,
        "goalid": 4,
        "title": "gym",
        "duration": 2,
        "start": "2022-09-19T05:00:00",
        "deadline": "2022-09-19T07:00:00"
      },
      {
        "taskid": 417,
        "goalid": 4,
        "title": "gym",
        "duration": 2,
        "start": "2022-09-18T05:00:00",
        "deadline": "2022-09-18T07:00:00"
      },
      {
        "taskid": 416,
        "goalid": 4,
        "title": "gym",
        "duration": 2,
        "start": "2022-09-17T05:00:00",
        "deadline": "2022-09-17T07:00:00"
      },
      {
        "taskid": 415,
        "goalid": 4,
        "title": "gym",
        "duration": 2,
        "start": "2022-09-16T05:00:00",
        "deadline": "2022-09-16T07:00:00"
      },
      {
        "taskid": 414,
        "goalid": 4,
        "title": "gym",
        "duration": 2,
        "start": "2022-09-15T05:00:00",
        "deadline": "2022-09-15T07:00:00"
      },
      {
        "taskid": 413,
        "goalid": 4,
        "title": "gym",
        "duration": 2,
        "start": "2022-09-14T05:00:00",
        "deadline": "2022-09-14T07:00:00"
      },
      {
        "taskid": 412,
        "goalid": 4,
        "title": "gym",
        "duration": 2,
        "start": "2022-09-13T05:00:00",
        "deadline": "2022-09-13T07:00:00"
      },
      {
        "taskid": 411,
        "goalid": 4,
        "title": "gym",
        "duration": 2,
        "start": "2022-09-12T05:00:00",
        "deadline": "2022-09-12T07:00:00"
      },
      {
        "taskid": 410,
        "goalid": 4,
        "title": "gym",
        "duration": 2,
        "start": "2022-09-11T05:00:00",
        "deadline": "2022-09-11T07:00:00"
      },
      {
        "taskid": 49, 
        "goalid": 4,
        "title": "gym",
        "duration": 2,
        "start": "2022-09-10T05:00:00",
        "deadline": "2022-09-10T07:00:00"
      },
      {
        "taskid": 48,
        "goalid": 4,
        "title": "gym",
        "duration": 2,
        "start": "2022-09-09T05:00:00",
        "deadline": "2022-09-09T07:00:00"
      },
      {
        "taskid": 47,
        "goalid": 4,
        "title": "gym",
        "duration": 2,
        "start": "2022-09-08T05:00:00",
        "deadline": "2022-09-08T07:00:00"
      },
      {
        "taskid": 46,
        "goalid": 4,
        "title": "gym",
        "duration": 2,
        "start": "2022-09-07T05:00:00",
        "deadline": "2022-09-07T07:00:00"
      },
      {
        "taskid": 45,
        "goalid": 4,
        "title": "gym",
        "duration": 2,
        "start": "2022-09-06T05:00:00",
        "deadline": "2022-09-06T07:00:00"
      },
      {
        "taskid": 44,
        "goalid": 4,
        "title": "gym",
        "duration": 2,
        "start": "2022-09-05T05:00:00",
        "deadline": "2022-09-05T07:00:00"
      },
      {
        "taskid": 43,
        "goalid": 4,
        "title": "gym",
        "duration": 2,
        "start": "2022-09-04T05:00:00",
        "deadline": "2022-09-04T07:00:00"
      },
      {
        "taskid": 42,
        "goalid": 4,
        "title": "gym",
        "duration": 2,
        "start": "2022-09-03T05:00:00",
        "deadline": "2022-09-03T07:00:00"
      },
      {
        "taskid": 41,
        "goalid": 4,
        "title": "gym",
        "duration": 2,
        "start": "2022-09-02T05:00:00",
        "deadline": "2022-09-02T07:00:00"
      },
      {
        "taskid": 40,
        "goalid": 4,
        "title": "gym",
        "duration": 2,
        "start": "2022-09-01T05:00:00",
        "deadline": "2022-09-01T07:00:00"
      },
      {
        "taskid": 33,
        "goalid": 3,
        "title": "piano practice",
        "duration": 2,
        "start": "2022-09-28T12:00:00",
        "deadline": "2022-09-28T14:00:00"
      },
      {
        "taskid": 32,
        "goalid": 3,
        "title": "piano practice",
        "duration": 2,
        "start": "2022-09-21T12:00:00",
        "deadline": "2022-09-21T14:00:00"
      },
      {
        "taskid": 31,
        "goalid": 3,
        "title": "piano practice",
        "duration": 2,
        "start": "2022-09-14T12:00:00",
        "deadline": "2022-09-14T14:00:00"
      },
      {
        "taskid": 30,
        "goalid": 3,
        "title": "piano practice",
        "duration": 2,
        "start": "2022-09-07T12:00:00",
        "deadline": "2022-09-07T14:00:00"
      },
      {
        "taskid": 80,
        "goalid": 8,
        "title": "repair sink",
        "duration": 3,
        "start": "2022-09-25T00:00:00",
        "deadline": "2022-09-25T03:00:00"
      },
      {
        "taskid": 70,
        "goalid": 7,
        "title": "shopping",
        "duration": 2,
        "start": "2022-09-10T12:00:00",
        "deadline": "2022-09-10T14:00:00"
      },
      {
        "taskid": 55,
        "goalid": 5,
        "title": "work",
        "duration": 8,
        "start": "2022-09-06T09:00:00",
        "deadline": "2022-09-06T17:00:00"
      },
      {
        "taskid": 54,
        "goalid": 5,
        "title": "work",
        "duration": 8,
        "start": "2022-09-05T09:00:00",
        "deadline": "2022-09-05T17:00:00"
      },
      {
        "taskid": 53,
        "goalid": 5,
        "title": "work",
        "duration": 8,
        "start": "2022-09-04T09:00:00",
        "deadline": "2022-09-04T17:00:00"
      },
      {
        "taskid": 52,
        "goalid": 5,
        "title": "work",
        "duration": 8,
        "start": "2022-09-03T09:00:00",
        "deadline": "2022-09-03T17:00:00"
      },
      {
        "taskid": 51,
        "goalid": 5,
        "title": "work",
        "duration": 8,
        "start": "2022-09-02T09:00:00",
        "deadline": "2022-09-02T17:00:00"
      },
      {
        "taskid": 50,
        "goalid": 5,
        "title": "work",
        "duration": 8,
        "start": "2022-09-01T09:00:00",
        "deadline": "2022-09-01T17:00:00"
      },
      {
        "taskid": 229,
        "goalid": 2,
        "title": "read",
        "duration": 1,
        "start": "2022-09-30T20:00:00",
        "deadline": "2022-09-30T21:00:00"
      },
      {
        "taskid": 228,
        "goalid": 2,
        "title": "read",
        "duration": 1,
        "start": "2022-09-29T20:00:00",
        "deadline": "2022-09-29T21:00:00"
      },
      {
        "taskid": 227,
        "goalid": 2,
        "title": "read",
        "duration": 1,
        "start": "2022-09-28T20:00:00",
        "deadline": "2022-09-28T21:00:00"
      },
      {
        "taskid": 226,
        "goalid": 2,
        "title": "read",
        "duration": 1,
        "start": "2022-09-27T20:00:00",
        "deadline": "2022-09-27T21:00:00"
      },
      {
        "taskid": 225,
        "goalid": 2,
        "title": "read",
        "duration": 1,
        "start": "2022-09-26T20:00:00",
        "deadline": "2022-09-26T21:00:00"
      },
      {
        "taskid": 224,
        "goalid": 2,
        "title": "read",
        "duration": 1,
        "start": "2022-09-25T20:00:00",
        "deadline": "2022-09-25T21:00:00"
      },
      {
        "taskid": 223,
        "goalid": 2,
        "title": "read",
        "duration": 1,
        "start": "2022-09-24T20:00:00",
        "deadline": "2022-09-24T21:00:00"
      },
      {
        "taskid": 222,
        "goalid": 2,
        "title": "read",
        "duration": 1,
        "start": "2022-09-23T20:00:00",
        "deadline": "2022-09-23T21:00:00"
      },
      {
        "taskid": 221,
        "goalid": 2,
        "title": "read",
        "duration": 1,
        "start": "2022-09-22T20:00:00",
        "deadline": "2022-09-22T21:00:00"
      },
      {
        "taskid": 220,
        "goalid": 2,
        "title": "read",
        "duration": 1,
        "start": "2022-09-21T20:00:00",
        "deadline": "2022-09-21T21:00:00"
      },
      {
        "taskid": 219,
        "goalid": 2,
        "title": "read",
        "duration": 1,
        "start": "2022-09-20T20:00:00",
        "deadline": "2022-09-20T21:00:00"
      },
      {
        "taskid": 218,
        "goalid": 2,
        "title": "read",
        "duration": 1,
        "start": "2022-09-19T20:00:00",
        "deadline": "2022-09-19T21:00:00"
      },
      {
        "taskid": 217,
        "goalid": 2,
        "title": "read",
        "duration": 1,
        "start": "2022-09-18T20:00:00",
        "deadline": "2022-09-18T21:00:00"
      },
      {
        "taskid": 216,
        "goalid": 2,
        "title": "read",
        "duration": 1,
        "start": "2022-09-17T20:00:00",
        "deadline": "2022-09-17T21:00:00"
      },
      {
        "taskid": 215,
        "goalid": 2,
        "title": "read",
        "duration": 1,
        "start": "2022-09-16T20:00:00",
        "deadline": "2022-09-16T21:00:00"
      },
      {
        "taskid": 214,
        "goalid": 2,
        "title": "read",
        "duration": 1,
        "start": "2022-09-15T20:00:00",
        "deadline": "2022-09-15T21:00:00"
      },
      {
        "taskid": 213,
        "goalid": 2,
        "title": "read",
        "duration": 1,
        "start": "2022-09-14T20:00:00",
        "deadline": "2022-09-14T21:00:00"
      },
      {
        "taskid": 212,
        "goalid": 2,
        "title": "read",
        "duration": 1,
        "start": "2022-09-13T20:00:00",
        "deadline": "2022-09-13T21:00:00"
      },
      {
        "taskid": 211,
        "goalid": 2,
        "title": "read",
        "duration": 1,
        "start": "2022-09-12T20:00:00",
        "deadline": "2022-09-12T21:00:00"
      },
      {
        "taskid": 210,
        "goalid": 2,
        "title": "read",
        "duration": 1,
        "start": "2022-09-11T20:00:00",
        "deadline": "2022-09-11T21:00:00"
      },
      {
        "taskid": 29,
        "goalid": 2,
        "title": "read",
        "duration": 1,
        "start": "2022-09-10T20:00:00",
        "deadline": "2022-09-10T21:00:00"
      },
      {
        "taskid": 28,
        "goalid": 2,
        "title": "read",
        "duration": 1,
        "start": "2022-09-09T20:00:00",
        "deadline": "2022-09-09T21:00:00"
      },
      {
        "taskid": 27,
        "goalid": 2,
        "title": "read",
        "duration": 1,
        "start": "2022-09-08T20:00:00",
        "deadline": "2022-09-08T21:00:00"
      },
      {
        "taskid": 26,
        "goalid": 2,
        "title": "read",
        "duration": 1,
        "start": "2022-09-07T20:00:00",
        "deadline": "2022-09-07T21:00:00"
      },
      {
        "taskid": 25,
        "goalid": 2,
        "title": "read",
        "duration": 1,
        "start": "2022-09-06T20:00:00",
        "deadline": "2022-09-06T21:00:00"
      },
      {
        "taskid": 24,
        "goalid": 2,
        "title": "read",
        "duration": 1,
        "start": "2022-09-05T20:00:00",
        "deadline": "2022-09-05T21:00:00"
      },
      {
        "taskid": 23,
        "goalid": 2,
        "title": "read",
        "duration": 1,
        "start": "2022-09-04T20:00:00",
        "deadline": "2022-09-04T21:00:00"
      },
      {
        "taskid": 22,
        "goalid": 2,
        "title": "read",
        "duration": 1,
        "start": "2022-09-03T20:00:00",
        "deadline": "2022-09-03T21:00:00"
      },
      {
        "taskid": 21,
        "goalid": 2,
        "title": "read",
        "duration": 1,
        "start": "2022-09-02T20:00:00",
        "deadline": "2022-09-02T21:00:00"
      },
      {
        "taskid": 20,
        "goalid": 2,
        "title": "read",
        "duration": 1,
        "start": "2022-09-01T20:00:00",
        "deadline": "2022-09-01T21:00:00"
      },
      {
        "taskid": 129,
        "goalid": 1,
        "title": "walk",
        "duration": 1,
        "start": "2022-09-30T17:00:00",
        "deadline": "2022-09-30T18:00:00"
      },
      {
        "taskid": 128,
        "goalid": 1,
        "title": "walk",
        "duration": 1,
        "start": "2022-09-29T17:00:00",
        "deadline": "2022-09-29T18:00:00"
      },
      {
        "taskid": 127,
        "goalid": 1,
        "title": "walk",
        "duration": 1,
        "start": "2022-09-28T17:00:00",
        "deadline": "2022-09-28T18:00:00"
      },
      {
        "taskid": 126,
        "goalid": 1,
        "title": "walk",
        "duration": 1,
        "start": "2022-09-27T17:00:00",
        "deadline": "2022-09-27T18:00:00"
      },
      {
        "taskid": 125,
        "goalid": 1,
        "title": "walk",
        "duration": 1,
        "start": "2022-09-26T17:00:00",
        "deadline": "2022-09-26T18:00:00"
      },
      {
        "taskid": 124,
        "goalid": 1,
        "title": "walk",
        "duration": 1,
        "start": "2022-09-25T17:00:00",
        "deadline": "2022-09-25T18:00:00"
      },
      {
        "taskid": 123,
        "goalid": 1,
        "title": "walk",
        "duration": 1,
        "start": "2022-09-24T17:00:00",
        "deadline": "2022-09-24T18:00:00"
      },
      {
        "taskid": 122,
        "goalid": 1,
        "title": "walk",
        "duration": 1,
        "start": "2022-09-23T17:00:00",
        "deadline": "2022-09-23T18:00:00"
      },
      {
        "taskid": 121,
        "goalid": 1,
        "title": "walk",
        "duration": 1,
        "start": "2022-09-22T17:00:00",
        "deadline": "2022-09-22T18:00:00"
      },
      {
        "taskid": 120,
        "goalid": 1,
        "title": "walk",
        "duration": 1,
        "start": "2022-09-21T17:00:00",
        "deadline": "2022-09-21T18:00:00"
      },
      {
        "taskid": 119,
        "goalid": 1,
        "title": "walk",
        "duration": 1,
        "start": "2022-09-20T17:00:00",
        "deadline": "2022-09-20T18:00:00"
      },
      {
        "taskid": 118,
        "goalid": 1,
        "title": "walk",
        "duration": 1,
        "start": "2022-09-19T17:00:00",
        "deadline": "2022-09-19T18:00:00"
      },
      {
        "taskid": 117,
        "goalid": 1,
        "title": "walk",
        "duration": 1,
        "start": "2022-09-18T17:00:00",
        "deadline": "2022-09-18T18:00:00"
      },
      {
        "taskid": 116,
        "goalid": 1,
        "title": "walk",
        "duration": 1,
        "start": "2022-09-17T17:00:00",
        "deadline": "2022-09-17T18:00:00"
      },
      {
        "taskid": 115,
        "goalid": 1,
        "title": "walk",
        "duration": 1,
        "start": "2022-09-16T17:00:00",
        "deadline": "2022-09-16T18:00:00"
      },
      {
        "taskid": 114,
        "goalid": 1,
        "title": "walk",
        "duration": 1,
        "start": "2022-09-15T17:00:00",
        "deadline": "2022-09-15T18:00:00"
      },
      {
        "taskid": 113,
        "goalid": 1,
        "title": "walk",
        "duration": 1,
        "start": "2022-09-14T17:00:00",
        "deadline": "2022-09-14T18:00:00"
      },
      {
        "taskid": 112,
        "goalid": 1,
        "title": "walk",
        "duration": 1,
        "start": "2022-09-13T17:00:00",
        "deadline": "2022-09-13T18:00:00"
      },
      {
        "taskid": 111,
        "goalid": 1,
        "title": "walk",
        "duration": 1,
        "start": "2022-09-12T17:00:00",
        "deadline": "2022-09-12T18:00:00"
      },
      {
        "taskid": 110,
        "goalid": 1,
        "title": "walk",
        "duration": 1,
        "start": "2022-09-11T17:00:00",
        "deadline": "2022-09-11T18:00:00"
      },
      {
        "taskid": 19,
        "goalid": 1,
        "title": "walk",
        "duration": 1,
        "start": "2022-09-10T17:00:00",
        "deadline": "2022-09-10T18:00:00"
      },
      {
        "taskid": 18,
        "goalid": 1,
        "title": "walk",
        "duration": 1,
        "start": "2022-09-09T17:00:00",
        "deadline": "2022-09-09T18:00:00"
      },
      {
        "taskid": 17,
        "goalid": 1,
        "title": "walk",
        "duration": 1,
        "start": "2022-09-08T17:00:00",
        "deadline": "2022-09-08T18:00:00"
      },
      {
        "taskid": 16,
        "goalid": 1,
        "title": "walk",
        "duration": 1,
        "start": "2022-09-07T17:00:00",
        "deadline": "2022-09-07T18:00:00"
      },
      {
        "taskid": 15,
        "goalid": 1,
        "title": "walk",
        "duration": 1,
        "start": "2022-09-06T17:00:00",
        "deadline": "2022-09-06T18:00:00"
      },
      {
        "taskid": 14,
        "goalid": 1,
        "title": "walk",
        "duration": 1,
        "start": "2022-09-05T17:00:00",
        "deadline": "2022-09-05T18:00:00"
      },
      {
        "taskid": 13,
        "goalid": 1,
        "title": "walk",
        "duration": 1,
        "start": "2022-09-04T17:00:00",
        "deadline": "2022-09-04T18:00:00"
      },
      {
        "taskid": 12,
        "goalid": 1,
        "title": "walk",
        "duration": 1,
        "start": "2022-09-03T17:00:00",
        "deadline": "2022-09-03T18:00:00"
      },
      {
        "taskid": 11,
        "goalid": 1,
        "title": "walk",
        "duration": 1,
        "start": "2022-09-02T17:00:00",
        "deadline": "2022-09-02T18:00:00"
      },
      {
        "taskid": 10,
        "goalid": 1,
        "title": "walk",
        "duration": 1,
        "start": "2022-09-01T17:00:00",
        "deadline": "2022-09-01T18:00:00"
      }
]         
  );
});
