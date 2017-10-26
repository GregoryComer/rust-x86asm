use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movaps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 40, 243], OperandSize::Dword)
}

#[test]
fn movaps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Eight, 37112043, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 40, 132, 255, 235, 72, 54, 2], OperandSize::Dword)
}

#[test]
fn movaps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 40, 207], OperandSize::Qword)
}

#[test]
fn movaps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 622854894, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 40, 52, 213, 238, 2, 32, 37], OperandSize::Qword)
}

#[test]
fn movaps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 40, 240], OperandSize::Dword)
}

#[test]
fn movaps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPS, operand1: Some(IndirectDisplaced(EAX, 1223710542, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 41, 128, 78, 87, 240, 72], OperandSize::Dword)
}

#[test]
fn movaps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 40, 245], OperandSize::Qword)
}

#[test]
fn movaps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPS, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Four, 475067406, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 41, 132, 142, 14, 244, 80, 28], OperandSize::Qword)
}

