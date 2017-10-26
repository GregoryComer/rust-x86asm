use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovntps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPS, operand1: Some(IndirectScaledIndexed(EDX, EAX, Eight, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 43, 36, 194], OperandSize::Dword)
}

#[test]
fn vmovntps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPS, operand1: Some(IndirectScaledIndexed(RDX, RBX, Four, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 43, 12, 154], OperandSize::Qword)
}

#[test]
fn vmovntps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPS, operand1: Some(IndirectDisplaced(EDX, 1564291633, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 43, 186, 49, 50, 61, 93], OperandSize::Dword)
}

#[test]
fn vmovntps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPS, operand1: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 43, 50], OperandSize::Qword)
}

#[test]
fn vmovntps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPS, operand1: Some(IndirectDisplaced(EBX, 133856106, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 43, 131, 106, 123, 250, 7], OperandSize::Dword)
}

#[test]
fn vmovntps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPS, operand1: Some(IndirectDisplaced(RCX, 734593531, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 120, 43, 137, 251, 1, 201, 43], OperandSize::Qword)
}

#[test]
fn vmovntps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPS, operand1: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 43, 35], OperandSize::Dword)
}

#[test]
fn vmovntps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPS, operand1: Some(IndirectScaledDisplaced(RDI, Four, 648196370, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 124, 43, 28, 189, 18, 177, 162, 38], OperandSize::Qword)
}

#[test]
fn vmovntps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPS, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Eight, 1520205289, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 124, 72, 43, 180, 216, 233, 125, 156, 90], OperandSize::Dword)
}

#[test]
fn vmovntps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPS, operand1: Some(IndirectScaledIndexed(RCX, RAX, Four, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 124, 72, 43, 36, 129], OperandSize::Qword)
}

