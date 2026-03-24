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
    _controller = VideoPlayerController.network(
      'https://www.w3schools.com/html/mov_bbb.mp4',
    )..initialize().then((_) {
        _controller.setLooping(true);
        _controller.setVolume(0.0); // 🔇 MUTE
        _controller.play();
        setState(() {});
      });
  }

  @override
  void dispose() {
    _controller.dispose(); // ✅ IMPORTANT (prevents memory leak)
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return SingleChildScrollView(
      child: Column(
        children: [

          /// 🔥 HERO SECTION (VIDEO BG)
          Stack(
            children: [
              SizedBox(
                height: 600,
                width: double.infinity,
                child: _controller.value.isInitialized
                    ? FittedBox(
                        fit: BoxFit.cover,
                        child: SizedBox(
                          width: _controller.value.size.width,
                          height: _controller.value.size.height,
                          child: VideoPlayer(_controller),
                        ),
                      )
                    : Container(color: Colors.black),
              ),

              /// 🔥 DARK OVERLAY
              Container(
                height: 600,
                color: Colors.black.withOpacity(0.6),
              ),

              /// 🔥 CONTENT
              SizedBox(
                height: 600,
                child: Center(
                  child: AnimatedSection(
                    child: Column(
                      mainAxisAlignment: MainAxisAlignment.center,
                      children: [

                        /// LOGO / TITLE
                        const Text(
                          "CobaltDev",
                          style: TextStyle(
                            fontSize: 50,
                            fontWeight: FontWeight.bold,
                            letterSpacing: 1.2,
                          ),
                        ),

                        const SizedBox(height: 30),

                        /// 💎 GLASS CARD
                        GlassCard(
                          child: Column(
                            children: const [
                              Text(
                                "Flutter + Rust Developer",
                                style: TextStyle(
                                  fontSize: 24,
                                  fontWeight: FontWeight.w500,
                                ),
                              ),
                              SizedBox(height: 10),
                              Text(
                                "Building scalable apps, systems & backend infrastructure",
                                textAlign: TextAlign.center,
                              ),
                            ],
                          ),
                        ),

                        const SizedBox(height: 30),

                        /// 🚀 CTA BUTTON
                        ElevatedButton(
                          onPressed: () {
                            // TODO: Navigate to contact page or scroll
                          },
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

          /// 🚀 SERVICES PREVIEW
          AnimatedSection(
            child: Padding(
              padding: const EdgeInsets.all(40),
              child: Column(
                children: const [
                  Text(
                    "What I Build",
                    style: TextStyle(fontSize: 32),
                  ),
                  SizedBox(height: 20),
                  Text(
                    "Web Apps • Mobile Apps • Backend Systems • Rust Systems • AI Foundations",
                    textAlign: TextAlign.center,
                  ),
                ],
              ),
            ),
          ),

          /// 💼 CTA SECTION
          AnimatedSection(
            child: Padding(
              padding: const EdgeInsets.all(40),
              child: ElevatedButton(
                onPressed: () {},
                child: const Text("Work With Me"),
              ),
            ),
          ),
        ],
      ),
    );
  }
}