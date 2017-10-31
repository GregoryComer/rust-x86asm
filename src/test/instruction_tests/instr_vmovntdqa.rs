use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovntdqa_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQA, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Two, 253456781, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 42, 132, 65, 141, 113, 27, 15], OperandSize::Dword)
}

#[test]
fn vmovntdqa_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQA, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(RCX, 848888941, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 42, 185, 109, 4, 153, 50], OperandSize::Qword)
}

#[test]
fn vmovntdqa_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQA, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 708038520, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 42, 12, 221, 120, 207, 51, 42], OperandSize::Dword)
}

#[test]
fn vmovntdqa_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQA, operand1: Some(Direct(YMM0)), operand2: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 42, 0], OperandSize::Qword)
}

#[test]
fn vmovntdqa_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQA, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Eight, 1978612140, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 42, 180, 214, 172, 57, 239, 117], OperandSize::Dword)
}

#[test]
fn vmovntdqa_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQA, operand1: Some(Direct(XMM28)), operand2: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 125, 8, 42, 35], OperandSize::Qword)
}

#[test]
fn vmovntdqa_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQA, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexed(EAX, EAX, Eight, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 42, 12, 192], OperandSize::Dword)
}

#[test]
fn vmovntdqa_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQA, operand1: Some(Direct(YMM18)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 924300643, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 125, 40, 42, 20, 197, 99, 181, 23, 55], OperandSize::Qword)
}

#[test]
fn vmovntdqa_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQA, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledIndexed(ESI, EDI, Four, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 125, 72, 42, 4, 190], OperandSize::Dword)
}

#[test]
fn vmovntdqa_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQA, operand1: Some(Direct(ZMM23)), operand2: Some(IndirectScaledIndexed(RDI, RSI, Two, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 125, 72, 42, 60, 119], OperandSize::Qword)
}

