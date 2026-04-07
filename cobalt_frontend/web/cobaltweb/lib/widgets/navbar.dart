import 'package:flutter/material.dart';
import 'dart:ui';

class Navbar extends StatelessWidget {
  const Navbar({super.key});

  Widget navItem(BuildContext context, String title, String route) {
    final currentRoute = ModalRoute.of(context)?.settings.name;
    final isActive = currentRoute == route;

    return TextButton(
      onPressed: () => Navigator.pushReplacementNamed(context, route),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          Text(
            title,
            style: TextStyle(
              fontSize: 16,
              color: isActive ? Colors.blue : Colors.white,
              fontWeight:
                  isActive ? FontWeight.bold : FontWeight.normal,
            ),
          ),

          /// 🔥 UNDERLINE
          AnimatedContainer(
            duration: const Duration(milliseconds: 300),
            margin: const EdgeInsets.only(top: 4),
            height: 2,
            width: isActive ? 20 : 0,
            color: Colors.blue,
          )
        ],
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final width = MediaQuery.of(context).size.width;
    final isMobile = width < 800;

    return ClipRRect(
      child: BackdropFilter(
        filter: ImageFilter.blur(sigmaX: 10, sigmaY: 10),
        child: Container(
          padding: EdgeInsets.symmetric(
            horizontal: isMobile ? 20 : 40,
            vertical: 20,
          ),
          decoration: BoxDecoration(
            color: Colors.black.withOpacity(0.5),
            border: const Border(
              bottom: BorderSide(color: Colors.white12),
            ),
          ),

          /// 🔥 MOBILE vs DESKTOP SWITCH
          child: isMobile
              ? _mobileNav(context)
              : _desktopNav(context),
        ),
      ),
    );
  }

  /// 📱 MOBILE NAVBAR
  Widget _mobileNav(BuildContext context) {
    return Row(
      mainAxisAlignment: MainAxisAlignment.spaceBetween,
      children: [
        GestureDetector(
          onTap: () {
            Navigator.pushNamedAndRemoveUntil(
              context,
              '/',
              (route) => false,
            );
          },
          child: const Text(
            "CobaltDev",
            style: TextStyle(
              fontSize: 20,
              fontWeight: FontWeight.bold,
            ),
          ),
        ),

        /// 🔥 MENU BUTTON (FIXED)
        Builder(
          builder: (context) => IconButton(
            icon: const Icon(Icons.menu),
            onPressed: () => Scaffold.of(context).openDrawer(),
          ),
        ),
      ],
    );
  }

  /// 💻 DESKTOP NAVBAR (YOUR ORIGINAL STYLE)
  Widget _desktopNav(BuildContext context) {
    return Row(
      mainAxisAlignment: MainAxisAlignment.spaceBetween,
      children: [
        GestureDetector(
          onTap: () {
            Navigator.pushNamedAndRemoveUntil(
              context,
              '/',
              (route) => false,
            );
          },
          child: const Text(
            "CobaltDev",
            style: TextStyle(
              fontSize: 20,
              fontWeight: FontWeight.bold,
            ),
          ),
        ),

        Row(
          children: [
            navItem(context, "Home", "/"),
            navItem(context, "Services", "/services"),
            navItem(context, "Products", "/products"),
            navItem(context, "Portfolio", "/portfolio"),
            navItem(context, "About", "/about"),
            navItem(context, "Contact", "/contact"),
          ],
        ),
      ],
    );
  }
}