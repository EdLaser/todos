import http from 'k6/http';
import { sleep } from 'k6';

export let options = {
  vus: 10,
  duration: '30s',
};

export default function () {
  // Test the list_todos_handler() endpoint
  http.get('http://web:8000/');
  sleep(1);

  // Test the list_todo_handler(todo_id) endpoint
  http.get('http://web:8000/todo/1'); // Replace '1' with an actual todo_id
  sleep(1);

  // Test the done_todo_handler(todo_id) endpoint
  http.get('http://web:8000/done/1'); // Replace '1' with an actual todo_id
  sleep(1);

  // Test the create_todo_handler(todo) endpoint
  const todo = {
    // Replace with actual JSON data for a new todo
  };
  http.post('http://web:8000/new_todo', JSON.stringify(todo), {
    headers: {
      'Content-Type': 'application/json',
    },
  });
  sleep(1);

  // Test the delete_todo_handler(todo_id) endpoint
  http.get('http://web:8000/delete/1'); // Replace '1' with an actual todo_id
  sleep(1);
}
