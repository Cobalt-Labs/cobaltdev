import 'package:flutter/material.dart';
import 'package:url_launcher/url_launcher.dart';
import 'package:flutter_dropzone/flutter_dropzone.dart';
import '../widgets/animated_section.dart';
import '../widgets/glass_card.dart';

class CloudPage extends StatefulWidget {
  const CloudPage({super.key});

  @override
  State<CloudPage> createState() => _CloudPageState();
}

class _CloudPageState extends State<CloudPage> {
  late DropzoneViewController controller;
  int dropCount = 0;
  bool isHighlighted = false;
  final int dropLimit = 3;
  List<String> uploadedFiles = [];

  Future<void> _openGithub() async {
    final url = Uri.parse("https://github.com/ibrahim-3595/cobaltdev/tree/main/cobalt_cloud");
    if (await canLaunchUrl(url)) {
      await launchUrl(url, mode: LaunchMode.externalApplication);
    }
  }

  void _onDrop(dynamic file) async {
    if (dropCount >= dropLimit) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(
          content: Text("Limit reached: You can only drop 3 files in this demo."),
          backgroundColor: Colors.redAccent,
        ),
      );
      return;
    }

    final name = await controller.getFilename(file);
    setState(() {
      dropCount++;
      uploadedFiles.add(name);
      isHighlighted = false;
    });

    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text("Uploaded $name successfully! ($dropCount/$dropLimit)"),
        backgroundColor: const Color(0xFF10B981),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final width = MediaQuery.of(context).size.width;
    final isDesktop = width > 900;
    final isMobile = width < 700;

    return Scaffold(
      body: SingleChildScrollView(
        child: Padding(
          padding: EdgeInsets.symmetric(horizontal: isMobile ? 20 : 40, vertical: isMobile ? 40 : 80),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              AnimatedSection(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      "Cobalt Cloud ☁️",
                      style: TextStyle(
                        fontSize: isDesktop ? 62 : 48,
                        fontWeight: FontWeight.bold,
                        color: const Color(0xFF10B981),
                      ),
                    ),
                    const SizedBox(height: 16),
                    const Text(
                      "A self-hosted private cloud infrastructure built with Rust & Dioxus.",
                      style: TextStyle(fontSize: 22, color: Colors.white70),
                    ),
                    const SizedBox(height: 32),
                    ElevatedButton.icon(
                      onPressed: _openGithub,
                      icon: const Icon(Icons.code),
                      label: const Text("View on GitHub"),
                      style: ElevatedButton.styleFrom(
                        backgroundColor: const Color(0xFF10B981),
                        foregroundColor: Colors.black,
                        padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 16),
                      ),
                    ),
                  ],
                ),
              ),

              const SizedBox(height: 60),

              // DROP ZONE SECTION
              AnimatedSection(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    const Text(
                      "Try it out (Demo)",
                      style: TextStyle(fontSize: 28, fontWeight: FontWeight.bold),
                    ),
                    const SizedBox(height: 8),
                    Text(
                      "Drag and drop files here. Limit: $dropLimit files.",
                      style: const TextStyle(color: Colors.white54),
                    ),
                    const SizedBox(height: 24),
                    Stack(
                      children: [
                        SizedBox(
                          height: 250,
                          child: DropzoneView(
                            operation: DragOperation.copy,
                            cursor: CursorType.Default,
                            onCreated: (ctrl) => controller = ctrl,
                            onDrop: _onDrop,
                            onHover: () => setState(() => isHighlighted = true),
                            onLeave: () => setState(() => isHighlighted = false),
                          ),
                        ),
                        IgnorePointer(
                          child: Container(
                            height: 250,
                            decoration: BoxDecoration(
                              color: isHighlighted 
                                ? const Color(0xFF10B981).withOpacity(0.1) 
                                : Colors.white.withOpacity(0.03),
                              borderRadius: BorderRadius.circular(24),
                              border: Border.all(
                                color: isHighlighted ? const Color(0xFF10B981) : Colors.white10,
                                width: 2,
                                style: BorderStyle.solid,
                              ),
                            ),
                            child: Center(
                              child: Column(
                                mainAxisAlignment: MainAxisAlignment.center,
                                children: [
                                  Icon(
                                    Icons.cloud_upload_outlined,
                                    size: 64,
                                    color: isHighlighted ? const Color(0xFF10B981) : Colors.white24,
                                  ),
                                  const SizedBox(height: 16),
                                  Text(
                                    isHighlighted ? "Drop it!" : "Drag files here",
                                    style: TextStyle(
                                      fontSize: 20,
                                      color: isHighlighted ? const Color(0xFF10B981) : Colors.white38,
                                    ),
                                  ),
                                  if (dropCount > 0) ...[
                                    const SizedBox(height: 12),
                                    Text(
                                      "$dropCount / $dropLimit uploaded",
                                      style: const TextStyle(color: Color(0xFF10B981), fontWeight: FontWeight.bold),
                                    ),
                                  ]
                                ],
                              ),
                            ),
                          ),
                        ),
                      ],
                    ),
                    if (uploadedFiles.isNotEmpty) ...[
                      const SizedBox(height: 20),
                      Wrap(
                        spacing: 10,
                        children: uploadedFiles.map((f) => Chip(
                          label: Text(f),
                          backgroundColor: Colors.white10,
                          labelStyle: const TextStyle(color: Colors.white70),
                        )).toList(),
                      ),
                    ]
                  ],
                ),
              ),

              const SizedBox(height: 80),

              Wrap(
                spacing: 30,
                runSpacing: 40,
                children: [
                  _featureCard(context,
                    "Frontend (Dioxus)",
                    "A cross-platform Rust frontend framework used to create the client-side app. Enables smooth drag & drop folder uploading.",
                    "💻",
                  ),
                  _featureCard(context,
                    "Backend (Axum)",
                    "High performance backend API running in Rust using Axum and object_store to handle file streaming and chunked uploads.",
                    "🦀",
                  ),
                  _featureCard(context,
                    "Self-Hosted",
                    "Designed to run on your own hardware. Your data stays on your local hard drive, giving you full control and privacy.",
                    "🔒",
                  ),
                  _featureCard(context,
                    "Desktop & Web",
                    "Dioxus compiles effortlessly into blazingly fast desktop and web applications native to your system.",
                    "⚡",
                  ),
                ],
              ),
              
              const SizedBox(height: 120),

              Center(
                child: GlassCard(
                  child: Padding(
                    padding: EdgeInsets.all(isMobile ? 30 : 40),
                    child: Column(
                      children: [
                         const Text(
                          "Ready to take back your data?",
                          style: TextStyle(fontSize: 24, fontWeight: FontWeight.bold),
                        ),
                        const SizedBox(height: 16),
                        const Text(
                          "Check out the project structure in the repository. Star ⭐ the project if you find it helpful!",
                          style: TextStyle(fontSize: 16, color: Colors.white70),
                          textAlign: TextAlign.center,
                        ),
                        const SizedBox(height: 24),
                        OutlinedButton(
                          onPressed: _openGithub,
                          style: OutlinedButton.styleFrom(
                            foregroundColor: const Color(0xFF10B981),
                            side: const BorderSide(color: Color(0xFF10B981)),
                            padding: const EdgeInsets.symmetric(horizontal: 32, vertical: 16),
                          ),
                          child: const Text("Deploy Cobalt Cloud"),
                        ),
                      ],
                    ),
                  ),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }

  Widget _featureCard(BuildContext context, String title, String desc, String emoji) {
    final sw = MediaQuery.of(context).size.width;
    return SizedBox(
      width: sw < 450 ? sw - 40 : 380,
      child: GlassCard(
        child: Padding(
          padding: const EdgeInsets.all(24),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(emoji, style: const TextStyle(fontSize: 42)),
              const SizedBox(height: 16),
              Text(title, style: const TextStyle(fontSize: 22, fontWeight: FontWeight.bold)),
              const SizedBox(height: 12),
              Text(desc, style: const TextStyle(color: Colors.white70, height: 1.5)),
            ],
          ),
        ),
      ),
    );
  }
}
