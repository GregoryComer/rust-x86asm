use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovntdqa_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQA, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 1381592065, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 42, 60, 77, 1, 108, 89, 82], OperandSize::Dword)
}

#[test]
fn vmovntdqa_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQA, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 1819049728, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 42, 44, 221, 0, 127, 108, 108], OperandSize::Qword)
}

#[test]
fn vmovntdqa_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQA, operand1: Some(Direct(YMM2)), operand2: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 42, 18], OperandSize::Dword)
}

#[test]
fn vmovntdqa_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQA, operand1: Some(Direct(YMM2)), operand2: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 42, 17], OperandSize::Qword)
}

#[test]
fn vmovntdqa_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQA, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 1967228240, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 42, 28, 157, 80, 133, 65, 117], OperandSize::Dword)
}

#[test]
fn vmovntdqa_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQA, operand1: Some(Direct(XMM10)), operand2: Some(IndirectDisplaced(RAX, 1782469213, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 98, 121, 42, 144, 93, 82, 62, 106], OperandSize::Qword)
}

#[test]
fn vmovntdqa_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQA, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledDisplaced(EDX, Two, 216448030, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 42, 12, 85, 30, 188, 230, 12], OperandSize::Dword)
}

#[test]
fn vmovntdqa_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQA, operand1: Some(Direct(YMM14)), operand2: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 98, 125, 42, 51], OperandSize::Qword)
}

#[test]
fn vmovntdqa_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQA, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 1293796419, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 125, 72, 42, 36, 221, 67, 196, 29, 77], OperandSize::Dword)
}

#[test]
fn vmovntdqa_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTDQA, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Eight, 1386660530, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 125, 72, 42, 164, 243, 178, 194, 166, 82], OperandSize::Qword)
}

