import http from 'k6/http';
import { sleep } from 'k6';

export let options = {
  stages: [
    { duration: '1m', target: 100 }, // ramp up to 100 users
    { duration: '3m', target: 100 }, // stay at 100 users
    { duration: '1m', target: 0 },   // scale down. (optional)
  ],
};

export default function () {
  let baseUrl = "http://web:8000"; // Replace with your actual base URL

  // Test GET list of todos
  let listTodosRes = http.get(`${baseUrl}/`);
  check(listTodosRes, { 'status was 200 for list todos': (r) => r.status == 200 });

  // Test POST new todo
  let newTodoData = JSON.stringify({ title: 'New Todo', description: 'Test description' });
  let createRes = http.post(`${baseUrl}/new_todo`, newTodoData, { headers: { 'Content-Type': 'application/json' } });
  check(createRes, { 'status was 201 for new todo': (r) => r.status == 201 });

  // Assume we have a todo with ID 1 to read, mark as done, and delete.
  let todoId = 1;

  // Test GET single todo
  let listSingleTodoRes = http.get(`${baseUrl}/todo/${todoId}`);
  check(listSingleTodoRes, { 'status was 200 for single todo': (r) => r.status == 200 });

  // Test GET mark todo as done
  let markDoneRes = http.get(`${baseUrl}/done/${todoId}`);
  check(markDoneRes, { 'status was 200 for mark done': (r) => r.status == 200 });

  // Test GET delete todo
  let deleteRes = http.get(`${baseUrl}/delete/${todoId}`);
  check(deleteRes, { 'status was 200 for delete': (r) => r.status == 200 });

  sleep(1);  // Sleep for 1 second (simulate think time)
}
