use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vmovntdqa_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQA, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Two, 1943672852, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 42, 156, 73, 20, 24, 218, 115], OperandSize::Dword)
}

fn vmovntdqa_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQA, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 1673667558, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 42, 44, 141, 230, 35, 194, 99], OperandSize::Qword)
}

fn vmovntdqa_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQA, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 473018871, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 42, 28, 197, 247, 177, 49, 28], OperandSize::Dword)
}

fn vmovntdqa_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQA, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexed(RDI, RAX, Eight, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 42, 36, 199], OperandSize::Qword)
}

fn vmovntdqa_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQA, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(ESI, 1739610860, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 42, 150, 236, 90, 176, 103], OperandSize::Dword)
}

fn vmovntdqa_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQA, operand1: Some(Direct(XMM8)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 1120632768, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 98, 121, 42, 4, 117, 192, 127, 203, 66], OperandSize::Qword)
}

fn vmovntdqa_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQA, operand1: Some(Direct(YMM7)), operand2: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 42, 63], OperandSize::Dword)
}

fn vmovntdqa_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQA, operand1: Some(Direct(YMM0)), operand2: Some(IndirectDisplaced(RDI, 619428473, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 42, 135, 121, 186, 235, 36], OperandSize::Qword)
}

fn vmovntdqa_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQA, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Eight, 1024052425, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 125, 72, 42, 132, 199, 201, 204, 9, 61], OperandSize::Dword)
}

fn vmovntdqa_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQA, operand1: Some(Direct(ZMM21)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Two, 1156493482, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 125, 72, 42, 172, 122, 170, 176, 238, 68], OperandSize::Qword)
}

