use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovlps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 232, 18, 40], OperandSize::Dword)
}

#[test]
fn vmovlps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 1932032345, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 200, 18, 44, 117, 89, 121, 40, 115], OperandSize::Qword)
}

#[test]
fn vmovlps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 216, 18, 4, 129], OperandSize::Dword)
}

#[test]
fn vmovlps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPS, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Eight, 295169030, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 124, 8, 18, 132, 194, 6, 236, 151, 17], OperandSize::Qword)
}

#[test]
fn vmovlps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPS, operand1: Some(IndirectScaledDisplaced(ESI, Two, 853733532, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 19, 12, 117, 156, 240, 226, 50], OperandSize::Dword)
}

#[test]
fn vmovlps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPS, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Four, 999059781, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 19, 180, 137, 69, 113, 140, 59], OperandSize::Qword)
}

#[test]
fn vmovlps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPS, operand1: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 19, 10], OperandSize::Dword)
}

#[test]
fn vmovlps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPS, operand1: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 19, 43], OperandSize::Qword)
}

