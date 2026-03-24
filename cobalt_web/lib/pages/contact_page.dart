import 'package:flutter/material.dart';
import 'dart:convert';
import 'package:http/http.dart' as http;
import '../widgets/animated_section.dart';

class ContactPage extends StatelessWidget {
  const ContactPage({super.key});

  @override
  Widget build(BuildContext context) {
    return AnimatedSection(
      child: Padding(
        padding: const EdgeInsets.all(40),
        child: Column(
          children: [
            const Text("Contact Me", style: TextStyle(fontSize: 30)),
            const SizedBox(height: 20),

            TextField(decoration: InputDecoration(labelText: "Name")),
            TextField(decoration: InputDecoration(labelText: "Email")),
            TextField(
              decoration: InputDecoration(labelText: "Message"),
              maxLines: 5,
            ),

            const SizedBox(height: 20),
            ElevatedButton(
              onPressed: () async {
                final response = await http.post(
                  // Uri.parse("http://127.0.0.1:3000/contact"),
                  Uri.parse("http://localhost:3000/contact"),
                  headers: {"Content-Type": "application/json"},
                  body: jsonEncode({
                    "name": "Test",
                    "email": "test@email.com",
                    "message": "Hello from Flutter",
                  }),
                );

                print(response.body);
              },
              child: const Text("Send Message"),
            ),
          ],
        ),
      ),
    );
  }
}
