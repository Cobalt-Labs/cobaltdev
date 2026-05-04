import 'package:flutter/material.dart';
import 'package:url_launcher/url_launcher.dart';
import 'package:http/http.dart' as http;
import 'dart:convert';
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
  String? errorMessage;
  String? successMessage;

  // physical device
  final String backendUrl = "http://192.168.x.x:8080/api/send-email";
  // android emulator
  // final String backendUrl = "http://10.0.2.2:8080/api/send-email"; 
  
  Future<void> sendMessage() async {
    if (nameController.text.trim().isEmpty ||
        emailController.text.trim().isEmpty ||
        messageController.text.trim().isEmpty) {
      setState(() => errorMessage = "Please fill all fields");
      return;
    }

    setState(() {
      isLoading = true;
      errorMessage = null;
      successMessage = null;
    });

    try {
      final response = await http.post(
        Uri.parse(backendUrl),
        headers: {"Content-Type": "application/json"},
        body: jsonEncode({
          "name": nameController.text.trim(),
          "email": emailController.text.trim(),
          "message": messageController.text.trim(),
        }),
      );

      if (response.statusCode == 200) {
        setState(() => successMessage = "Message sent successfully!");
        _showSuccessDialog();
        _clearFields();
      } else {
        setState(() => errorMessage = "Failed to send message. Try again.");
      }
    } catch (e) {
      setState(() => errorMessage = "Connection error. Please check your internet.");
      // Fallback to mailto
      await _tryMailtoFallback();
    }

    setState(() => isLoading = false);
  }

  Future<void> _tryMailtoFallback() async {
    final String subject = Uri.encodeComponent("CobaltDev Inquiry from ${nameController.text.trim()}");
    final String body = Uri.encodeComponent(
        "${messageController.text.trim()}\n\n---\nSender Email: ${emailController.text.trim()}"
    );

    final Uri emailUri = Uri.parse("mailto:ibrahim.haji.3595@gmail.com?subject=$subject&body=$body");

    if (await canLaunchUrl(emailUri)) {
      await launchUrl(emailUri);
    }
  }

  void _clearFields() {
    nameController.clear();
    emailController.clear();
    messageController.clear();
  }

  void _showSuccessDialog() {
    showDialog(
      context: context,
      builder: (_) => AlertDialog(
        backgroundColor: const Color(0xFF0B091C),
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(12),
          side: const BorderSide(color: Color(0xFF27272A)),
        ),
        title: const Text("Message Sent", style: TextStyle(color: Colors.white)),
        content: const Text(
          "Thank you! I'll get back to you soon.",
          style: TextStyle(color: Colors.white70),
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context),
            child: const Text("OK", style: TextStyle(color: Color(0xFFA855F7))),
          ),
        ],
      ),
    );
  }

  @override
  void dispose() {
    nameController.dispose();
    emailController.dispose();
    messageController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final width = MediaQuery.of(context).size.width;
    final isMobile = width < 700;

    return Center(
      child: AnimatedSection(
        child: SingleChildScrollView(
          padding: EdgeInsets.symmetric(horizontal: isMobile ? 24 : 48, vertical: 40),
          child: ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 600),
            child: GlassCard(
              padding: EdgeInsets.all(isMobile ? 32 : 48),
              child: Column(
                mainAxisSize: MainAxisSize.min,
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  const Text("Get In Touch", style: TextStyle(fontSize: 32, fontWeight: FontWeight.w600)),
                  const SizedBox(height: 8),
                  const Text(
                    "Have a project or idea? Let's build something great together.",
                    style: TextStyle(color: Colors.white70, fontSize: 16),
                  ),
                  const SizedBox(height: 32),

                  if (errorMessage != null)
                    Padding(
                      padding: const EdgeInsets.only(bottom: 16),
                      child: Text(errorMessage!, style: const TextStyle(color: Colors.redAccent)),
                    ),

                  if (successMessage != null)
                    Padding(
                      padding: const EdgeInsets.only(bottom: 16),
                      child: Text(successMessage!, style: const TextStyle(color: Colors.green)),
                    ),

                  TextField(controller: nameController, decoration: _inputStyle("Your Name")),
                  const SizedBox(height: 20),
                  TextField(
                    controller: emailController,
                    keyboardType: TextInputType.emailAddress,
                    decoration: _inputStyle("Your Email"),
                  ),
                  const SizedBox(height: 20),
                  TextField(
                    controller: messageController,
                    maxLines: 5,
                    decoration: _inputStyle("Your Message"),
                  ),

                  const SizedBox(height: 40),

                  SizedBox(
                    width: double.infinity,
                    height: 48,
                    child: ElevatedButton(
                      onPressed: isLoading ? null : sendMessage,
                      style: ElevatedButton.styleFrom(
                        backgroundColor: const Color(0xFF4F46E5),
                        foregroundColor: Colors.white,
                        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
                      ),
                      child: isLoading
                          ? const CircularProgressIndicator(color: Colors.white, strokeWidth: 2)
                          : const Text("Send Message", style: TextStyle(fontSize: 16, fontWeight: FontWeight.w500)),
                    ),
                  ),

                  const SizedBox(height: 40),

                  // === Additional Contact Methods ===
                  const Text("Other ways to reach me", style: TextStyle(color: Colors.white70, fontSize: 15)),
                  const SizedBox(height: 16),
                  Wrap(
                    spacing: 12,
                    runSpacing: 12,
                    children: [
                      _contactButton("WhatsApp", Icons.chat, Colors.green, () => _launchUrl("https://wa.me/919175909443")),
                      _contactButton("LinkedIn", Icons.link, const Color(0xFF0A66C2), () => _launchUrl("https://www.linkedin.com/in/ibrahim-haji-647836347/")),
                      _contactButton("GitHub", Icons.code, Colors.white, () => _launchUrl("https://github.com/ibrahim-3595")),
                      _contactButton("Email", Icons.email, Colors.red, () => _launchUrl("mailto:ibrahim.haji.3595@gmail.com")),
                    ],
                  ),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }

  Widget _contactButton(String label, IconData icon, Color color, VoidCallback onTap) {
    return InkWell(
      onTap: onTap,
      borderRadius: BorderRadius.circular(8),
      child: Container(
        padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
        decoration: BoxDecoration(
          color: Colors.white.withOpacity(0.05),
          borderRadius: BorderRadius.circular(8),
          border: Border.all(color: Colors.white24),
        ),
        child: Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            Icon(icon, color: color, size: 20),
            const SizedBox(width: 8),
            Text(label, style: const TextStyle(color: Colors.white)),
          ],
        ),
      ),
    );
  }

  Future<void> _launchUrl(String url) async {
    final uri = Uri.parse(url);
    if (await canLaunchUrl(uri)) {
      await launchUrl(uri, mode: LaunchMode.externalApplication);
    }
  }

  InputDecoration _inputStyle(String label) {
    return InputDecoration(
      labelText: label,
      labelStyle: const TextStyle(color: Colors.white54, fontSize: 14),
      border: OutlineInputBorder(borderRadius: BorderRadius.circular(8), borderSide: const BorderSide(color: Color(0xFF27272A))),
      enabledBorder: OutlineInputBorder(borderRadius: BorderRadius.circular(8), borderSide: const BorderSide(color: Color(0xFF27272A))),
      focusedBorder: OutlineInputBorder(borderRadius: BorderRadius.circular(8), borderSide: const BorderSide(color: Color(0xFFA855F7), width: 1.5)),
      filled: true,
      fillColor: Colors.white.withOpacity(0.025),
      contentPadding: const EdgeInsets.symmetric(horizontal: 16, vertical: 16),
    );
  }
}