import 'package:flutter/material.dart';
import 'dart:convert';
import 'package:http/http.dart' as http;
import '../widgets/animated_section.dart';
import '../widgets/glass_card.dart';

class ContactPage extends StatefulWidget {
  const ContactPage({super.key});

  @override
  State<ContactPage> createState() => _ContactPageState();
}

class _ContactPageState extends State<ContactPage> {
  final nameController = TextEditingController();
  final emailController = TextEditingController();
  final messageController = TextEditingController();

  bool isLoading = false;

  Future<void> sendMessage() async {
    if (nameController.text.isEmpty ||
        emailController.text.isEmpty ||
        messageController.text.isEmpty) {
      _showDialog("Error", "Please fill all fields");
      return;
    }

    setState(() => isLoading = true);

    try {
      final response = await http.post(
        Uri.parse("http://localhost:3000/contact"),
        headers: {"Content-Type": "application/json"},
        body: jsonEncode({
          "name": nameController.text,
          "email": emailController.text,
          "message": messageController.text,
        }),
      );

      if (response.statusCode == 200) {
        _showDialog("Success 🚀", "Email sent successfully!");

        nameController.clear();
        emailController.clear();
        messageController.clear();
      } else {
        _showDialog("Error", "Failed to send message");
      }
    } catch (e) {
      _showDialog("Error", "Server not reachable");
    }

    setState(() => isLoading = false);
  }

  void _showDialog(String title, String message) {
    showDialog(
      context: context,
      builder: (_) => AlertDialog(
        title: Text(title),
        content: Text(message),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context),
            child: Text("OK"),
          )
        ],
      ),
    );
  }

  InputDecoration _inputStyle(String label) {
    return InputDecoration(
      labelText: label,
      border: OutlineInputBorder(
        borderRadius: BorderRadius.circular(10),
      ),
      focusedBorder: OutlineInputBorder(
        borderRadius: BorderRadius.circular(10),
        borderSide: BorderSide(color: Colors.blue),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final width = MediaQuery.of(context).size.width;
    final isDesktop = width > 800;

    return Scaffold(
      body: Center(
        child: AnimatedSection(
          child: ConstrainedBox(
            constraints: BoxConstraints(maxWidth: 600),
            child: Padding(
              padding: const EdgeInsets.all(40),
              child: GlassCard(
                child: Column(
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    /// 🔥 TITLE
                    Text(
                      "Contact Me",
                      style: TextStyle(
                        fontSize: isDesktop ? 36 : 28,
                        fontWeight: FontWeight.bold,
                      ),
                    ),

                    const SizedBox(height: 30),

                    /// 🧾 NAME
                    TextField(
                      controller: nameController,
                      decoration: _inputStyle("Name"),
                    ),

                    const SizedBox(height: 15),

                    /// 📧 EMAIL
                    TextField(
                      controller: emailController,
                      decoration: _inputStyle("Email"),
                    ),

                    const SizedBox(height: 15),

                    /// 💬 MESSAGE
                    TextField(
                      controller: messageController,
                      maxLines: 5,
                      decoration: _inputStyle("Message"),
                    ),

                    const SizedBox(height: 25),

                    /// 🚀 BUTTON
                    SizedBox(
                      width: double.infinity,
                      child: ElevatedButton(
                        onPressed: isLoading ? null : sendMessage,
                        style: ElevatedButton.styleFrom(
                          padding: const EdgeInsets.symmetric(vertical: 15),
                        ),
                        child: isLoading
                            ? const CircularProgressIndicator()
                            : const Text("Send Message"),
                      ),
                    ),
                  ],
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }
}