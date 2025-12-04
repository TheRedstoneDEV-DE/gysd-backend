fetch("http://127.0.0.1:8000/api/mission/mission", {
  method: "POST",
  body: JSON.stringify({
    id: 0,
    name: "Cleanup room",
		priority: 5,
		time: Date.now(),
		duration: 45,
		repeat: 7,
    is_preset: false
  }),
  headers: {
    "Content-type": "application/json; charset=UTF-8"
  }
});
