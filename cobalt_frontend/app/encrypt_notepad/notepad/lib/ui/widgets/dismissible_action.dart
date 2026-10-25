import 'package:flutter/material.dart';

class DismissibleController implements DismissAnimationController {
  Future<void> dismiss(DismissDirection direction) async {}
}

class DismissAnimationController {}

class DismissibleAnimationController extends StatefulWidget {
  const DismissibleAnimationController({super.key});

  @override
  State<DismissibleAnimationController> createState() =>
      _DismissibleAnimationControllerState();
}

class _DismissibleAnimationControllerState
    extends State<DismissibleAnimationController>
    with SingleTickerProviderStateMixin
    implements DismissibleController {
  late AnimationController _controller;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      duration: const Duration(milliseconds: 500),
      vsync: this,
    );
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Future<void> dismiss(DismissDirection direction) async {
    if (_controller.isCompleted) return;
    try {
      await _controller.animateTo(
        1.0,
        duration: const Duration(milliseconds: 400),
        curve: Curves.easeOutCubic,
      );
    } on TickerCanceled {
      // print("animation cancelled");
    }
  }

  @override
  Widget build(BuildContext context) {
    return const SizedBox.shrink();
  }
}

// Example of proper Dismissible usage
class DismissibleExample extends StatefulWidget {
  const DismissibleExample({super.key});

  @override
  State<DismissibleExample> createState() => _DismissibleExampleState();
}

class _DismissibleExampleState extends State<DismissibleExample> {
  final List<String> _items = List<String>.generate(10, (index) => "Item ${index + 1}");

  @override
  Widget build(BuildContext context) {
    return ListView.builder(
      itemCount: _items.length,
      itemBuilder: (context, index) {
        final item = _items[index];
        return Dismissible(
          key: Key(item),
          direction: DismissDirection.horizontal,
          
          onDismissed: (direction) {
            setState(() {
              _items.removeAt(index);
            });
            
            ScaffoldMessenger.of(context).showSnackBar(
              SnackBar(content: Text('$item dismissed')),
            );
          },
          
          background: Container(
            color: Colors.red,
            alignment: Alignment.centerLeft,
            padding: const EdgeInsets.only(left: 20),
            child: const Icon(Icons.delete, color: Colors.white),
          ),
          
          secondaryBackground: Container(
            color: Colors.red,
            alignment: Alignment.centerRight,
            padding: const EdgeInsets.only(right: 20),
            child: const Icon(Icons.delete, color: Colors.white),
          ),
          
          confirmDismiss: (direction) async {
            return await showDialog(
              context: context,
              builder: (BuildContext context) {
                return AlertDialog(
                  title: const Text("Confirm"),
                  content: const Text("Are you sure you want to delete this item?"),
                  actions: [
                    TextButton(
                      onPressed: () => Navigator.of(context).pop(false),
                      child: const Text("Cancel"),
                    ),
                    TextButton(
                      onPressed: () => Navigator.of(context).pop(true),
                      child: const Text("Delete"),
                    ),
                  ],
                );
              },
            );
          },
          
          child: ListTile(
            title: Text(item),
          ),
        );
      },
    );
  }
}