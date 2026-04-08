import 'package:flutter/material.dart';
import 'dart:ui';

class Navbar extends StatefulWidget {
  const Navbar({super.key});

  @override
  State<Navbar> createState() => _NavbarState();
}

class _NavbarState extends State<Navbar> {
  String currentRoute = '/';

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    currentRoute = ModalRoute.of(context)?.settings.name ?? '/';
  }

  Widget navItem(String title, String route) {
    final isActive = currentRoute == route;

    return MouseRegion(
      cursor: SystemMouseCursors.click,
      child: GestureDetector(
        onTap: () {
          setState(() => currentRoute = route);
          Navigator.pushReplacementNamed(context, route);
        },
        child: AnimatedContainer(
          duration: const Duration(milliseconds: 250),
          padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 8),
          decoration: BoxDecoration(
            borderRadius: BorderRadius.circular(30),
            color: isActive
                ? const Color(0xFF10B981).withOpacity(0.15)
                : Colors.transparent,
          ),
          child: Text(
            title,
            style: TextStyle(
              fontSize: 16,
              fontWeight: isActive ? FontWeight.w600 : FontWeight.w500,
              color: isActive ? const Color(0xFF10B981) : Colors.white70,
            ),
          ),
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final width = MediaQuery.of(context).size.width;
    final isMobile = width < 900;

    return Padding(
      padding: const EdgeInsets.all(16.0),
      child: ClipRRect(
        borderRadius: BorderRadius.circular(40),
        child: BackdropFilter(
          filter: ImageFilter.blur(sigmaX: 24, sigmaY: 24),
          child: Container(
            padding: EdgeInsets.symmetric(
              horizontal: isMobile ? 24 : 40,
              vertical: 16,
            ),
            decoration: BoxDecoration(
              color: const Color(0xFF0F0F13).withOpacity(0.5),
              borderRadius: BorderRadius.circular(40),
              border: Border.all(
                color: Colors.white.withOpacity(0.1),
                width: 1,
              ),
            ),
            child: Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                // Logo
                GestureDetector(
                  onTap: () {
                    Navigator.pushReplacementNamed(context, '/');
                  },
                  child: Row(
                    children: [
                      Image.asset(
                        'assets/images/cobalt_logo.png',
                        height: 36,
                        errorBuilder: (context, error, stackTrace) =>
                            const SizedBox(),
                      ),
                      const SizedBox(width: 12),
                      const Text(
                        "Cobalt",
                        style: TextStyle(
                          fontSize: 26,
                          fontWeight: FontWeight.bold,
                          color: Color(0xFF10B981),
                        ),
                      ),
                      const Text(
                        "Dev",
                        style: TextStyle(
                          fontSize: 26,
                          fontWeight: FontWeight.bold,
                          color: Colors.white,
                        ),
                      ),
                    ],
                  ),
                ),

                if (!isMobile)
                  Row(
                    children: [
                      navItem("Home", "/"),
                      const SizedBox(width: 12),
                      navItem("Services", "/services"),
                      const SizedBox(width: 12),
                      navItem("Products", "/products"),
                      const SizedBox(width: 12),
                      navItem("Portfolio", "/portfolio"),
                      const SizedBox(width: 12),
                      navItem("About", "/about"),
                      const SizedBox(width: 12),
                      navItem("Contact", "/contact"),
                    ],
                  ),

                // Mobile Menu Button
                if (isMobile)
                  Builder(
                    builder: (context) => IconButton(
                      icon: const Icon(Icons.menu, color: Colors.white),
                      onPressed: () => Scaffold.of(context).openDrawer(),
                    ),
                  ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
