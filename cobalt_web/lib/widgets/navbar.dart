import 'package:flutter/material.dart';
import 'dart:ui';

class Navbar extends StatelessWidget {
  const Navbar({super.key});

  Widget navItem(BuildContext context, String title, String route) {
    final currentRoute = ModalRoute.of(context)?.settings.name;
  
    final isActive = currentRoute == route;
  
    return TextButton(
      onPressed: () => Navigator.pushNamed(context, route),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          Text(
            title,
            style: TextStyle(
              fontSize: 16,
              color: isActive ? Colors.blue : Colors.white,
              fontWeight: isActive ? FontWeight.bold : FontWeight.normal,
            ),
          ),
  
          /// 🔥 UNDERLINE INDICATOR
          AnimatedContainer(
            duration: Duration(milliseconds: 300),
            margin: EdgeInsets.only(top: 4),
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
    return ClipRRect(
      child: BackdropFilter(
        filter: ImageFilter.blur(sigmaX: 10, sigmaY: 10),
        child: Container(
          padding: const EdgeInsets.symmetric(horizontal: 40, vertical: 20),
          decoration: BoxDecoration(
            color: Colors.black.withOpacity(0.5),
            border: Border(bottom: BorderSide(color: Colors.white12)),
          ),
          child: Row(
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
                  style: TextStyle(fontSize: 20, fontWeight: FontWeight.bold),
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
          ),
        ),
      ),
    );
  }
}
