#ifndef spirq_h
#define spirq_h

#pragma once

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum Error {
  Success = 0,
  ArgumentNull = -1,
  ArgumentOutOfRange = -2,
  InvalidArgument = -3,
  CorruptedSpirv = -4,
  UnsupportedSpirv = -5,
  InvalidSpecialization = -6,
};
typedef int32_t Error;

typedef uint32_t Bool;

typedef uint32_t SpecId;

typedef struct Specialization {
  SpecId spec_id;
  uintptr_t value_size;
  const void *value;
} Specialization;

typedef struct ReflectConfig {
  uintptr_t spirv_size;
  const uint32_t *spirv;
  Bool reference_all_resources;
  Bool combine_image_samplers;
  Bool generate_unique_names;
  uint32_t specialization_count;
  const struct Specialization *specializations;
} ReflectConfig;

typedef void *Reflection;

typedef void *EntryPoint;

Error create_reflection(const struct ReflectConfig *config, Reflection *reflection);

void destroy_reflection(Reflection *reflection);

Error enumerate_entry_points(const Reflection *reflection,
                             uint32_t *entry_point_count,
                             EntryPoint *entry_points);

#endif /* spirq_h */
