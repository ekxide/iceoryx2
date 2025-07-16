C++
---

.. toctree::
   :maxdepth: 1

   cxx_pub_sub
   cxx_event

All examples for all languages can be found in the table in the
`iceoryx2 examples directory <https://github.com/eclipse-iceoryx/iceoryx2/tree/main/examples>`_.

The publish-subscriber example can be started with 2 terminals.

Start in terminal 1:

.. code-block:: sh

   ./target/ffi/build/examples/cxx/publish_subscribe/example_cxx_publish_subscribe_subscriber

And in terminal 2:

.. code-block:: sh

   ./target/ffi/build/examples/cxx/publish_subscribe/example_cxx_publish_subscribe_publisher

You should observe how the publisher application sends data to the subscriber application.
