#include <linux/bpf.h>
#include <linux/ptrace.h>
#include <linux/sched.h>
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>
#include <linux/types.h>  // 추가된 헤더

#define TASK_COMM_LEN 16  // 커맨드 이름의 길이 정의

// 구조체를 올바르게 정의
struct trace_event_raw_sched_switch {
    // u64 __data_loc_prev_comm;
    // u64 __data_loc_next_comm;
    // 필요한 필드 추가
    char prev_comm[TASK_COMM_LEN];
    char next_comm[TASK_COMM_LEN];
};

SEC("tracepoint/sched/sched_switch")
int trace_sched_switch(struct trace_event_raw_sched_switch *ctx) {
    bpf_printk("Context switch: %s -> %s\n", ctx->prev_comm, ctx->next_comm);
    return 0;
}

char LICENSE[] SEC("license") = "GPL";

