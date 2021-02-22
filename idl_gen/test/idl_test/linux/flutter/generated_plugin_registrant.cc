//
//  Generated file. Do not edit.
//

#include "generated_plugin_registrant.h"

#include <idl_test/idl_test_plugin.h>

void fl_register_plugins(FlPluginRegistry* registry) {
  g_autoptr(FlPluginRegistrar) idl_test_registrar =
      fl_plugin_registry_get_registrar_for_plugin(registry, "IdlTestPlugin");
  idl_test_plugin_register_with_registrar(idl_test_registrar);
}
