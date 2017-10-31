use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movaps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 40, 193], OperandSize::Dword)
}

#[test]
fn movaps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(ECX, 50249435, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 40, 161, 219, 190, 254, 2], OperandSize::Dword)
}

#[test]
fn movaps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 40, 233], OperandSize::Qword)
}

#[test]
fn movaps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 638709544, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 40, 28, 189, 40, 239, 17, 38], OperandSize::Qword)
}

#[test]
fn movaps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 40, 245], OperandSize::Dword)
}

#[test]
fn movaps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPS, operand1: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 41, 25], OperandSize::Dword)
}

#[test]
fn movaps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 40, 211], OperandSize::Qword)
}

#[test]
fn movaps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPS, operand1: Some(IndirectScaledIndexed(RDI, RDX, Two, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 41, 28, 87], OperandSize::Qword)
}

