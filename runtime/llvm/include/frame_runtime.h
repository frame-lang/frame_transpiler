#ifndef FRAME_RUNTIME_LLVM_H
#define FRAME_RUNTIME_LLVM_H

#ifdef __cplusplus
extern "C" {
#endif

typedef struct FrameEvent FrameEvent;
typedef struct FrameCompartment FrameCompartment;
typedef struct FrameKernel FrameKernel;

FrameEvent* frame_runtime_event_new(const char* message);
void frame_runtime_event_free(FrameEvent* event);

FrameCompartment* frame_runtime_compartment_new(const char* state);
void frame_runtime_compartment_free(FrameCompartment* compartment);

FrameKernel* frame_runtime_kernel_new(FrameCompartment* compartment);
void frame_runtime_kernel_free(FrameKernel* kernel);
int frame_runtime_kernel_dispatch(FrameKernel* kernel, FrameEvent* event);

#ifdef __cplusplus
}
#endif

#endif /* FRAME_RUNTIME_LLVM_H */
