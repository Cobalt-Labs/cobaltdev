import 'package:flutter/material.dart';
import 'package:video_player/video_player.dart';

import '../widgets/animated_section.dart';
import '../widgets/glass_card.dart';

class HomePage extends StatefulWidget {
  const HomePage({super.key});

  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  late VideoPlayerController _controller;

  @override
  void initState() {
    super.initState();

    _controller =
        VideoPlayerController.network(
            'https://www.w3schools.com/html/mov_bbb.mp4',
          )
          ..initialize().then((_) {
            _controller
              ..setLooping(true)
              ..setVolume(0)
              ..play();
            setState(() {});
          });
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final width = MediaQuery.of(context).size.width;
    final height = MediaQuery.of(context).size.height;

    final isDesktop = width > 1000;
    final isTablet = width > 600 && width <= 1000;

    return Scaffold(
      body: Column(
        children: [
          /// ✅ SCROLLABLE CONTENT
          Expanded(
            child: SingleChildScrollView(
              child: Column(
                children: [
                  /// 🔥 HERO SECTION
                  SizedBox(
                    height: height,
                    child: Stack(
                      children: [
                        /// 🎥 VIDEO
                        if (_controller.value.isInitialized)
                          SizedBox.expand(
                            child: FittedBox(
                              fit: BoxFit.cover,
                              child: SizedBox(
                                width: _controller.value.size.width,
                                height: _controller.value.size.height,
                                child: VideoPlayer(_controller),
                              ),
                            ),
                          )
                        else
                          Container(color: Colors.black),

                        /// 🌑 OVERLAY
                        Container(color: Colors.black.withOpacity(0.6)),

                        /// 💎 CONTENT
                        Center(
                          child: ConstrainedBox(
                            constraints: const BoxConstraints(maxWidth: 1200),
                            child: AnimatedSection(
                              child: Column(
                                mainAxisAlignment: MainAxisAlignment.center,
                                children: [
                                  /// TITLE
                                  Text(
                                    "CobaltDev",
                                    style: TextStyle(
                                      fontSize: isDesktop
                                          ? 60
                                          : isTablet
                                          ? 42
                                          : 32,
                                      fontWeight: FontWeight.bold,
                                    ),
                                  ),

                                  const SizedBox(height: 20),

                                  /// GLASS CARD
                                  GlassCard(
                                    child: Column(
                                      children: [
                                        Text(
                                          "Flutter + Rust Developer",
                                          style: TextStyle(
                                            fontSize: isDesktop ? 24 : 18,
                                          ),
                                        ),
                                        const SizedBox(height: 10),
                                        const Text(
                                          "Building scalable apps, systems & backend infrastructure",
                                          textAlign: TextAlign.center,
                                        ),
                                      ],
                                    ),
                                  ),

                                  const SizedBox(height: 30),

                                  /// CTA
                                  ElevatedButton(
                                    onPressed: () {},
                                    style: ElevatedButton.styleFrom(
                                      padding: const EdgeInsets.symmetric(
                                        horizontal: 30,
                                        vertical: 15,
                                      ),
                                    ),
                                    child: const Text("Get Started"),
                                  ),
                                ],
                              ),
                            ),
                          ),
                        ),
                      ],
                    ),
                  ),

                  /// 🚀 SERVICES
                  AnimatedSection(
                    child: Padding(
                      padding: const EdgeInsets.all(40),
                      child: Wrap(
                        spacing: 20,
                        runSpacing: 20,
                        alignment: WrapAlignment.center,
                        children: [
                          _serviceCard(
                            "📱",
                            "Mobile Apps",
                            "Flutter apps for iOS & Android",
                          ),
                          _serviceCard(
                            "🌐",
                            "Web Apps",
                            "Modern responsive web apps",
                          ),
                          _serviceCard(
                            "🦀",
                            "Rust Backend",
                            "High-performance APIs with Axum",
                          ),
                          _serviceCard(
                            "⚡",
                            "Systems",
                            "Scalable backend infrastructure",
                          ),
                        ],
                      ),
                    ),
                  ),

                  ///Projects
                  AnimatedSection(
                    child: Padding(
                      padding: const EdgeInsets.all(40),
                      child: Column(
                        children: [
                          Text("Projects", style: TextStyle(fontSize: 32)),
                          SizedBox(height: 20),
                          Wrap(
                            spacing: 20,
                            runSpacing: 20,
                            children: [
                              _projectCard(
                                "Auth System",
                                "Flutter + Rust auth system",
                              ),
                              _projectCard(
                                "Portfolio Site",
                                "Animated Flutter web UI",
                              ),
                            ],
                          ),
                        ],
                      ),
                    ),
                  ),

                  ///About me
                  AnimatedSection(
                    child: Padding(
                      padding: const EdgeInsets.all(40),
                      child: Column(
                        children: [
                          Text("About Me", style: TextStyle(fontSize: 32)),
                          SizedBox(height: 20),
                          Text(
                            "I'm a Flutter & Rust developer focused on building scalable apps and backend systems...",
                            textAlign: TextAlign.center,
                          ),
                        ],
                      ),
                    ),
                  ),

                  /// 💼 CTA
                  AnimatedSection(
                    child: Container(
                      padding: EdgeInsets.all(40),
                      child: Column(
                        children: [
                          Text(
                            "Let’s Build Something Serious",
                            style: TextStyle(fontSize: 28),
                          ),
                          SizedBox(height: 20),
                          ElevatedButton(
                            onPressed: () {},
                            child: Text("Contact Me"),
                          ),
                        ],
                      ),
                    ),
                  ),
                  Container(
                    padding: EdgeInsets.all(20),
                    color: Colors.black,
                    child: Column(
                      children: [
                        Text("© 2026 CobaltDev"),
                        SizedBox(height: 10),
                        Text("Built with Flutter & Rust"),
                      ],
                    ),
                  ),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }
}

Widget _serviceCard(String icon, String title, String desc) {
  return SizedBox(
    width: 250,
    child: GlassCard(
      child: Column(
        children: [
          Text(icon, style: TextStyle(fontSize: 40)),
          SizedBox(height: 10),
          Text(title, style: TextStyle(fontSize: 18)),
          SizedBox(height: 10),
          Text(desc, textAlign: TextAlign.center),
        ],
      ),
    ),
  );
}
Widget _projectCard(String title, String desc) {
  return SizedBox(
    width: 280,
    child: GlassCard(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          /// 🖼️ Placeholder image
          Container(
            height: 150,
            width: double.infinity,
            decoration: BoxDecoration(
              color: Colors.grey[800],
              borderRadius: BorderRadius.circular(10),
            ),
            child: const Center(
              child: Text("Preview"),
            ),
          ),

          const SizedBox(height: 10),

          /// 📌 Title
          Text(
            title,
            style: const TextStyle(
              fontSize: 18,
              fontWeight: FontWeight.bold,
            ),
          ),

          const SizedBox(height: 6),

          /// 📝 Description
          Text(desc),

          const SizedBox(height: 10),

          /// 🔗 Optional button
          TextButton(
            onPressed: () {},
            child: const Text("View Project"),
          )
        ],
      ),
    ),
  );
}