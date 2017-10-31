use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovntps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPS, operand1: Some(IndirectScaledDisplaced(EAX, Eight, 1182868890, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 43, 28, 197, 154, 37, 129, 70], OperandSize::Dword)
}

#[test]
fn vmovntps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPS, operand1: Some(IndirectScaledIndexed(RCX, RAX, Two, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 43, 36, 65], OperandSize::Qword)
}

#[test]
fn vmovntps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPS, operand1: Some(IndirectScaledIndexed(EDX, EAX, Four, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 43, 60, 130], OperandSize::Dword)
}

#[test]
fn vmovntps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPS, operand1: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 43, 58], OperandSize::Qword)
}

#[test]
fn vmovntps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPS, operand1: Some(IndirectDisplaced(EBX, 1690601273, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 43, 131, 57, 135, 196, 100], OperandSize::Dword)
}

#[test]
fn vmovntps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPS, operand1: Some(IndirectDisplaced(RBX, 1041067704, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 124, 8, 43, 155, 184, 110, 13, 62], OperandSize::Qword)
}

#[test]
fn vmovntps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPS, operand1: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 43, 59], OperandSize::Dword)
}

#[test]
fn vmovntps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPS, operand1: Some(IndirectScaledDisplaced(RDX, Four, 1976092349, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 124, 40, 43, 52, 149, 189, 198, 200, 117], OperandSize::Qword)
}

#[test]
fn vmovntps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPS, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Four, 1327738943, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 124, 72, 43, 132, 191, 63, 176, 35, 79], OperandSize::Dword)
}

#[test]
fn vmovntps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPS, operand1: Some(IndirectScaledDisplaced(RCX, Two, 1764710895, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 124, 72, 43, 60, 77, 239, 89, 47, 105], OperandSize::Qword)
}

