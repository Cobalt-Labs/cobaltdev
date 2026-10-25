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
  final _nameController = TextEditingController();
  final _emailController = TextEditingController();
  final _messageController = TextEditingController();

  bool _isLoading = false;
  String? _errorMessage;
  String? _successMessage;

  @override
  void dispose() {
    _nameController.dispose();
    _emailController.dispose();
    _messageController.dispose();
    super.dispose();
  }

  Future<void> _sendMessage() async {
    final name = _nameController.text.trim();
    final email = _emailController.text.trim();
    final message = _messageController.text.trim();

    if (name.isEmpty || email.isEmpty || message.isEmpty) {
      setState(() => _errorMessage = "Please fill in all fields.");
      return;
    }

    // Simple email format check
    if (!RegExp(r'^[\w.-]+@[\w.-]+\.\w+$').hasMatch(email)) {
      setState(() => _errorMessage = "Please enter a valid email address.");
      return;
    }

    setState(() {
      _isLoading = true;
      _errorMessage = null;
      _successMessage = null;
    });

    try {
      final subject = Uri.encodeComponent("CobaltDev Inquiry from $name");
      final body = Uri.encodeComponent(
        "$message\n\n---\nSender: $name\nEmail: $email",
      );
      final emailUri = Uri.parse(
        "mailto:ibrahim.haji.3595@gmail.com?subject=$subject&body=$body",
      );

      await launchUrl(emailUri, mode: LaunchMode.externalApplication);

      if (mounted) {
        setState(() => _successMessage = "Opening your email client...");
        _clearFields();
        _showSuccessDialog();
      }
    } catch (e) {
      if (mounted) {
        setState(
          () => _errorMessage =
              "Could not open email client. Use the links below to reach me.",
        );
      }
    } finally {
      if (mounted) setState(() => _isLoading = false);
    }
  }

  void _clearFields() {
    _nameController.clear();
    _emailController.clear();
    _messageController.clear();
  }

  void _showSuccessDialog() {
    if (!mounted) return;
    showDialog(
      context: context,
      builder: (_) => AlertDialog(
        backgroundColor: const Color(0xFF0F0B22),
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(16),
          side: const BorderSide(color: Color(0x44A855F7)),
        ),
        title: const Row(
          children: [
            Icon(Icons.check_circle_rounded, color: Color(0xFF4ADE80), size: 22),
            SizedBox(width: 10),
            Text("Message Sent!", style: TextStyle(color: Colors.white)),
          ],
        ),
        content: const Text(
          "Thank you! I'll get back to you as soon as possible.",
          style: TextStyle(color: Colors.white70),
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context),
            child: const Text(
              "OK",
              style: TextStyle(color: Color(0xFFA855F7)),
            ),
          ),
        ],
      ),
    );
  }

  Future<void> _launchUrl(String url) async {
    final uri = Uri.parse(url);
    try {
      await launchUrl(uri, mode: LaunchMode.externalApplication);
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text("Could not open link. Try: $url"),
            backgroundColor: const Color(0xFF1E1A36),
          ),
        );
      }
    }
  }

  InputDecoration _inputStyle(String label) {
    return InputDecoration(
      labelText: label,
      labelStyle: const TextStyle(color: Colors.white54, fontSize: 14),
      border: OutlineInputBorder(
        borderRadius: BorderRadius.circular(10),
        borderSide: const BorderSide(color: Color(0xFF2D2A4A)),
      ),
      enabledBorder: OutlineInputBorder(
        borderRadius: BorderRadius.circular(10),
        borderSide: const BorderSide(color: Color(0xFF2D2A4A)),
      ),
      focusedBorder: OutlineInputBorder(
        borderRadius: BorderRadius.circular(10),
        borderSide: const BorderSide(color: Color(0xFFA855F7), width: 1.5),
      ),
      filled: true,
      fillColor: const Color(0xFF0D0B1E),
      contentPadding: const EdgeInsets.symmetric(horizontal: 16, vertical: 16),
    );
  }

  @override
  Widget build(BuildContext context) {
    final width = MediaQuery.of(context).size.width;
    final isMobile = width < 700;

    return SingleChildScrollView(
      physics: const ClampingScrollPhysics(),
      child: Center(
        child: AnimatedSection(
          child: Padding(
            padding: EdgeInsets.symmetric(
              horizontal: isMobile ? 24 : 48,
              vertical: 40,
            ),
            child: ConstrainedBox(
              constraints: const BoxConstraints(maxWidth: 600),
              child: GlassCard(
                padding: EdgeInsets.all(isMobile ? 28 : 48),
                child: Column(
                  mainAxisSize: MainAxisSize.min,
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    // Header
                    const Text(
                      "Get In Touch",
                      style: TextStyle(
                        fontSize: 32,
                        fontWeight: FontWeight.w700,
                        letterSpacing: -0.5,
                      ),
                    ),
                    const SizedBox(height: 8),
                    const Text(
                      "Have a project or idea? Let's build something great together.",
                      style: TextStyle(color: Colors.white70, fontSize: 16),
                    ),
                    const SizedBox(height: 36),

                    // Error / Success banners
                    if (_errorMessage != null)
                      _banner(
                        _errorMessage!,
                        const Color(0xFFEF4444),
                        Icons.error_outline_rounded,
                      ),
                    if (_successMessage != null)
                      _banner(
                        _successMessage!,
                        const Color(0xFF4ADE80),
                        Icons.check_circle_outline_rounded,
                      ),

                    // Form fields
                    TextField(
                      controller: _nameController,
                      decoration: _inputStyle("Your Name"),
                      style: const TextStyle(color: Colors.white),
                    ),
                    const SizedBox(height: 16),
                    TextField(
                      controller: _emailController,
                      keyboardType: TextInputType.emailAddress,
                      decoration: _inputStyle("Your Email"),
                      style: const TextStyle(color: Colors.white),
                    ),
                    const SizedBox(height: 16),
                    TextField(
                      controller: _messageController,
                      maxLines: 5,
                      decoration: _inputStyle("Your Message"),
                      style: const TextStyle(color: Colors.white),
                    ),

                    const SizedBox(height: 32),

                    // Submit Button
                    SizedBox(
                      width: double.infinity,
                      height: 52,
                      child: MouseRegion(
                        cursor: SystemMouseCursors.click,
                        child: GestureDetector(
                          onTap: _isLoading ? null : _sendMessage,
                          child: AnimatedContainer(
                            duration: const Duration(milliseconds: 200),
                            decoration: BoxDecoration(
                              gradient: _isLoading
                                  ? null
                                  : const LinearGradient(
                                      colors: [
                                        Color(0xFF9333EA),
                                        Color(0xFF7C3AED),
                                      ],
                                    ),
                              color: _isLoading
                                  ? const Color(0xFF2D2A4A)
                                  : null,
                              borderRadius: BorderRadius.circular(12),
                              boxShadow: _isLoading
                                  ? []
                                  : [
                                      BoxShadow(
                                        color: const Color(0xFF9333EA)
                                            .withValues(alpha: 0.4),
                                        blurRadius: 20,
                                        offset: const Offset(0, 4),
                                      ),
                                    ],
                            ),
                            child: Center(
                              child: _isLoading
                                  ? const SizedBox(
                                      width: 22,
                                      height: 22,
                                      child: CircularProgressIndicator(
                                        color: Colors.white54,
                                        strokeWidth: 2,
                                      ),
                                    )
                                  : const Text(
                                      "Send Message",
                                      style: TextStyle(
                                        fontSize: 16,
                                        fontWeight: FontWeight.w600,
                                        color: Colors.white,
                                      ),
                                    ),
                            ),
                          ),
                        ),
                      ),
                    ),

                    const SizedBox(height: 40),

                    // Divider
                    const Divider(color: Color(0x22FFFFFF)),

                    const SizedBox(height: 24),

                    // Other ways to reach
                    const Text(
                      "Other ways to reach me",
                      style: TextStyle(color: Colors.white60, fontSize: 14),
                    ),
                    const SizedBox(height: 16),
                    Wrap(
                      spacing: 12,
                      runSpacing: 12,
                      children: [
                        _contactButton(
                          "WhatsApp",
                          Icons.chat_rounded,
                          const Color(0xFF25D366),
                          () => _launchUrl("https://wa.me/919175909443"),
                        ),
                        _contactButton(
                          "LinkedIn",
                          Icons.link_rounded,
                          const Color(0xFF0A66C2),
                          () => _launchUrl(
                            "https://www.linkedin.com/in/ibrahim-haji-647836347/",
                          ),
                        ),
                        _contactButton(
                          "GitHub",
                          Icons.code_rounded,
                          Colors.white,
                          () => _launchUrl("https://github.com/ibrahim-3595"),
                        ),
                        _contactButton(
                          "Email",
                          Icons.email_rounded,
                          const Color(0xFFEF4444),
                          () => _launchUrl(
                            "mailto:ibrahim.haji.3595@gmail.com",
                          ),
                        ),
                      ],
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

  Widget _banner(String message, Color color, IconData icon) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 20),
      child: Container(
        padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
        decoration: BoxDecoration(
          color: color.withValues(alpha: 0.1),
          borderRadius: BorderRadius.circular(10),
          border: Border.all(color: color.withValues(alpha: 0.3)),
        ),
        child: Row(
          children: [
            Icon(icon, color: color, size: 18),
            const SizedBox(width: 10),
            Expanded(
              child: Text(
                message,
                style: TextStyle(color: color, fontSize: 14),
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _contactButton(
    String label,
    IconData icon,
    Color color,
    VoidCallback onTap,
  ) {
    return InkWell(
      onTap: onTap,
      borderRadius: BorderRadius.circular(10),
      child: Container(
        padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
        decoration: BoxDecoration(
          color: color.withValues(alpha: 0.07),
          borderRadius: BorderRadius.circular(10),
          border: Border.all(color: color.withValues(alpha: 0.2)),
        ),
        child: Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            Icon(icon, color: color, size: 18),
            const SizedBox(width: 8),
            Text(
              label,
              style: const TextStyle(color: Colors.white, fontSize: 14),
            ),
          ],
        ),
      ),
    );
  }
}