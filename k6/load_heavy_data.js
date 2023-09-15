import http from 'k6/http';
import { sleep, check } from 'k6';

// Sample data for new todos
const newTodos = [
    { title: 'Buy groceries', description: 'Milk, Cheese, Pizza, Fruit, Tylenol' },
    { title: 'Go to the gym', description: 'Leg day workout' },
    { title: 'Read a book', description: '1984 by George Orwell' },
    { title: 'Buy groceries', description: 'Milk, Cheese, Pizza, Fruit, Tylenol' },
    { title: 'Go to the gym', description: 'Leg day workout' },
    { title: 'Read a book', description: '1984 by George Orwell' },
    { title: 'Buy groceries', description: 'Milk, Cheese, Pizza, Fruit, Tylenol' },
    { title: 'Go to the gym', description: 'Leg day workout' },
    { title: 'Read a book', description: '1984 by George Orwell' }
];

export let options = {
    stages: [
        { duration: '1m', target: 100 }
    ],
};

export default function () {
    let baseUrl = "http://web:8000"; // Replace with your actual base URL

    // Test GET list of todos
    let listTodosRes = http.get(`${baseUrl}/`);
    check(listTodosRes, { 'status was 200 for list todos': (r) => r.status == 200 });

    // Test POST new todo using parameterized data
    let todoData = newTodos[Math.floor(Math.random() * newTodos.length)];
    let createRes = http.post(`${baseUrl}/new_todo`, JSON.stringify(todoData), { headers: { 'Content-Type': 'application/json' } });
    check(createRes, { 'status was 201 for new todo': (r) => r.status == 201 });

    // Here, I'm assuming that the application returns the created Todo's ID. 
    // Replace this with the actual ID extraction logic
    let createdTodoId = JSON.parse(createRes.body).id || 1;

    // Test GET single todo
    let listSingleTodoRes = http.get(`${baseUrl}/todo/${createdTodoId}`);
    check(listSingleTodoRes, { 'status was 200 for single todo': (r) => r.status == 200 });

    // Test GET mark todo as done
    let markDoneRes = http.get(`${baseUrl}/done/${createdTodoId}`);
    check(markDoneRes, { 'status was 200 for mark done': (r) => r.status == 200 });

    // Test GET delete todo
    let deleteRes = http.get(`${baseUrl}/delete/${createdTodoId}`);
    check(deleteRes, { 'status was 200 for delete': (r) => r.status == 200 });

    sleep(1);  // Sleep for 1 second (simulate think time)
}
