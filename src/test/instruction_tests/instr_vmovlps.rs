use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovlps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Two, 240563492, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 192, 18, 148, 115, 36, 181, 86, 14], OperandSize::Dword)
}

#[test]
fn vmovlps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(RBX, 1939540621, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 232, 18, 179, 141, 10, 155, 115], OperandSize::Qword)
}

#[test]
fn vmovlps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 1332391308, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 216, 18, 28, 141, 140, 173, 106, 79], OperandSize::Dword)
}

#[test]
fn vmovlps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM28)), operand3: Some(IndirectDisplaced(RDX, 304946967, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 28, 0, 18, 178, 23, 31, 45, 18], OperandSize::Qword)
}

#[test]
fn vmovlps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPS, operand1: Some(IndirectScaledIndexed(ESI, EDI, Four, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 19, 60, 190], OperandSize::Dword)
}

#[test]
fn vmovlps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPS, operand1: Some(IndirectScaledDisplaced(RAX, Two, 1783779934, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 19, 36, 69, 94, 82, 82, 106], OperandSize::Qword)
}

#[test]
fn vmovlps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPS, operand1: Some(IndirectScaledIndexed(ESI, ESI, Four, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 19, 44, 182], OperandSize::Dword)
}

#[test]
fn vmovlps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPS, operand1: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 124, 8, 19, 11], OperandSize::Qword)
}

