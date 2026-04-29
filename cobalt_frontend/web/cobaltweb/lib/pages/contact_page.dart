import 'package:flutter/material.dart';
import 'package:url_launcher/url_launcher.dart';
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
    });

    final String subject = Uri.encodeComponent("CobaltDev Inquiry from ${nameController.text.trim()}");
    final String body = Uri.encodeComponent(
        "${messageController.text.trim()}\n\n---\nSender Email: ${emailController.text.trim()}"
    );
    
    final Uri emailUri = Uri.parse("mailto:ibrahimhaji3595@gmail.com?subject=$subject&body=$body");

    try {
      if (await canLaunchUrl(emailUri)) {
        await launchUrl(emailUri);
        _showSuccessDialog();
        nameController.clear();
        emailController.clear();
        messageController.clear();
      } else {
        setState(() => errorMessage = "Could not open default email client.");
      }
    } catch (e) {
      setState(() => errorMessage = "Error opening email client: $e");
    }

    setState(() => isLoading = false);
  }

  void _showSuccessDialog() {
    showDialog(
      context: context,
      builder: (_) => AlertDialog(
        backgroundColor: const Color(0xFF18181B), // zinc-900
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12), side: const BorderSide(color: Color(0xFF27272A))), // border-zinc-800
        title: const Text("Message Sent", style: TextStyle(color: Colors.white, fontWeight: FontWeight.w600)),
        content: const Text(
          "Thank you! I'll get back to you as soon as possible.",
          style: TextStyle(color: Colors.white70, fontSize: 15),
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context),
            child: const Text("OK", style: TextStyle(color: Color(0xFF6366F1), fontWeight: FontWeight.w500)), // Indigo
          ),
        ],
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final width = MediaQuery.of(context).size.width;
    final isMobile = width < 700;

    return Scaffold(
      backgroundColor: const Color(0xFF18181B),
      body: Center(
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
                    const Text(
                      "Get In Touch",
                      style: TextStyle(fontSize: 32, fontWeight: FontWeight.w600, letterSpacing: -0.5),
                    ),
                    const SizedBox(height: 8),
                    const Text(
                      "Have a project or idea? Let's build something great together.",
                      style: TextStyle(color: Colors.white70, fontSize: 16),
                    ),

                    const SizedBox(height: 40),

                    if (errorMessage != null)
                      Padding(
                        padding: const EdgeInsets.only(bottom: 24),
                        child: Text(
                          errorMessage!,
                          style: const TextStyle(color: Colors.redAccent, fontSize: 14),
                        ),
                      ),

                    TextField(
                      controller: nameController,
                      decoration: _inputStyle("Your Name"),
                      style: const TextStyle(fontSize: 15),
                    ),
                    const SizedBox(height: 20),

                    TextField(
                      controller: emailController,
                      keyboardType: TextInputType.emailAddress,
                      decoration: _inputStyle("Your Email"),
                      style: const TextStyle(fontSize: 15),
                    ),
                    const SizedBox(height: 20),

                    TextField(
                      controller: messageController,
                      maxLines: 5,
                      decoration: _inputStyle("Your Message"),
                      style: const TextStyle(fontSize: 15),
                    ),

                    const SizedBox(height: 40),

                    SizedBox(
                      width: double.infinity,
                      height: 48,
                      child: ElevatedButton(
                        onPressed: isLoading ? null : sendMessage,
                        style: ElevatedButton.styleFrom(
                          backgroundColor: const Color(0xFF4F46E5), // Indigo 600
                          foregroundColor: Colors.white,
                          shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
                          elevation: 0,
                        ),
                        child: isLoading
                            ? const SizedBox(height: 20, width: 20, child: CircularProgressIndicator(color: Colors.white, strokeWidth: 2))
                            : const Text("Send Message", style: TextStyle(fontSize: 15, fontWeight: FontWeight.w500)),
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

  InputDecoration _inputStyle(String label) {
    return InputDecoration(
      labelText: label,
      labelStyle: const TextStyle(color: Colors.white54, fontSize: 14),
      border: OutlineInputBorder(
        borderRadius: BorderRadius.circular(8),
        borderSide: const BorderSide(color: Color(0xFF27272A)), // zinc-800
      ),
      enabledBorder: OutlineInputBorder(
        borderRadius: BorderRadius.circular(8),
        borderSide: const BorderSide(color: Color(0xFF27272A)),
      ),
      focusedBorder: OutlineInputBorder(
        borderRadius: BorderRadius.circular(8),
        borderSide: const BorderSide(color: Color(0xFF6366F1), width: 1), // Indigo
      ),
      filled: true,
      fillColor: const Color(0xFF18181B), // zinc-900
      contentPadding: const EdgeInsets.symmetric(horizontal: 16, vertical: 16),
    );
  }
}