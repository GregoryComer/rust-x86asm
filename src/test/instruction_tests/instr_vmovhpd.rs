use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovhpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(ESI, ESI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 22, 20, 246], OperandSize::Dword)
}

#[test]
fn vmovhpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Two, 179343676, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 22, 156, 78, 60, 145, 176, 10], OperandSize::Qword)
}

#[test]
fn vmovhpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Two, 774280271, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 22, 180, 89, 79, 148, 38, 46], OperandSize::Dword)
}

#[test]
fn vmovhpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPD, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM22)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Four, 1194565199, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 205, 0, 22, 180, 182, 79, 158, 51, 71], OperandSize::Qword)
}

#[test]
fn vmovhpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPD, operand1: Some(IndirectScaledIndexed(EDI, EAX, Four, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 23, 20, 135], OperandSize::Dword)
}

#[test]
fn vmovhpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPD, operand1: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 23, 47], OperandSize::Qword)
}

#[test]
fn vmovhpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPD, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Two, 354369583, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 23, 172, 95, 47, 64, 31, 21], OperandSize::Dword)
}

#[test]
fn vmovhpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPD, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Two, 593494864, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 253, 8, 23, 188, 86, 80, 3, 96, 35], OperandSize::Qword)
}

