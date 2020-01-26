/*
 * Copyright © 1998-2004  David Turner and Werner Lemberg
 * Copyright © 2004,2007,2009,2010  Red Hat, Inc.
 * Copyright © 2011,2012  Google, Inc.
 *
 *  This is part of HarfBuzz, a text shaping library.
 *
 * Permission is hereby granted, without written agreement and without
 * license or royalty fees, to use, copy, modify, and distribute this
 * software and its documentation for any purpose, provided that the
 * above copyright notice and the following two paragraphs appear in
 * all copies of this software.
 *
 * IN NO EVENT SHALL THE COPYRIGHT HOLDER BE LIABLE TO ANY PARTY FOR
 * DIRECT, INDIRECT, SPECIAL, INCIDENTAL, OR CONSEQUENTIAL DAMAGES
 * ARISING OUT OF THE USE OF THIS SOFTWARE AND ITS DOCUMENTATION, EVEN
 * IF THE COPYRIGHT HOLDER HAS BEEN ADVISED OF THE POSSIBILITY OF SUCH
 * DAMAGE.
 *
 * THE COPYRIGHT HOLDER SPECIFICALLY DISCLAIMS ANY WARRANTIES, INCLUDING,
 * BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND
 * FITNESS FOR A PARTICULAR PURPOSE.  THE SOFTWARE PROVIDED HEREUNDER IS
 * ON AN "AS IS" BASIS, AND THE COPYRIGHT HOLDER HAS NO OBLIGATION TO
 * PROVIDE MAINTENANCE, SUPPORT, UPDATES, ENHANCEMENTS, OR MODIFICATIONS.
 *
 * Red Hat Author(s): Owen Taylor, Behdad Esfahbod
 * Google Author(s): Behdad Esfahbod
 */

#pragma once

#include "hb.hh"
#include "hb-unicode.hh"


#ifndef HB_BUFFER_MAX_LEN_FACTOR
#define HB_BUFFER_MAX_LEN_FACTOR 32
#endif
#ifndef HB_BUFFER_MAX_LEN_MIN
#define HB_BUFFER_MAX_LEN_MIN 8192
#endif
#ifndef HB_BUFFER_MAX_LEN_DEFAULT
#define HB_BUFFER_MAX_LEN_DEFAULT 0x3FFFFFFF /* Shaping more than a billion chars? Let us know! */
#endif

#ifndef HB_BUFFER_MAX_OPS_FACTOR
#define HB_BUFFER_MAX_OPS_FACTOR 64
#endif
#ifndef HB_BUFFER_MAX_OPS_MIN
#define HB_BUFFER_MAX_OPS_MIN 1024
#endif
#ifndef HB_BUFFER_MAX_OPS_DEFAULT
#define HB_BUFFER_MAX_OPS_DEFAULT 0x1FFFFFFF /* Shaping more than a billion operations? Let us know! */
#endif

static_assert ((sizeof (hb_glyph_info_t) == 20), "");
static_assert ((sizeof (hb_glyph_info_t) == sizeof (hb_glyph_position_t)), "");

HB_MARK_AS_FLAG_T (hb_buffer_flags_t);
HB_MARK_AS_FLAG_T (hb_buffer_serialize_flags_t);
HB_MARK_AS_FLAG_T (hb_buffer_scratch_flags_t);

/*
 * hb_buffer_t
 */

struct hb_buffer_t
{
  hb_object_header_t header;

  /* Information about how the text in the buffer should be treated */
  hb_buffer_flags_t flags; /* BOT / EOT / etc. */
  hb_buffer_cluster_level_t cluster_level;
  hb_codepoint_t replacement; /* U+FFFD or something else. */
  hb_codepoint_t invisible; /* 0 or something else. */
  hb_buffer_scratch_flags_t scratch_flags; /* Have space-fallback, etc. */
  int max_ops; /* Maximum allowed operations. */

  /* Buffer contents */
  hb_buffer_content_type_t content_type;
  hb_segment_properties_t props; /* Script, language, direction */

  bool have_output; /* Whether we have an output buffer going on */
  bool have_positions; /* Whether we have positions */

  unsigned int idx; /* Cursor into ->info and ->pos arrays */
  unsigned int out_len; /* Length of ->out array if have_output */

  hb_vector_t<hb_glyph_info_t> info_vec;
  hb_vector_t<hb_glyph_position_t> pos_vec;
  hb_glyph_info_t     *info;
  hb_glyph_position_t *pos;
  bool have_separate_output;
  
  unsigned int serial;

  /* Text before / after the main buffer contents.
   * Always in Unicode, and ordered outward.
   * Index 0 is for "pre-context", 1 for "post-context". */
  static constexpr unsigned CONTEXT_LENGTH = 5u;
  hb_codepoint_t context[2][CONTEXT_LENGTH];
  unsigned int context_len[2];
};
DECLARE_NULL_INSTANCE (hb_buffer_t);
